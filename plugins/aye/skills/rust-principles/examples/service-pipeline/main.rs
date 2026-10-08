mod pipeline;

#[cfg(test)]
mod tests;

use std::num::NonZeroU32;

use pipeline::{
    run_batch, Audit, Batch, Event, ExecutionLayer, Operation, RiskLayer, RiskPolicy,
    SimulatedExecution,
};
use tower_layer::Layer;

#[tokio::main(flavor = "current_thread")]
async fn main() -> pipeline::Result<()> {
    let audit = Audit::default();
    let execution = SimulatedExecution {
        audit: audit.clone(),
        lose_response: false,
    };
    let pipeline = ExecutionLayer {
        audit: audit.clone(),
        reject_commit: false,
    }
    .layer(execution);
    let mut service = RiskLayer(RiskPolicy {
        max_batch_submit_quantity: 2,
    })
    .layer(pipeline);
    let quantity = NonZeroU32::new(1).expect("literal one is nonzero");
    let batch = Batch(vec![
        Operation::Submit {
            order_id: 1,
            quantity,
        },
        Operation::Submit {
            order_id: 2,
            quantity,
        },
        Operation::Submit {
            order_id: 3,
            quantity,
        },
        Operation::Cancel { order_id: 1 },
    ]);
    let outcome = run_batch(&mut service, batch).await?;
    println!(
        "commit={}, completed={}, denied={}",
        outcome.commit_id,
        outcome.completed.len(),
        outcome.denied.len()
    );
    for event in audit.events() {
        match event {
            Event::Committed(id) => println!("committed {id}"),
            Event::Executed { operation, .. } => println!("executed {operation:?}"),
        }
    }
    println!("retained records={}", audit.records().len());
    Ok(())
}
