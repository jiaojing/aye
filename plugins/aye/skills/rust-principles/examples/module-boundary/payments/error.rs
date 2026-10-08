use std::error::Error as StdError;

pub type Result<T> = std::result::Result<T, Error>;

/// 构造、校验和支付调用共用这个业务 Error；底层 SDK 类型不出现在契约中。
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("payment reference is empty")]
    InvalidReference,
    #[error("payment amount must be positive")]
    InvalidAmount,
    #[error("payment declined")]
    Declined,
    #[error("payment gateway unavailable")]
    Unavailable {
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
    #[error("payment outcome is unknown")]
    OutcomeUnknown {
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },
}
