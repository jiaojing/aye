//! Adapter 只通过业务模块的公开契约接线，拥有跨上下文与传输错误映射。

use std::io;

use crate::{payments, sales};

/// 销售事实映射成支付请求；双方各自验证输入，不共享内部订单模型。
pub fn to_charge(fact: &sales::Confirmation) -> payments::Result<payments::ChargeRequest> {
    payments::ChargeRequest::new(
        &format!("sales/order/{}", fact.order_id()),
        fact.total_minor(),
    )
}

pub enum Reply {
    Accepted,
    Declined,
}

/// 发送阶段由底层契约提供，不能仅根据 io::ErrorKind 猜测执行结果。
#[derive(Debug, thiserror::Error)]
pub enum TransportError {
    #[error("request was not sent")]
    NotSent(#[source] io::Error),
    #[error("request was sent without a conclusive response")]
    Unconfirmed(#[source] io::Error),
}

pub type Result<T> = std::result::Result<T, TransportError>;

/// 确定性的示例解释器。只提供一次传输结果，不执行真实 I/O 或幂等重试。
pub struct DemoGateway {
    reply: Option<Result<Reply>>,
    attempts: usize,
    last_request: Option<(String, u64)>,
}

impl DemoGateway {
    pub fn new(reply: Result<Reply>) -> Self {
        Self {
            reply: Some(reply),
            attempts: 0,
            last_request: None,
        }
    }

    pub fn attempts(&self) -> usize {
        self.attempts
    }

    pub fn last_request(&self) -> Option<(&str, u64)> {
        self.last_request
            .as_ref()
            .map(|(reference, amount)| (reference.as_str(), *amount))
    }
}

impl payments::PaymentGateway for DemoGateway {
    fn charge(&mut self, request: &payments::ChargeRequest) -> payments::Result<()> {
        self.attempts += 1;
        self.last_request = Some((request.reference().to_owned(), request.amount_minor()));
        match self
            .reply
            .take()
            .expect("demo supplies one transport reply")
        {
            Ok(Reply::Accepted) => Ok(()),
            Ok(Reply::Declined) => Err(payments::Error::Declined),
            Err(source @ TransportError::NotSent(_)) => Err(payments::Error::Unavailable {
                source: Box::new(source),
            }),
            Err(source @ TransportError::Unconfirmed(_)) => Err(payments::Error::OutcomeUnknown {
                source: Box::new(source),
            }),
        }
    }
}
