use std::time::Duration;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::{error, info};

use crate::message::{AuthPermissions, Message, MsgDataBound};

#[cfg(feature = "log_tokio")]
use super::task_runtime_metrics::TaskRuntimeMetrics;
use super::{
    Cache, MsgBusLinker, TokioRuntimeMetrics, component::IComponent, error::ComponentError,
    join_set_spawn, task_internal::TaskInternal, types::FnAuth,
};

#[cfg(feature = "log_tokio")]
const RUNTIME_METRICS_PERIOD: Duration = Duration::from_millis(100);

pub type FnTokioMetrics<TMsg> = fn(TokioRuntimeMetrics) -> Option<TMsg>;

/// Запуск коллекции компонентов в работу
pub struct ComponentExecutor<TMsg>
where
    TMsg: MsgDataBound,
{
    task_set: JoinSet<Result<(), ComponentError>>,
    cmp_in_out: MsgBusLinker<TMsg>,
}

/// Настройка исполнителя
pub struct ComponentExecutorConfig<TMsg> {
    /// Размер буфера канала сообщения
    pub buffer_size: usize,

    /// Функция фильтрации сообщений в зависимости от текущей авторизации
    ///
    /// **Примеры**
    ///
    /// - Все сообщения блокируются
    ///
    /// ```rust
    /// |_, _| None
    /// ```
    ///
    /// - Все сообщения разрешены
    ///
    /// ```rust
    /// |msg, _| Some(msg)
    /// ```
    pub fn_auth: FnAuth<TMsg>,

    /// Задержка публикации сообщений
    ///
    /// Рассылка сообщений осуществляется по каналу broadcast. При инициализации компоненты
    /// получают только новые сообщения. Эта задержка нужна для того, чтобы компоненты успели
    /// запуститься.
    pub delay_publish: Duration,

    /// Функция создания сообщения с метриками `tokio`
    ///
    /// Заглушка: `|_| None`
    pub fn_tokio_metrics: FnTokioMetrics<TMsg>,
}

impl<TMsg> ComponentExecutor<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    /// Создание коллекции компонентов
    pub fn new(config: ComponentExecutorConfig<TMsg>) -> Self {
        info!("ComponentExecutor start creation");
        let (component_input_send, component_input) =
            broadcast::channel::<Message<TMsg>>(config.buffer_size);
        let (component_output, component_output_recv) =
            mpsc::channel::<Message<TMsg>>(config.buffer_size);
        let cache: Cache<TMsg> = Cache::new();
        let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

        // Запускаем внутреннюю задачу

        let task = TaskInternal {
            output: component_output_recv,
            input: component_input_send.clone(),
            cache: cache.clone(),
            delay_publish: config.delay_publish,
            max_capacity: config.buffer_size,
        };
        join_set_spawn(&mut task_set, "internal_task", task.spawn());

        // Запускаем задачу сбора метрик tokio
        #[cfg(feature = "log_tokio")]
        {
            let task = TaskRuntimeMetrics::<TMsg> {
                output: component_input_send,
                period: RUNTIME_METRICS_PERIOD,
                fn_tokio_metrics: config.fn_tokio_metrics,
            };
            join_set_spawn(&mut task_set, "tokio_metrics", task.spawn());
        }

        let cmp_in_out = MsgBusLinker::new(
            component_input,
            component_output,
            AuthPermissions::default(),
            config.fn_auth,
        );

        Self {
            task_set,
            cmp_in_out,
        }
    }

    /// Добавить компонент
    #[cfg(not(feature = "single-thread"))]
    pub fn add_cmp(mut self, mut component: impl IComponent<TMsg> + Send + 'static) -> Self {
        component.set_interface(self.cmp_in_out.clone());

        self.task_set.spawn(async move { component.spawn().await });

        self
    }
    /// Добавить компонент (?Send)
    #[cfg(feature = "single-thread")]
    pub fn add_cmp(mut self, mut component: impl IComponent<TMsg> + 'static) -> Self {
        component.set_interface(self.cmp_in_out.clone());

        self.task_set
            .spawn_local(async move { component.spawn().await });
        self
    }

    /// Запустить на выполнение все компоненты и ожидать завершения выполнения выполнения какого-то
    /// компонента.
    pub async fn wait_result(mut self) -> Result<(), ComponentError> {
        // Удаляем неиспользуемые каналы шины сообщений
        drop(self.cmp_in_out);

        let msg;
        if let Some(result) = self.task_set.join_next().await {
            match result {
                Ok(Ok(_)) => {
                    msg = "Component has finished executing with Ok result".to_string();
                    info!(msg);
                    return Ok(());
                }

                Ok(Err(err)) => {
                    msg = format!("Component has finished executing with error: {err:?}");
                }

                Err(err) => {
                    msg = format!("Component has finished executing with error: {err:?}");
                }
            };
            error!(msg);
            return Err(ComponentError::Execution(msg));
        }
        Ok(())
    }
}
