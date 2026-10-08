use asyncband::{semaphore::Semaphore, singleflight::Group};
use aye_asyncband_coordination::{StopOutcome, Worker, coalesced_read, stop_worker};
use std::cell::Cell;
use std::future::{Future, IntoFuture, poll_fn};
use std::pin::Pin;
use std::sync::Arc;
use std::task::Poll;

async fn poll_once<F: Future>(mut future: Pin<&mut F>) -> Poll<F::Output> {
    poll_fn(|cx| Poll::Ready(future.as_mut().poll(cx))).await
}

#[tokio::test]
async fn work_shares_a_failed_value_but_does_not_cache_it() {
    #[derive(Debug)]
    struct Failure;

    let flights = Group::new();
    let slots = Semaphore::new(1);
    let calls = Cell::new(0);
    let (release, gate) = tokio::sync::oneshot::channel();
    let mut first = Box::pin(coalesced_read(&flights, &slots, 7, async || {
        calls.set(calls.get() + 1);
        gate.await.unwrap();
        Err::<u32, _>(Failure)
    }));
    assert!(poll_once(first.as_mut()).await.is_pending());
    assert_eq!(slots.available_permits(), 0);
    let mut follower = Box::pin(coalesced_read(&flights, &slots, 7, async || {
        calls.set(calls.get() + 1);
        Ok::<_, Failure>(2)
    }));
    assert!(poll_once(follower.as_mut()).await.is_pending());
    assert_eq!(calls.get(), 1);
    release.send(()).unwrap();
    let first_error = first.await.unwrap_err();
    let follower_error = follower.await.unwrap_err();
    assert!(Arc::ptr_eq(&first_error, &follower_error));
    assert_eq!(slots.available_permits(), 1);
    let later = coalesced_read(&flights, &slots, 7, async || {
        calls.set(calls.get() + 1);
        Ok::<_, Failure>(3)
    })
    .await
    .unwrap();
    assert_eq!(*later, 3);
    assert_eq!(calls.get(), 2);
}

#[tokio::test]
async fn try_work_retries_a_followers_initializer_after_error() {
    let flights = Group::new();
    let calls = Cell::new(0);
    let (release, gate) = tokio::sync::oneshot::channel();
    let mut first = Box::pin(flights.try_work("key", async || {
        calls.set(calls.get() + 1);
        gate.await.unwrap();
        Err::<u32, _>("failed")
    }));
    assert!(poll_once(first.as_mut()).await.is_pending());
    let mut follower = Box::pin(flights.try_work("key", async || {
        calls.set(calls.get() + 1);
        Ok::<_, &str>(2)
    }));
    assert!(poll_once(follower.as_mut()).await.is_pending());
    release.send(()).unwrap();
    assert_eq!(first.await, Err("failed"));
    assert_eq!(follower.await, Ok(2));
    assert_eq!(calls.get(), 2);
}

#[tokio::test]
async fn cancelling_the_leader_allows_a_follower_to_execute() {
    let flights = Group::new();
    let calls = Cell::new(0);
    let mut first = Box::pin(flights.work(1, async || {
        calls.set(calls.get() + 1);
        std::future::pending::<u32>().await
    }));
    assert!(poll_once(first.as_mut()).await.is_pending());
    let mut follower = Box::pin(flights.work(1, async || {
        calls.set(calls.get() + 1);
        2
    }));
    assert!(poll_once(follower.as_mut()).await.is_pending());
    drop(first);
    assert_eq!(follower.await, 2);
    assert_eq!(calls.get(), 2);
}

#[tokio::test]
async fn distinct_keys_share_the_backend_concurrency_limit() {
    let flights = Group::new();
    let slots = Semaphore::new(1);
    let calls = Cell::new(0);
    let mut first = Box::pin(coalesced_read(&flights, &slots, 1, async || {
        calls.set(calls.get() + 1);
        std::future::pending::<Result<u32, ()>>().await
    }));
    assert!(poll_once(first.as_mut()).await.is_pending());
    let mut other = Box::pin(coalesced_read(&flights, &slots, 2, async || {
        calls.set(calls.get() + 1);
        Ok::<_, ()>(2)
    }));
    assert!(poll_once(other.as_mut()).await.is_pending());
    assert_eq!(calls.get(), 1);
    drop(first);
    assert_eq!(*other.await.unwrap(), 2);
    assert_eq!(calls.get(), 2);
}

#[tokio::test]
async fn a_waitgroup_participant_drop_also_completes() {
    let group = asyncband::waitgroup::WaitGroup::new();
    let participant = group.clone();
    let mut observer = Box::pin(group.into_future());
    assert!(poll_once(observer.as_mut()).await.is_pending());
    drop(participant);
    observer.await;
}

#[tokio::test]
async fn shutdown_signal_survives_cancellation_and_watches_do_not_hold_completion() {
    let (shutdown, guard) = asyncband::shutdown::new();
    let watch = guard.watch();
    drop(shutdown.clone());
    assert!(!guard.is_shutdown_requested());
    let mut wait = Box::pin(shutdown.clone());
    assert!(poll_once(wait.as_mut()).await.is_pending());
    assert!(watch.is_shutdown_requested());
    drop(wait);
    assert!(guard.is_shutdown_requested());
    drop(guard);
    shutdown.await;
    assert!(watch.is_shutdown_requested());
}

#[tokio::test]
async fn lazycell_resumes_the_same_initializer_after_cancellation() {
    let attempts = Cell::new(0);
    let (release, gate) = tokio::sync::oneshot::channel();
    let lazy = asyncband::once::LazyCell::new(|| {
        attempts.set(attempts.get() + 1);
        Box::pin(async { gate.await.unwrap() })
    });
    let mut first = Box::pin(asyncband::once::LazyCell::force(&lazy));
    assert!(poll_once(first.as_mut()).await.is_pending());
    drop(first);
    release.send(42).unwrap();
    assert_eq!(*asyncband::once::LazyCell::force(&lazy).await, 42);
    assert_eq!(attempts.get(), 1);
}

#[tokio::test]
async fn cancelling_acquire_does_not_consume_a_released_permit() {
    let slots = Semaphore::new(1);
    let held = slots.acquire(1).await;
    let mut wait = Box::pin(slots.acquire(1));
    assert!(poll_once(wait.as_mut()).await.is_pending());
    drop(wait);
    drop(held);
    assert!(slots.try_acquire(1).is_some());
    assert_eq!(slots.available_permits(), 1);
}

#[tokio::test]
async fn slow_broadcast_receiver_blocks_send_until_it_is_dropped() {
    let (sender, mut fast) = asyncband::broadcast::mpmc::bounded(1);
    let slow = sender.subscribe();
    sender.send(1).await;
    assert_eq!(fast.recv().await, Ok(1));
    let mut next = Box::pin(sender.send(2));
    assert!(poll_once(next.as_mut()).await.is_pending());
    drop(slow);
    next.await;
    assert_eq!(fast.recv().await, Ok(2));
}

#[tokio::test(start_paused = true)]
async fn stop_worker_observes_success() {
    let (shutdown, guard) = asyncband::shutdown::new();
    let mut worker = tokio::spawn(async move {
        guard.shutdown_requested().await;
        drop(guard);
    });
    assert_eq!(
        stop_worker(&shutdown, &mut worker, std::time::Duration::from_secs(1))
            .await
            .unwrap(),
        StopOutcome::Drained
    );
}

#[tokio::test(start_paused = true)]
async fn stop_worker_aborts_and_waits_when_cleanup_exceeds_budget() {
    let (shutdown, guard) = asyncband::shutdown::new();
    let mut worker = tokio::spawn(async move {
        guard.shutdown_requested().await;
        std::future::pending::<()>().await;
        drop(guard);
    });
    assert_eq!(
        stop_worker(&shutdown, &mut worker, std::time::Duration::from_secs(1))
            .await
            .unwrap(),
        StopOutcome::Aborted
    );
    shutdown.await;
}

#[tokio::test(start_paused = true)]
async fn stop_worker_reports_a_panic_even_after_guard_completion() {
    let (shutdown, guard) = asyncband::shutdown::new();
    let mut worker = tokio::spawn(async move {
        guard.shutdown_requested().await;
        panic!("worker failed");
    });
    let error = stop_worker(&shutdown, &mut worker, std::time::Duration::from_secs(1))
        .await
        .unwrap_err();
    assert!(error.is_panic());
}

#[tokio::test(start_paused = true)]
async fn stop_worker_budget_also_covers_join_after_an_early_guard_drop() {
    let (shutdown, guard) = asyncband::shutdown::new();
    let mut worker = tokio::spawn(async move {
        guard.shutdown_requested().await;
        drop(guard);
        std::future::pending::<()>().await;
    });
    assert_eq!(
        stop_worker(&shutdown, &mut worker, std::time::Duration::from_secs(1))
            .await
            .unwrap(),
        StopOutcome::Aborted
    );
}

#[tokio::test]
async fn cancelling_borrowed_shutdown_leaves_a_joinable_handle_with_the_caller() {
    let (shutdown, guard) = asyncband::shutdown::new();
    let watch = guard.watch();
    let (release, gate) = tokio::sync::oneshot::channel();
    let mut task = tokio::spawn(async move {
        guard.shutdown_requested().await;
        gate.await.unwrap();
        drop(guard);
    });
    let mut wait = Box::pin(stop_worker(
        &shutdown,
        &mut task,
        std::time::Duration::from_secs(1),
    ));
    assert!(poll_once(wait.as_mut()).await.is_pending());
    drop(wait);
    assert!(watch.is_shutdown_requested());
    release.send(()).unwrap();
    task.await.unwrap();
    shutdown.await;
}

struct ExitSignal(Option<tokio::sync::oneshot::Sender<()>>);

impl Drop for ExitSignal {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _ = sender.send(());
        }
    }
}

async fn pending_worker() -> (Worker, tokio::sync::oneshot::Receiver<()>) {
    let (started, startup) = tokio::sync::oneshot::channel();
    let (exited, exit) = tokio::sync::oneshot::channel();
    let worker = Worker::spawn(move |watch| async move {
        let _exit = ExitSignal(Some(exited));
        started.send(()).unwrap();
        watch.shutdown_requested().await;
        std::future::pending::<()>().await;
    });
    startup.await.unwrap();
    (worker, exit)
}

#[tokio::test(start_paused = true)]
async fn dropping_the_owner_cancels_the_running_worker() {
    let (worker, exit) = pending_worker().await;
    drop(worker);
    tokio::time::timeout(std::time::Duration::from_secs(1), exit)
        .await
        .expect("owner Drop must request abort")
        .expect("the running worker must drop its local state");
}

#[tokio::test(start_paused = true)]
async fn cancelling_owned_shutdown_cancels_the_running_worker() {
    let (worker, exit) = pending_worker().await;
    let mut wait = Box::pin(worker.shutdown(std::time::Duration::from_secs(1)));
    assert!(poll_once(wait.as_mut()).await.is_pending());
    drop(wait);
    tokio::time::timeout(std::time::Duration::from_secs(1), exit)
        .await
        .expect("cancelled shutdown must drop the owner and request abort")
        .expect("the running worker must drop its local state");
}
