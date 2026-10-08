//! Shared read failures, an accepted queue drained on shutdown, and broadcast backpressure.

use std::{cell::Cell, sync::Arc, time::Duration};

use anyhow::Context;
use asyncband::{broadcast::mpmc, mpsc, semaphore::Semaphore, singleflight::Group};
use aye_asyncband_coordination::{StopOutcome, Worker, coalesced_read};

#[derive(Debug, thiserror::Error)]
enum LoadError {
    #[error("configuration source is unavailable")]
    Unavailable,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    shared_reads().await;
    drain_accepted_work().await?;
    slow_subscription().await?;
    Ok(())
}

async fn shared_reads() {
    // The application owns the namespace and resource budget across calls.
    let flights = Group::new();
    let backend_slots = Semaphore::new(1);
    let calls = Cell::new(0);
    let key = ("tenant-a", "configuration-v1");
    let (release, gate) = tokio::sync::oneshot::channel();

    let (first, follower, ()) = tokio::join! {
        // Explicit order makes overlap deterministic without a sleep.
        biased;
        coalesced_read(&flights, &backend_slots, key, async || {
            calls.set(calls.get() + 1);
            gate.await.expect("release branch owns the sender");
            Err::<String, _>(LoadError::Unavailable)
        }),
        coalesced_read(&flights, &backend_slots, key, async || {
            calls.set(calls.get() + 1);
            Ok::<_, LoadError>("duplicate backend call".to_owned())
        }),
        async {
            release.send(()).expect("the active initializer holds the receiver");
        },
    };
    let first = first.expect_err("this backend attempt returns Unavailable");
    let follower = follower.expect_err("the overlapping caller shares the failure");
    assert!(Arc::ptr_eq(&first, &follower));
    assert_eq!(calls.get(), 1);
    match first.as_ref() {
        LoadError::Unavailable => println!("same key: two callers share one typed backend failure"),
    }

    let later = coalesced_read(&flights, &backend_slots, key, async || {
        calls.set(calls.get() + 1);
        Ok::<_, LoadError>("fresh configuration".to_owned())
    })
    .await
    .expect("the later simulated backend attempt succeeds");
    assert_eq!(calls.get(), 2);
    println!("later call: {later}; no completed cache is retained");
}

async fn drain_accepted_work() -> anyhow::Result<()> {
    let (sender, mut receiver) = mpsc::bounded(3);
    for job in ["build", "index", "publish"] {
        sender.send(job).await.context("enqueue accepted work")?;
    }
    // This example has one producer. Closing admission means dropping that producer.
    // A service with cloned producers must separately define how admission stops.
    drop(sender);

    let (completed, result) = tokio::sync::oneshot::channel();
    let worker = Worker::spawn(move |watch| async move {
        watch.shutdown_requested().await;
        let mut processed = Vec::new();
        while let Ok(job) = receiver.recv().await {
            processed.push(job);
        }
        let _ = completed.send(processed);
    });
    assert_eq!(
        worker.shutdown(Duration::from_secs(1)).await?,
        StopOutcome::Drained
    );
    let processed = result.await.context("worker reports drained jobs")?;
    assert_eq!(processed, ["build", "index", "publish"]);
    println!(
        "shutdown: drained {} accepted jobs and joined the worker",
        processed.len()
    );
    Ok(())
}

async fn slow_subscription() -> anyhow::Result<()> {
    let (sender, mut fast) = mpmc::bounded(1);
    let slow = sender.subscribe();
    sender.send("state-v1").await;
    assert_eq!(fast.recv().await?, "state-v1");
    assert_eq!(
        sender.try_send("state-v2"),
        Err(mpmc::TrySendError::Full("state-v2"))
    );

    // The application explicitly unsubscribes a receiver that will not drain.
    drop(slow);
    sender.send("state-v2").await;
    assert_eq!(fast.recv().await?, "state-v2");
    println!("broadcast: the slow subscription holds capacity until it drains or leaves");
    Ok(())
}
