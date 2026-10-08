//! 目录模块的公开门面：子模块按实现职责拆分，共用一个支付错误契约。
//! 外部依赖 payments::ChargeRequest / payments::Error / payments::Result，不依赖内部文件路径。

mod error;
mod gateway;
mod request;

pub use error::{Error, Result};
pub use gateway::PaymentGateway;
pub use request::ChargeRequest;
