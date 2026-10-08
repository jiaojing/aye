//! Asyncband coordination examples; application policy remains with the caller.

use std::hash::Hash;
use std::sync::Arc;

use asyncband::{semaphore::Semaphore, singleflight::Group};

/// Coalesce overlapping reads and bound actual backend work.
///
/// The key must identify equivalent reads. Errors are shared as completed values;
/// cancellation of the active initializer may still let a waiting caller execute again.
pub async fn coalesced_read<K, T, E>(
    flights: &Group<K, Result<Arc<T>, Arc<E>>>,
    backend_slots: &Semaphore,
    key: K,
    fetch: impl AsyncFnOnce() -> Result<T, E>,
) -> Result<Arc<T>, Arc<E>>
where
    K: Eq + Hash,
{
    flights
        .work(key, async || {
            let _permit = backend_slots.acquire(1).await;
            fetch().await.map(Arc::new).map_err(Arc::new)
        })
        .await
}

#[derive(Debug, PartialEq, Eq)]
pub enum StopOutcome {
    /// The task returned successfully; draining is part of its application contract.
    Drained,
    /// The budget expired and cancellation was observed after requesting abort.
    Aborted,
}

/// Request shutdown and share one budget between guard completion and task joining.
///
/// Cancellation leaves the handle with the caller. Abort relies on cooperative
/// scheduling; it does not interrupt blocking work or perform asynchronous cleanup.
pub async fn stop_worker(
    shutdown: &asyncband::shutdown::Shutdown,
    worker: &mut tokio::task::JoinHandle<()>,
    budget: std::time::Duration,
) -> Result<StopOutcome, tokio::task::JoinError> {
    shutdown.request_shutdown();
    let completed = tokio::time::timeout(budget, async {
        shutdown.clone().await;
        (&mut *worker).await
    })
    .await;
    match completed {
        Ok(result) => result.map(|()| StopOutcome::Drained),
        Err(_) => {
            worker.abort();
            match worker.await {
                Ok(()) => Ok(StopOutcome::Drained),
                Err(error) if error.is_cancelled() => Ok(StopOutcome::Aborted),
                Err(error) => Err(error),
            }
        }
    }
}

/// An owner for one ordinary async worker, including cancellation of shutdown itself.
///
/// This small example owns execution policy. It is not a general worker framework.
pub struct Worker {
    shutdown: asyncband::shutdown::Shutdown,
    task: tokio::task::JoinHandle<()>,
}

impl Worker {
    pub fn spawn<F, Fut>(run: F) -> Self
    where
        F: FnOnce(asyncband::shutdown::ShutdownWatch) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = ()> + Send + 'static,
    {
        let (shutdown, guard) = asyncband::shutdown::new();
        let watch = guard.watch();
        let task = tokio::spawn(async move {
            run(watch).await;
            // The owner keeps this participant alive through all user work.
            drop(guard);
        });
        Self { shutdown, task }
    }

    pub async fn shutdown(
        mut self,
        budget: std::time::Duration,
    ) -> Result<StopOutcome, tokio::task::JoinError> {
        stop_worker(&self.shutdown, &mut self.task, budget).await
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.shutdown.request_shutdown();
        self.task.abort(); // Drop cannot await. Explicit shutdown observes completion.
    }
}
