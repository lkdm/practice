mod tasks;

use std::time::Duration;

use tasks::{task_fallible_error, task_fallible_okay, task_nonfallible, task_timeout};
use tasks_example_rs::tasks::create_task;
use tokio::sync::broadcast;
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let (shutdown_send, _) = broadcast::channel(1);

    // Start configuring a `fmt` subscriber
    tracing_subscriber::registry()
        .with(EnvFilter::from("info"))
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_file(false)
                .with_line_number(false)
                .with_thread_ids(false)
                .with_target(false),
        )
        .init();

    info!("tracing started...");

    // Create tasks
    let task1 = create_task(
        Duration::from_secs(5),
        Duration::from_secs(30),
        || task_nonfallible(),
        &shutdown_send,
    );

    let task2 = create_task(
        Duration::from_secs(10),
        Duration::from_secs(30),
        || task_fallible_error(),
        &shutdown_send,
    );

    let task3 = create_task(
        Duration::from_secs(15),
        Duration::from_secs(30),
        || task_fallible_okay(),
        &shutdown_send,
    );

    let task4 = create_task(
        Duration::from_secs(4),
        Duration::from_secs(10),
        || task_timeout(),
        &shutdown_send,
    );

    // Simulate some work
    tokio::time::sleep(Duration::from_secs(30)).await;

    // Signal shutdown
    shutdown_send.send(()).unwrap();

    // Wait for tasks to complete
    let _ = tokio::join!(task1, task2, task3, task4);
}
