use std::future::Future;

use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::warn;

/// Добавляет возможность завершения задачи
pub async fn cancellable_task<T, E>(
    future: impl Future<Output = Result<T, E>>,
    cancel: CancellationToken,
) -> Result<T, E>
where
    T: Default,
{
    select! {
        val = future => {val},
        _ = cancel.cancelled() => {
            warn!("Cancel task");
            Ok(T::default())
        }
    }
}
