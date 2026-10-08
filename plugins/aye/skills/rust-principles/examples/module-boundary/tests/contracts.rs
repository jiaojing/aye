//! 与 CLI 一样，外部 consumer 只使用公开模块门面。

use aye_rust_principles_examples::{adapter, payments, sales};
use payments::PaymentGateway;
use std::io;

fn draft() -> sales::Order {
    sales::Order::draft(42.try_into().unwrap(), 2).unwrap()
}

#[test]
fn sales_construction_rejects_invalid_values() {
    assert_eq!(sales::OrderId::try_from(0), Err(sales::Error::InvalidId));
    assert_eq!(
        sales::Order::draft(42.try_into().unwrap(), 0),
        Err(sales::Error::InvalidUnits)
    );
}

#[test]
fn confirmation_preserves_input_and_rejects_invalid_transitions() {
    let original = draft();
    assert_eq!(
        sales::confirm(&original, 0),
        Err(sales::Error::InvalidPrice)
    );
    assert_eq!(
        sales::confirm(&original, u64::MAX),
        Err(sales::Error::TotalOverflow)
    );
    let confirmed = sales::confirm(&original, 150).unwrap();
    assert!(original.confirmation().is_none());
    assert_eq!(
        sales::confirm(&confirmed, 150),
        Err(sales::Error::AlreadyConfirmed)
    );
    assert_eq!(confirmed.confirmation().unwrap().total_minor(), 300);
}

#[test]
fn payment_construction_uses_the_same_public_error_contract_as_calls() {
    assert!(matches!(
        payments::ChargeRequest::new(" ", 300),
        Err(payments::Error::InvalidReference)
    ));
    assert!(matches!(
        payments::ChargeRequest::new("sales/order/42", 0),
        Err(payments::Error::InvalidAmount)
    ));
}

#[test]
fn cross_context_mapping_preserves_reference_and_amount() {
    let fact = sales::confirm(&draft(), 150)
        .unwrap()
        .confirmation()
        .unwrap();
    let request = adapter::to_charge(&fact).unwrap();
    let mut gateway = adapter::DemoGateway::new(Ok(adapter::Reply::Accepted));
    gateway.charge(&request).unwrap();
    assert_eq!(gateway.last_request(), Some(("sales/order/42", 300)));
    assert_eq!(gateway.attempts(), 1);
}

#[test]
fn gateway_exposes_rejection_as_a_payment_error() {
    let request = payments::ChargeRequest::new("sales/order/42", 300).unwrap();
    let mut gateway = adapter::DemoGateway::new(Ok(adapter::Reply::Declined));
    assert!(matches!(
        gateway.charge(&request),
        Err(payments::Error::Declined)
    ));
    assert_eq!(gateway.attempts(), 1);
}

#[test]
fn same_io_error_kind_can_map_to_different_business_failures() {
    let request = payments::ChargeRequest::new("sales/order/42", 300).unwrap();
    let mut not_sent = adapter::DemoGateway::new(Err(adapter::TransportError::NotSent(
        io::Error::new(io::ErrorKind::TimedOut, "connect timed out"),
    )));
    let mut unconfirmed = adapter::DemoGateway::new(Err(adapter::TransportError::Unconfirmed(
        io::Error::new(io::ErrorKind::TimedOut, "response timed out"),
    )));
    assert!(matches!(
        not_sent.charge(&request),
        Err(payments::Error::Unavailable { .. })
    ));
    assert!(matches!(
        unconfirmed.charge(&request),
        Err(payments::Error::OutcomeUnknown { .. })
    ));
    assert_eq!(not_sent.attempts(), 1);
    assert_eq!(unconfirmed.attempts(), 1);
}
