//! 一个存储边界的操作协议；构造和检查 Operation 不执行后端调用。

use std::collections::BTreeMap;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("operation {operation} is unsupported")]
    UnsupportedOperation { operation: &'static str },
    #[error("operation {operation} expected {expected}, received {received}")]
    InvalidResponse {
        operation: &'static str,
        expected: &'static str,
        received: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Get {
    pub key: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Put {
    pub key: String,
    pub value: String,
}

/// 闭合的操作集合；payload 是数据，不捕获 client、锁或执行闭包。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Get(Get),
    Put(Put),
}

impl Operation {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Get(_) => "get",
            Self::Put(_) => "put",
        }
    }

    pub fn is_write(&self) -> bool {
        matches!(self, Self::Put(_))
    }
}

impl From<Get> for Operation {
    fn from(value: Get) -> Self {
        Self::Get(value)
    }
}

impl From<Put> for Operation {
    fn from(value: Put) -> Self {
        Self::Put(value)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    Value(Option<String>),
    Stored,
}

impl Reply {
    fn name(&self) -> &'static str {
        match self {
            Self::Value(_) => "value",
            Self::Stored => "stored",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Capabilities {
    writes: bool,
}

impl Capabilities {
    pub const READ_ONLY: Self = Self { writes: false };
    pub const READ_WRITE: Self = Self { writes: true };

    pub fn supports(self, operation: &Operation) -> bool {
        !operation.is_write() || self.writes
    }
}

/// 所有实现均支持 Get；Put 替换已有值，并只在执行完成后返回 Stored。
/// 不支持的操作必须在执行前拒绝。缺值是 Value(None)，并非调用失败。
pub trait Interpreter {
    fn capabilities(&self) -> Capabilities;
    fn exec(&mut self, operation: Operation) -> Result<Reply>;
}

/// 纯能力预检；不保证之后的执行成功、能力不变或整份计划原子提交。
pub fn validate_plan(operations: &[Operation], capabilities: Capabilities) -> Result<()> {
    for operation in operations {
        if !capabilities.supports(operation) {
            return Err(Error::UnsupportedOperation {
                operation: operation.name(),
            });
        }
    }
    Ok(())
}

/// Service 持有状态并解释操作；空存储是有效的 Default。
#[derive(Debug, Default)]
pub struct MemoryStore {
    values: BTreeMap<String, String>,
}

impl Interpreter for MemoryStore {
    fn capabilities(&self) -> Capabilities {
        Capabilities::READ_WRITE
    }

    fn exec(&mut self, operation: Operation) -> Result<Reply> {
        match operation {
            Operation::Get(Get { key }) => Ok(Reply::Value(self.values.get(&key).cloned())),
            Operation::Put(Put { key, value }) => {
                self.values.insert(key, value);
                Ok(Reply::Stored)
            }
        }
    }
}

/// 一个能力受限的解释器；直接调用 exec 也无法绕过只读约束。
pub struct ReadOnlyStore<I> {
    inner: I,
}

impl<I> ReadOnlyStore<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I: Interpreter> Interpreter for ReadOnlyStore<I> {
    fn capabilities(&self) -> Capabilities {
        Capabilities::READ_ONLY
    }

    fn exec(&mut self, operation: Operation) -> Result<Reply> {
        if !self.capabilities().supports(&operation) {
            return Err(Error::UnsupportedOperation {
                operation: operation.name(),
            });
        }
        self.inner.exec(operation)
    }
}

/// 类型化 consumer 门面：Get 和 Put 暴露不同返回类型。
/// 门面检查协议响应；检查失败不保证底层操作未执行，不自动重试。
pub struct Client<'a, I: Interpreter + ?Sized> {
    interpreter: &'a mut I,
}

impl<'a, I: Interpreter + ?Sized> Client<'a, I> {
    pub fn new(interpreter: &'a mut I) -> Self {
        Self { interpreter }
    }

    pub fn get(&mut self, key: impl Into<String>) -> Result<Option<String>> {
        match self.send(Get { key: key.into() }.into())? {
            Reply::Value(value) => Ok(value),
            reply => Err(Error::InvalidResponse {
                operation: "get",
                expected: "value",
                received: reply.name(),
            }),
        }
    }

    pub fn put(&mut self, key: impl Into<String>, value: impl Into<String>) -> Result<()> {
        let operation = Put {
            key: key.into(),
            value: value.into(),
        };
        match self.send(operation.into())? {
            Reply::Stored => Ok(()),
            reply => Err(Error::InvalidResponse {
                operation: "put",
                expected: "stored",
                received: reply.name(),
            }),
        }
    }

    fn send(&mut self, operation: Operation) -> Result<Reply> {
        if !self.interpreter.capabilities().supports(&operation) {
            return Err(Error::UnsupportedOperation {
                operation: operation.name(),
            });
        }
        self.interpreter.exec(operation)
    }
}
