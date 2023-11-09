use std::future::Future;

use tokio::{select, task::JoinHandle};
use tokio_util::sync::CancellationToken;
use tracing::warn;

pub async fn cancellable_task<T>(
    future: impl Future<Output = T>,
    cancel: CancellationToken,
) -> T
where
    T: Default,
{
    select! {
        val = future => {val},
        _ = cancel.cancelled() => {
            warn!("Cancel task");
            T::default()
        }
    }
}

pub async fn flatten_task_result<T, E1, E2>(
    handle: JoinHandle<Result<T, E1>>,
    join_handle_error: E2,
) -> Result<T, E2>
where
    E2: From<E1>,
{
    match handle.await {
        Ok(Ok(result)) => Ok(result),
        Ok(Err(err)) => Err(err.into()),
        Err(_) => Err(join_handle_error),
    }
}
