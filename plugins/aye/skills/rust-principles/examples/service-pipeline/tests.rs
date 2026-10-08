use std::future::{ready, Ready};
use std::num::NonZeroU32;
use std::task::{Context, Poll, Waker};

use tower_layer::Layer;
use tower_service::Service;

use super::pipeline::{
    run_batch, Audit, Batch, CommittedBatch, Error, Event, ExecutionLayer, Operation, Outcome,
    RiskLayer, RiskPolicy, SimulatedExecution,
};

fn submit(order_id: u64, quantity: u32) -> Operation {
    Operation::Submit {
        order_id,
        quantity: NonZeroU32::new(quantity).unwrap(),
    }
}

fn assemble(
    audit: &Audit,
    limit: u64,
    reject_commit: bool,
    lose_response: bool,
) -> impl Service<Batch, Response = Outcome, Error = Error> {
    RiskLayer(RiskPolicy {
        max_batch_submit_quantity: limit,
    })
    .layer(
        ExecutionLayer {
            audit: audit.clone(),
            reject_commit,
        }
        .layer(SimulatedExecution {
            audit: audit.clone(),
            lose_response,
        }),
    )
}

#[tokio::test]
async fn batch_admission_counts_earlier_allowed_orders_and_preserves_operation_order() {
    let audit = Audit::default();
    let mut service = assemble(&audit, 2, false, false);
    let outcome = run_batch(
        &mut service,
        Batch(vec![submit(1, 1), submit(2, 1), submit(3, 1)]),
    )
    .await
    .unwrap();
    assert_eq!(outcome.completed, vec![submit(1, 1), submit(2, 1)]);
    assert_eq!(outcome.denied.len(), 1);
    assert_eq!(outcome.denied[0].order_id, 3);
    let records = audit.records();
    assert_eq!(records[0].allowed, outcome.completed);
    assert_eq!(records[0].denied, outcome.denied);
    assert_eq!(
        audit.events(),
        vec![
            Event::Committed(0),
            Event::Executed {
                commit_id: 0,
                operation: submit(1, 1)
            },
            Event::Executed {
                commit_id: 0,
                operation: submit(2, 1)
            },
        ]
    );
}

#[tokio::test]
async fn denial_is_committed_as_a_business_result_without_external_execution() {
    let audit = Audit::default();
    let mut service = assemble(&audit, 1, false, false);
    let outcome = run_batch(&mut service, Batch(vec![submit(1, 2)]))
        .await
        .unwrap();
    assert!(outcome.completed.is_empty());
    assert_eq!(outcome.denied[0].order_id, 1);
    assert_eq!(audit.records()[0].denied, outcome.denied);
    assert_eq!(audit.events(), vec![Event::Committed(0)]);
}

#[tokio::test]
async fn cancel_does_not_release_the_submit_budget_of_the_same_batch() {
    let audit = Audit::default();
    let mut service = assemble(&audit, 1, false, false);
    let cancel = Operation::Cancel { order_id: 1 };
    let outcome = run_batch(
        &mut service,
        Batch(vec![submit(1, 1), cancel.clone(), submit(2, 1)]),
    )
    .await
    .unwrap();
    assert_eq!(outcome.completed, vec![submit(1, 1), cancel]);
    assert_eq!(outcome.denied[0].order_id, 2);
}

struct MustNotExecute;

impl Service<CommittedBatch> for MustNotExecute {
    type Response = Outcome;
    type Error = Error;
    type Future = Ready<super::pipeline::Result<Outcome>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<super::pipeline::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _request: CommittedBatch) -> Self::Future {
        panic!("failed commit must not dispatch a request")
    }
}

#[tokio::test]
async fn failed_commit_never_calls_the_execution_service() {
    let audit = Audit::default();
    let mut service = RiskLayer(RiskPolicy {
        max_batch_submit_quantity: 1,
    })
    .layer(
        ExecutionLayer {
            audit: audit.clone(),
            reject_commit: true,
        }
        .layer(MustNotExecute),
    );
    assert!(matches!(
        run_batch(&mut service, Batch(vec![submit(1, 1)])).await,
        Err(Error::CommitRejected)
    ));
    assert!(audit.records().is_empty());
    assert!(audit.events().is_empty());
}

#[tokio::test]
async fn uncertain_execution_keeps_the_committed_intent_without_retrying() {
    let audit = Audit::default();
    let mut service = assemble(&audit, 2, false, true);
    assert!(matches!(
        run_batch(&mut service, Batch(vec![submit(1, 1), submit(2, 1)])).await,
        Err(Error::OutcomeUnknown { commit_id: 0 })
    ));
    assert_eq!(audit.records()[0].allowed, vec![submit(1, 1), submit(2, 1)]);
    assert_eq!(
        audit.events(),
        vec![
            Event::Committed(0),
            Event::Executed {
                commit_id: 0,
                operation: submit(1, 1)
            },
        ]
    );
}

struct BecomesReady {
    ready: bool,
}

impl Service<CommittedBatch> for BecomesReady {
    type Response = Outcome;
    type Error = Error;
    type Future = Ready<super::pipeline::Result<Outcome>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<super::pipeline::Result<()>> {
        if self.ready {
            Poll::Ready(Ok(()))
        } else {
            self.ready = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    fn call(&mut self, _request: CommittedBatch) -> Self::Future {
        assert!(self.ready, "call must use the service that became ready");
        ready(Ok(Outcome {
            commit_id: 0,
            completed: Vec::new(),
            denied: Vec::new(),
        }))
    }
}

#[test]
fn layers_forward_pending_readiness_before_any_commit() {
    let audit = Audit::default();
    let mut service = RiskLayer(RiskPolicy {
        max_batch_submit_quantity: 1,
    })
    .layer(
        ExecutionLayer {
            audit: audit.clone(),
            reject_commit: false,
        }
        .layer(BecomesReady { ready: false }),
    );
    let mut cx = Context::from_waker(Waker::noop());
    assert!(service.poll_ready(&mut cx).is_pending());
    assert!(audit.events().is_empty());
    assert!(matches!(service.poll_ready(&mut cx), Poll::Ready(Ok(()))));
}

#[tokio::test]
async fn readiness_is_awaited_on_the_instance_used_for_call() {
    let audit = Audit::default();
    let mut service = RiskLayer(RiskPolicy {
        max_batch_submit_quantity: 1,
    })
    .layer(
        ExecutionLayer {
            audit: audit.clone(),
            reject_commit: false,
        }
        .layer(BecomesReady { ready: false }),
    );
    run_batch(&mut service, Batch(vec![])).await.unwrap();
    assert_eq!(audit.events(), vec![Event::Committed(0)]);
}
