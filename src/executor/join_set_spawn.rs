use std::future::Future;

use tokio::task::JoinSet;

#[cfg(feature = "single-thread")]
/// Добавить задачу в множество задач (однопоточная версия)
pub fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, name: impl AsRef<str>, task: F)
where
    F: Future<Output = T> + 'static,
    T: Send + 'static,
{
    #[cfg(feature = "log_tokio")]
    {
        use tracing::error;

        let res = join_set.build_task().name(name.as_ref()).spawn_local(task);
        if let Err(e) = res {
            error!("Error spawning task: {}", e);
        }
    }

    #[cfg(not(feature = "log_tokio"))]
    {
        let _ = name;
        join_set.spawn_local(task);
    }
}

#[cfg(not(feature = "single-thread"))]
/// Добавить задачу в множество задач (многопоточная версия)
pub fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, name: impl AsRef<str>, task: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    use tracing::error;

    let res = join_set.build_task().name(name.as_ref()).spawn(task);

    if let Err(e) = res {
        error!("Error spawning task: {}", e);
    }
}

/// Добавить блокирующую задачу в множество задач (многопоточная версия)
pub fn join_set_spawn_blocking<F, T>(join_set: &mut JoinSet<T>, name: impl AsRef<str>, task: F)
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    #[cfg(feature = "log_tokio")]
    {
        use tracing::error;

        let res = join_set
            .build_task()
            .name(name.as_ref())
            .spawn_blocking(task);

        if let Err(e) = res {
            error!("Error spawning task: {}", e);
        }
    }

    #[cfg(not(feature = "log_tokio"))]
    {
        let _ = name;
        join_set.spawn_blocking(task);
    }
}
