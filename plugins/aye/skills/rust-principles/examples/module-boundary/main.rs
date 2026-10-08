//! CLI 作为独立 crate consumer，只导入 lib.rs 与 payments/mod.rs 的公开门面。

use anyhow::Context;
use aye_rust_principles_examples::{adapter, payments, sales};

#[derive(Debug, PartialEq, Eq)]
enum CommandOutcome {
    Charged,
    Declined,
}

/// 编排调用公开业务行为，先处理支付语义，再为需要报告的失败添加上下文。
fn execute(
    gateway: &mut impl payments::PaymentGateway,
    order_id: u64,
    units: u32,
    unit_price_minor: u64,
) -> anyhow::Result<CommandOutcome> {
    let id = sales::OrderId::try_from(order_id).context("parse order ID")?;
    let draft =
        sales::Order::draft(id, units).with_context(|| format!("create draft order {order_id}"))?;
    let confirmed = sales::confirm(&draft, unit_price_minor)
        .with_context(|| format!("confirm order {order_id}"))?;
    let fact = confirmed
        .confirmation()
        .expect("successful confirm returns a confirmed order");
    let request = adapter::to_charge(&fact)
        .with_context(|| format!("map confirmed order {order_id} to payment"))?;

    match gateway.charge(&request) {
        Ok(()) => Ok(CommandOutcome::Charged),
        Err(payments::Error::Declined) => Ok(CommandOutcome::Declined),
        Err(error @ payments::Error::OutcomeUnknown { .. }) => Err(error).with_context(|| {
            format!(
                "charge {}: check payment status before deciding to retry",
                request.reference()
            )
        }),
        Err(error) => Err(error).with_context(|| format!("charge {}", request.reference())),
    }
}

fn main() -> anyhow::Result<()> {
    use adapter::{DemoGateway, Reply, TransportError};
    use std::io;

    let mut gateway = DemoGateway::new(Ok(Reply::Accepted));
    println!("{:?}", execute(&mut gateway, 42, 2, 150)?);

    let mut gateway = DemoGateway::new(Ok(Reply::Declined));
    println!("{:?}", execute(&mut gateway, 43, 2, 150)?);

    // 同一底层错误类型可以有不同业务含义，完整来源链只在 CLI 报告一次。
    for failure in [
        TransportError::NotSent(io::Error::new(io::ErrorKind::TimedOut, "connect timed out")),
        TransportError::Unconfirmed(io::Error::new(
            io::ErrorKind::TimedOut,
            "response timed out",
        )),
    ] {
        let mut gateway = DemoGateway::new(Err(failure));
        if let Err(report) = execute(&mut gateway, 44, 2, 150) {
            eprintln!("{report:#}");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use adapter::{DemoGateway, Reply, TransportError};
    use std::io;

    #[test]
    fn invalid_order_never_calls_payment_gateway() {
        let mut gateway = DemoGateway::new(Ok(Reply::Accepted));
        let report = execute(&mut gateway, 42, 0, 150).unwrap_err();
        assert!(matches!(
            report.downcast_ref::<sales::Error>(),
            Some(sales::Error::InvalidUnits)
        ));
        assert_eq!(gateway.attempts(), 0);
    }

    #[test]
    fn decline_is_handled_as_a_business_outcome() {
        let mut gateway = DemoGateway::new(Ok(Reply::Declined));
        assert_eq!(
            execute(&mut gateway, 42, 2, 150).unwrap(),
            CommandOutcome::Declined
        );
        assert_eq!(gateway.attempts(), 1);
    }

    #[test]
    fn anyhow_context_preserves_module_error_and_original_cause() {
        let mut gateway = DemoGateway::new(Err(TransportError::NotSent(io::Error::new(
            io::ErrorKind::TimedOut,
            "connect timed out",
        ))));
        let report = execute(&mut gateway, 42, 2, 150).unwrap_err();
        assert!(matches!(
            report.downcast_ref::<payments::Error>(),
            Some(payments::Error::Unavailable { .. })
        ));
        let cause = report
            .chain()
            .find_map(|error| error.downcast_ref::<io::Error>())
            .expect("adapter preserves the I/O cause");
        assert_eq!(cause.kind(), io::ErrorKind::TimedOut);
        assert!(report.to_string().contains("sales/order/42"));
    }

    #[test]
    fn unconfirmed_payment_requires_status_check_without_blind_retry() {
        let mut gateway = DemoGateway::new(Err(TransportError::Unconfirmed(io::Error::new(
            io::ErrorKind::TimedOut,
            "response timed out",
        ))));
        let report = execute(&mut gateway, 42, 2, 150).unwrap_err();
        assert!(matches!(
            report.downcast_ref::<payments::Error>(),
            Some(payments::Error::OutcomeUnknown { .. })
        ));
        assert!(report.to_string().contains("check payment status"));
        assert_eq!(gateway.attempts(), 1);
    }
}
