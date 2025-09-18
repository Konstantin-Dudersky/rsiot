use std::future::Future;

use tokio::task::JoinSet;
use tracing::error;

#[cfg(feature = "single-thread")]
/// Добавить задачу в множество задач (однопоточная версия)
pub fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, name: impl AsRef<str>, task: F)
where
    F: Future<Output = T> + 'static,
    T: Send + 'static,
{
    let res = join_set.build_task().name(name.as_ref()).spawn_local(task);
    if let Err(e) = res {
        error!("Error spawning task: {}", e);
    }
}

#[cfg(not(feature = "single-thread"))]
/// Добавить задачу в множество задач (многопоточная версия)
pub fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, name: impl AsRef<str>, task: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let res = join_set.build_task().name(name.as_ref()).spawn(task);
    if let Err(e) = res {
        error!("Error spawning task: {}", e);
    }
}
