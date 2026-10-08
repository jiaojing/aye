mod store;

#[cfg(test)]
mod tests;

use store::{validate_plan, Client, Get, Interpreter, MemoryStore, Operation, Put, ReadOnlyStore};

fn main() -> store::Result<()> {
    let plan: Vec<Operation> = vec![
        Put {
            key: "order/42/status".into(),
            value: "confirmed".into(),
        }
        .into(),
        Get {
            key: "order/42/status".into(),
        }
        .into(),
    ];

    let mut memory = MemoryStore::default();
    validate_plan(&plan, memory.capabilities())?;
    for operation in plan {
        println!("{} (write={})", operation.name(), operation.is_write());
        println!("  {:?}", memory.exec(operation)?);
    }

    let mut read_only = ReadOnlyStore::new(memory);
    // 同一个 consumer 可以通过 trait object 切换解释器。
    let mut client = Client::new(&mut read_only as &mut dyn Interpreter);
    println!("typed get: {:?}", client.get("order/42/status")?);
    if let Err(error) = client.put("order/42/status", "cancelled") {
        println!("typed put: {error}");
    }
    Ok(())
}
