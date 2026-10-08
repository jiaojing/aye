//! 有状态 worker 由 Service 拥有；client 只拥有发送能力。
//! 正常关闭排空已接受工作；超时或 owner 被丢弃时放弃剩余工作。

use std::{future::Future, num::NonZeroUsize, time::Duration};
use tokio::{
    sync::{mpsc, oneshot},
    task::{JoinError, JoinHandle},
    time::timeout,
};

#[derive(Clone)]
struct Client(mpsc::Sender<u32>);

impl Client {
    async fn submit(&self, job: u32) -> Result<(), mpsc::error::SendError<u32>> {
        self.0.send(job).await
    }
}

struct Service {
    client: Client,
    stop: Option<oneshot::Sender<()>>,
    task: Option<JoinHandle<usize>>,
}

#[derive(Debug)]
enum ShutdownError {
    Worker(JoinError),
    TimedOut,
}

impl std::fmt::Display for ShutdownError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Worker(_) => f.write_str("worker failed"),
            Self::TimedOut => f.write_str("shutdown deadline exceeded"),
        }
    }
}

impl std::error::Error for ShutdownError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Worker(error) => Some(error),
            Self::TimedOut => None,
        }
    }
}

impl Service {
    fn start<F, Fut>(capacity: NonZeroUsize, mut process: F) -> Self
    where
        F: FnMut(u32) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send,
    {
        let (tx, mut rx) = mpsc::channel(capacity.get());
        let (stop, mut stopped) = oneshot::channel();
        let task = tokio::spawn(async move {
            let mut completed = 0;
            let mut draining = false;
            loop {
                let next = if draining {
                    rx.recv().await
                } else {
                    tokio::select! {
                        biased;
                        _ = &mut stopped => {
                            rx.close();
                            draining = true;
                            continue;
                        }
                        job = rx.recv() => job,
                    }
                };
                let Some(job) = next else {
                    return completed;
                };
                process(job).await;
                completed += 1;
            }
        });
        Self {
            client: Client(tx),
            stop: Some(stop),
            task: Some(task),
        }
    }

    fn client(&self) -> Client {
        self.client.clone()
    }

    async fn shutdown(mut self, budget: Duration) -> Result<usize, ShutdownError> {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(()); // worker 可能已结束，join 仍负责报告结果。
        }
        // handle 留在 owner 内；shutdown future 被取消时 Drop 仍可 abort。
        let task = self
            .task
            .as_mut()
            .expect("service owns its worker until joined");
        let result = match timeout(budget, &mut *task).await {
            Ok(result) => result.map_err(ShutdownError::Worker),
            Err(_) => {
                task.abort();
                let _ = task.await; // 观察 worker 结束；处理器仍需遵守非阻塞约定。
                Err(ShutdownError::TimedOut)
            }
        };
        self.task.take();
        result
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort(); // 同步兜底只请求终止；显式 shutdown 才报告完成。
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let service = Service::start(NonZeroUsize::new(2).unwrap(), |job| async move {
        println!("processed job {job}");
    });
    let client = service.client();
    client.submit(1).await.unwrap();
    client.submit(2).await.unwrap();
    assert_eq!(service.shutdown(Duration::from_secs(1)).await.unwrap(), 2);
    assert!(client.submit(3).await.is_err());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::sync::Notify;

    #[tokio::test]
    async fn shutdown_drains_accepted_jobs_and_closes_existing_clients() {
        let processed = Arc::new(Mutex::new(Vec::new()));
        let output = Arc::clone(&processed);
        let service = Service::start(NonZeroUsize::new(2).unwrap(), move |job| {
            output.lock().unwrap().push(job);
            async {}
        });
        let client = service.client();
        client.submit(1).await.unwrap();
        client.submit(2).await.unwrap();
        assert_eq!(service.shutdown(Duration::from_secs(1)).await.unwrap(), 2);
        assert_eq!(*processed.lock().unwrap(), vec![1, 2]);
        assert!(client.submit(3).await.is_err());
    }

    #[tokio::test]
    async fn bounded_queue_applies_backpressure_while_worker_is_busy() {
        let started = Arc::new(Notify::new());
        let release = Arc::new(Notify::new());
        let ready = Arc::clone(&started);
        let gate = Arc::clone(&release);
        let service = Service::start(NonZeroUsize::new(1).unwrap(), move |_| {
            let ready = Arc::clone(&ready);
            let gate = Arc::clone(&gate);
            async move {
                ready.notify_one();
                gate.notified().await;
            }
        });
        let client = service.client();
        client.submit(1).await.unwrap();
        started.notified().await;
        client.submit(2).await.unwrap();
        assert!(matches!(
            client.0.try_send(3),
            Err(mpsc::error::TrySendError::Full(3))
        ));
        release.notify_one();
        started.notified().await;
        release.notify_one();
        assert_eq!(service.shutdown(Duration::from_secs(1)).await.unwrap(), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn deadline_aborts_busy_worker_and_reports_timeout() {
        let started = Arc::new(Notify::new());
        let ready = Arc::clone(&started);
        let service = Service::start(NonZeroUsize::new(1).unwrap(), move |_| {
            let ready = Arc::clone(&ready);
            async move {
                ready.notify_one();
                std::future::pending::<()>().await;
            }
        });
        let client = service.client();
        let observer = service.task.as_ref().unwrap().abort_handle();
        client.submit(1).await.unwrap();
        started.notified().await;
        assert!(matches!(
            service.shutdown(Duration::from_secs(1)).await,
            Err(ShutdownError::TimedOut)
        ));
        assert!(observer.is_finished());
        assert!(client.submit(2).await.is_err());
    }

    #[tokio::test]
    async fn worker_panic_is_reported_to_owner() {
        let service = Service::start(NonZeroUsize::new(1).unwrap(), |_| async {
            panic!("demo worker failure");
        });
        service.client().submit(1).await.unwrap();
        let Err(ShutdownError::Worker(error)) = service.shutdown(Duration::from_secs(1)).await
        else {
            panic!("expected worker panic");
        };
        assert!(error.is_panic());
    }

    #[tokio::test]
    async fn cancelling_shutdown_does_not_detach_the_worker() {
        let (started, ready) = oneshot::channel();
        let mut started = Some(started);
        let service = Service::start(NonZeroUsize::new(1).unwrap(), move |_| {
            let started = started.take().unwrap();
            async move {
                started.send(()).unwrap();
                std::future::pending::<()>().await;
            }
        });
        let client = service.client();
        client.submit(1).await.unwrap();
        ready.await.unwrap();
        let observer = service.task.as_ref().unwrap().abort_handle();
        let mut shutdown = Box::pin(service.shutdown(Duration::from_secs(60)));
        assert!(matches!(
            std::future::poll_fn(|cx| { std::task::Poll::Ready(shutdown.as_mut().poll(cx)) }).await,
            std::task::Poll::Pending
        ));
        drop(shutdown);
        client.0.closed().await; // receiver 被释放，提供清理完成的同步证据。
        assert!(observer.is_finished());
    }
}
