//! 通过 store 的公开契约验证 DSL、解释器替换和返回类型约束。

use super::store::{
    validate_plan, Capabilities, Client, Error, Get, Interpreter, MemoryStore, Operation, Put,
    ReadOnlyStore, Reply,
};

fn read_with_same_consumer(
    interpreter: &mut dyn Interpreter,
) -> super::store::Result<Option<String>> {
    Client::new(interpreter).get("key")
}

#[test]
fn constructing_and_inspecting_an_operation_does_not_execute_it() {
    let operation: Operation = Put {
        key: "key".into(),
        value: "value".into(),
    }
    .into();
    assert_eq!(operation.name(), "put");
    assert!(operation.is_write());

    let mut memory = MemoryStore::default();
    assert_eq!(Client::new(&mut memory).get("key").unwrap(), None);
    assert_eq!(memory.exec(operation).unwrap(), Reply::Stored);
    assert_eq!(
        read_with_same_consumer(&mut memory).unwrap(),
        Some("value".into())
    );
}

#[test]
fn the_same_typed_consumer_works_with_different_interpreters() {
    let mut memory = MemoryStore::default();
    Client::new(&mut memory).put("key", "value").unwrap();
    assert_eq!(
        read_with_same_consumer(&mut memory).unwrap(),
        Some("value".into())
    );

    let mut read_only = ReadOnlyStore::new(memory);
    assert_eq!(
        read_with_same_consumer(&mut read_only).unwrap(),
        Some("value".into())
    );
}

#[test]
fn raw_execution_cannot_bypass_read_only_capabilities() {
    let mut memory = MemoryStore::default();
    Client::new(&mut memory).put("key", "original").unwrap();
    let mut read_only = ReadOnlyStore::new(memory);
    let error = read_only
        .exec(
            Put {
                key: "key".into(),
                value: "replacement".into(),
            }
            .into(),
        )
        .unwrap_err();
    assert!(matches!(
        error,
        Error::UnsupportedOperation { operation: "put" }
    ));
    assert_eq!(
        read_with_same_consumer(&mut read_only).unwrap(),
        Some("original".into())
    );
}

#[test]
fn a_mixed_plan_can_be_rejected_before_execution() {
    let plan = vec![
        Operation::from(Get { key: "key".into() }),
        Operation::from(Put {
            key: "key".into(),
            value: "value".into(),
        }),
    ];
    assert!(matches!(
        validate_plan(&plan, Capabilities::READ_ONLY),
        Err(Error::UnsupportedOperation { operation: "put" })
    ));
    assert!(validate_plan(&plan, Capabilities::READ_WRITE).is_ok());
}

struct WrongReply {
    writes: bool,
    calls: usize,
}

impl Interpreter for WrongReply {
    fn capabilities(&self) -> Capabilities {
        if self.writes {
            Capabilities::READ_WRITE
        } else {
            Capabilities::READ_ONLY
        }
    }

    fn exec(&mut self, operation: Operation) -> super::store::Result<Reply> {
        self.calls += 1;
        Ok(match operation {
            Operation::Get(_) => Reply::Stored,
            Operation::Put(_) => Reply::Value(None),
        })
    }
}

#[test]
fn typed_facade_reports_mismatched_replies_without_retrying() {
    let mut interpreter = WrongReply {
        writes: true,
        calls: 0,
    };
    let mut client = Client::new(&mut interpreter);
    assert!(matches!(
        client.get("key"),
        Err(Error::InvalidResponse {
            operation: "get",
            expected: "value",
            received: "stored",
        })
    ));
    assert!(matches!(
        client.put("key", "value"),
        Err(Error::InvalidResponse {
            operation: "put",
            expected: "stored",
            received: "value",
        })
    ));
    assert_eq!(interpreter.calls, 2);
}

#[test]
fn typed_facade_rejects_unsupported_operations_without_calling_the_interpreter() {
    let mut interpreter = WrongReply {
        writes: false,
        calls: 0,
    };
    assert!(matches!(
        Client::new(&mut interpreter).put("key", "value"),
        Err(Error::UnsupportedOperation { operation: "put" })
    ));
    assert_eq!(interpreter.calls, 0);
}
