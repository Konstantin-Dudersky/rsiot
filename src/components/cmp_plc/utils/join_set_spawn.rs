use std::future::Future;

use tokio::task::JoinSet;

#[cfg(feature = "single-thread")]
pub fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, task: F)
where
    F: Future<Output = T> + 'static,
    T: Send + 'static,
{
    join_set.spawn_local(task);
}

#[cfg(not(feature = "single-thread"))]
pub fn join_set_spawn<F, T>(join_set: &mut JoinSet<T>, task: F)
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    join_set.spawn(task);
}
