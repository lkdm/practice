use std::future::Future;
use thiserror::Error;
use tokio::select;
use tokio::sync::broadcast::{self, Sender};
use tokio::task::JoinHandle;
use tokio::time::{Duration, interval, timeout};
use tracing::instrument;

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("task was asked to backoff")]
    Backoff,
    #[error("task timed out")]
    Timeout,
    #[error("task failed: {0}")]
    TaskFailed(Box<dyn std::error::Error + Send + Sync>),
}

// TODO: Add max_retries
// TODO: Maybe try and clean it up as a fsm
// - Waiting
// - Working

/// Creates a task that runs on an interval with graceful shutdown support
#[instrument(skip(f, shutdown_signal), fields(task_name = task_name))]
pub fn create_task<F, Fut, E>(
    task_name: &'static str,
    duration: Duration,
    timeout_duration: Duration,
    mut f: F,
    shutdown_signal: &Sender<()>,
) -> JoinHandle<()>
where
    F: FnMut() -> Fut + Send + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    E: std::fmt::Debug + Send + 'static,
{
    let mut shutdown_recv = shutdown_signal.subscribe();
    tokio::spawn(async move {
        let mut clock = interval(duration);
        loop {
            select! {
                // Interval has been reached
                _ = clock.tick() => {
                    // Execute function, but start a timeout
                    match timeout(timeout_duration, f()).await {
                        Ok(Ok(_)) => {
                            tracing::info!("Task completed")
                        },
                        Ok(Err(e)) => {
                            tracing::debug!("Task encountered an error: {:?}", e);
                            eprintln!("Task encountered an error: {:?}", e);
                        }
                        Err(_) => {
                            tracing::debug!("Task timed out");
                            eprintln!("{}", TaskError::Timeout);
                            continue;
                        }

                    }
                   }
                // Shutdown signal
                _ = shutdown_recv.recv() => {
                    break;
                }
            }
        }
    })
}

#[derive(Debug, Error)]
#[error("task failed")]
pub struct MyTaskError;

#[instrument]
pub async fn task_nonfallible() -> Result<(), MyTaskError> {
    println!("Executing nonfallible task");
    Ok(())
}

#[instrument]
pub async fn task_fallible_error() -> Result<(), MyTaskError> {
    println!("Executing fallible task: ERROR");
    Err(MyTaskError)
}

#[instrument]
pub async fn task_fallible_okay() -> Result<(), MyTaskError> {
    println!("Executing fallible task: OKAY");
    Ok(())
}

#[instrument]
pub async fn task_timeout() -> Result<(), MyTaskError> {
    tokio::time::sleep(Duration::from_secs(420)).await;
    Ok(())
}
