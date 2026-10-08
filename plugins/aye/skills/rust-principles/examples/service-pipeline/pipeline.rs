//! 单线程内存示例：批次 policy、Layer 装配、提交与执行有不同的输入类型。

use std::cell::RefCell;
use std::future::{poll_fn, ready, Future};
use std::num::NonZeroU32;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

use tower_layer::Layer;
use tower_service::Service;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("in-memory commit rejected")]
    CommitRejected,
    #[error("execution outcome for commit {commit_id} is unknown")]
    OutcomeUnknown { commit_id: usize },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operation {
    Submit { order_id: u64, quantity: NonZeroU32 },
    Cancel { order_id: u64 },
}

pub struct Batch(pub Vec<Operation>);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Denied {
    pub order_id: u64,
    pub quantity: NonZeroU32,
    pub batch_limit: u64,
}

pub struct AdmittedBatch {
    allowed: Vec<Operation>,
    denied: Vec<Denied>,
}

#[derive(Clone, Copy)]
pub struct RiskPolicy {
    pub max_batch_submit_quantity: u64,
}

impl RiskPolicy {
    /// 纯批次规则；累计本批已放行的 submit，Cancel 不释放这个批次预算。
    /// 不代表账户资金、持仓或生产环境的完整风险模型。
    pub fn evaluate(self, batch: Batch) -> AdmittedBatch {
        let mut admitted = AdmittedBatch {
            allowed: Vec::new(),
            denied: Vec::new(),
        };
        let mut submitted = 0_u64;
        for operation in batch.0 {
            if let Operation::Submit { order_id, quantity } = &operation {
                match submitted.checked_add(u64::from(quantity.get())) {
                    Some(total) if total <= self.max_batch_submit_quantity => submitted = total,
                    _ => {
                        admitted.denied.push(Denied {
                            order_id: *order_id,
                            quantity: *quantity,
                            batch_limit: self.max_batch_submit_quantity,
                        });
                        continue;
                    }
                }
            }
            admitted.allowed.push(operation);
        }
        admitted
    }
}

pub struct RiskLayer(pub RiskPolicy);

impl<S> Layer<S> for RiskLayer {
    type Service = RiskService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RiskService {
            policy: self.0,
            inner,
        }
    }
}

pub struct RiskService<S> {
    policy: RiskPolicy,
    inner: S,
}

impl<S: Service<AdmittedBatch>> Service<Batch> for RiskService<S> {
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<std::result::Result<(), S::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Batch) -> Self::Future {
        // 调用已通过 poll_ready 的同一个 inner，不 clone 一个新 Service。
        self.inner.call(self.policy.evaluate(request))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    pub commit_id: usize,
    pub allowed: Vec<Operation>,
    pub denied: Vec<Denied>,
}

/// 只有内存提交阶段能构造；执行端不接受尚未提交的 Batch。
pub struct CommittedBatch(Record);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Committed(usize),
    Executed {
        commit_id: usize,
        operation: Operation,
    },
}

#[derive(Default)]
struct AuditState {
    records: Vec<Record>,
    events: Vec<Event>,
}

/// 单线程内存记录 owner；访问不跨 await，也不在借用中调用用户代码。
#[derive(Clone, Default)]
pub struct Audit(Rc<RefCell<AuditState>>);

impl Audit {
    pub fn records(&self) -> Vec<Record> {
        self.0.borrow().records.clone()
    }

    pub fn events(&self) -> Vec<Event> {
        self.0.borrow().events.clone()
    }
}

pub struct ExecutionLayer {
    pub audit: Audit,
    /// 仅用于模拟提交失败。
    pub reject_commit: bool,
}

impl<S> Layer<S> for ExecutionLayer {
    type Service = ExecutionPipeline<S>;

    fn layer(&self, execution: S) -> Self::Service {
        ExecutionPipeline {
            audit: self.audit.clone(),
            reject_commit: self.reject_commit,
            execution,
        }
    }
}

pub struct ExecutionPipeline<S> {
    audit: Audit,
    reject_commit: bool,
    execution: S,
}

impl<S> ExecutionPipeline<S> {
    fn commit(&mut self, admitted: AdmittedBatch) -> Result<CommittedBatch> {
        if self.reject_commit {
            return Err(Error::CommitRejected);
        }
        let mut state = self.audit.0.borrow_mut();
        let record = Record {
            commit_id: state.records.len(),
            allowed: admitted.allowed,
            denied: admitted.denied,
        };
        state.records.push(record.clone());
        state.events.push(Event::Committed(record.commit_id));
        Ok(CommittedBatch(record))
    }
}

impl<S> Service<AdmittedBatch> for ExecutionPipeline<S>
where
    S: Service<CommittedBatch, Response = Outcome, Error = Error>,
    S::Future: 'static,
{
    type Response = Outcome;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Outcome>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.execution.poll_ready(cx)
    }

    fn call(&mut self, admitted: AdmittedBatch) -> Self::Future {
        // 同步内存提交先完成；真实后端可以使用独立的异步提交执行单元。
        match self.commit(admitted) {
            Ok(committed) => Box::pin(self.execution.call(committed)),
            Err(error) => Box::pin(ready(Err(error))),
        }
    }
}

#[derive(Debug)]
pub struct Outcome {
    pub commit_id: usize,
    pub completed: Vec<Operation>,
    pub denied: Vec<Denied>,
}

pub struct SimulatedExecution {
    pub audit: Audit,
    /// 仅用于模拟首个副作用之后失去响应。
    pub lose_response: bool,
}

impl Service<CommittedBatch> for SimulatedExecution {
    type Response = Outcome;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Outcome>>>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, committed: CommittedBatch) -> Self::Future {
        let audit = self.audit.clone();
        let lose_response = self.lose_response;
        Box::pin(async move {
            let Record {
                commit_id,
                allowed,
                denied,
            } = committed.0;
            for operation in &allowed {
                audit.0.borrow_mut().events.push(Event::Executed {
                    commit_id,
                    operation: operation.clone(),
                });
                if lose_response {
                    return Err(Error::OutcomeUnknown { commit_id });
                }
            }
            Ok(Outcome {
                commit_id,
                completed: allowed,
                denied,
            })
        })
    }
}

pub async fn run_batch<S>(service: &mut S, batch: Batch) -> Result<Outcome>
where
    S: Service<Batch, Response = Outcome, Error = Error>,
{
    poll_fn(|cx| service.poll_ready(cx)).await?;
    service.call(batch).await
}
