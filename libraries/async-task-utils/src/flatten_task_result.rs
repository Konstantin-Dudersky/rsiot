use tokio::task::JoinHandle;

/// Упрощение результата выполнения задачи
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
