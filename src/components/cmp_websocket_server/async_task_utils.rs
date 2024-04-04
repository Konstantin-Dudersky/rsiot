use std::future::Future;

use tokio::select;
use tokio_util::sync::CancellationToken;
use tracing::warn;

pub async fn cancellable_task<T>(future: impl Future<Output = T>, cancel: CancellationToken) -> T
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
