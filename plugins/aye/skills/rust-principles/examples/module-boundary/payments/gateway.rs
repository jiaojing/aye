use super::{ChargeRequest, Result};

/// Consumer 需要的外部能力；实现拥有网络 I/O，支付核心只依赖业务契约。
pub trait PaymentGateway {
    fn charge(&mut self, request: &ChargeRequest) -> Result<()>;
}
