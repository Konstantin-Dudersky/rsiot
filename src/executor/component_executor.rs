use std::time::Duration;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::{debug, error, info, trace, warn};
use uuid::Uuid;

use crate::message::{system_messages::*, *};

use super::{
    component::IComponent, error::ComponentError, join_set_spawn, sleep, types::FnAuth, Cache,
    CmpInOut, TokioRuntimeMetrics,
};

const UPDATE_TTL_PERIOD: Duration = Duration::from_millis(1000);
const RUNTIME_METRICS_PERIOD: Duration = Duration::from_millis(100);

pub type FnTokioMetrics<TMsg> = fn(TokioRuntimeMetrics) -> Option<TMsg>;

/// Запуск коллекции компонентов в работу
pub struct ComponentExecutor<TMsg>
where
    TMsg: MsgDataBound,
{
    task_set: JoinSet<Result<(), ComponentError>>,
    cmp_in_out: CmpInOut<TMsg>,
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
        let id = MsgTrace::generate_uuid();
        let (component_input_send, component_input) =
            broadcast::channel::<Message<TMsg>>(config.buffer_size);
        let (component_output, component_output_recv) =
            mpsc::channel::<Message<TMsg>>(config.buffer_size);
        let cache: Cache<TMsg> = Cache::new();
        let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

        // Запускаем внутреннюю задачу
        let task_internal_handle = task_internal(
            component_output_recv,
            component_input_send.clone(),
            cache.clone(),
            id,
            config.delay_publish,
        );
        join_set_spawn(&mut task_set, "internal_task", task_internal_handle);

        // Запускаем задачу обновления TTL сообщений
        let task_update_ttl_in_cache_handle = task_update_ttl_in_cache(cache.clone());
        join_set_spawn(
            &mut task_set,
            "update_ttl_in_cache",
            task_update_ttl_in_cache_handle,
        );

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

        let cmp_in_out = CmpInOut::new(
            component_input,
            component_output,
            cache.clone(),
            "Trace name (maybe delete?)",
            id,
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

    /// Запустить на выполнение все компоненты.
    ///
    /// Компоненты не должны заканчивать выполнение. Если хоть один остановился (неважно по какой
    /// причине - по ошибке или нет), это ошибка выполнения.
    pub async fn wait_result(&mut self) -> Result<(), ComponentError> {
        let msg;
        if let Some(result) = self.task_set.join_next().await {
            match result {
                Ok(result) => match result {
                    Ok(_) => msg = "Component has finished executing".to_string(),
                    Err(err) => {
                        msg = format!("Component has finished executing with error: {err:?}");
                    }
                },
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

async fn task_internal<TMsg>(
    mut input: mpsc::Receiver<Message<TMsg>>,
    output: broadcast::Sender<Message<TMsg>>,
    cache: Cache<TMsg>,
    executor_id: Uuid,
    delay_publish: Duration,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    debug!("Internal task of ComponentExecutor: starting");

    // Задержка, чтобы компоненты успели запуститься и подписаться на получение сообщений
    sleep(delay_publish).await;

    while let Some(mut msg) = input.recv().await {
        // TODO
        trace!("ComponentExecutor: new message: {:?}", msg);
        // msg.add_trace_item(&executor_id, &format!("{}::internal_bus", service_name));
        msg.add_trace_item(&executor_id);
        let msg = save_msg_in_cache(msg, &cache).await;
        let Some(msg) = msg else { continue };
        output.send(msg).map_err(|err| {
            let err =
                format!("Internal task of ComponentExecutor: send to channel error, {err:?}",);
            ComponentError::Initialization(err)
        })?;
    }
    warn!("Internal task: stop");
    Ok(())
}

#[cfg(feature = "log_tokio")]
struct TaskRuntimeMetrics<TMsg>
where
    TMsg: MsgDataBound,
{
    pub output: broadcast::Sender<Message<TMsg>>,
    pub period: Duration,
    pub fn_tokio_metrics: FnTokioMetrics<TMsg>,
}
#[cfg(feature = "log_tokio")]
impl<TMsg> TaskRuntimeMetrics<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), ComponentError> {
        let handle = tokio::runtime::Handle::current();
        let runtime_monitor = tokio_metrics::RuntimeMonitor::new(&handle);
        for metrics in runtime_monitor.intervals() {
            tokio::time::sleep(self.period).await;
            let metrics: TokioRuntimeMetrics = metrics.into();
            let msg = (self.fn_tokio_metrics)(metrics);
            let Some(msg) = msg else { continue };
            let msg = Message::new_custom(msg);
            self.output.send(msg).map_err(|err| {
                let err =
                    format!("Internal task of ComponentExecutor: send to channel error, {err:?}");
                ComponentError::Initialization(err)
            })?;
        }
        Ok(())
    }
}

/// Обновить значения времени жизни сообщений. Удаляет сообщения, время которых истекло
async fn task_update_ttl_in_cache<TMsg>(cache: Cache<TMsg>) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    loop {
        sleep(UPDATE_TTL_PERIOD).await;
        let mut cache = cache.write().await;
        let mut keys_for_delete = vec![];

        cache.iter_mut().for_each(|(key, msg)| {
            msg.update_time_to_live(UPDATE_TTL_PERIOD);
            if !msg.is_alive() {
                keys_for_delete.push(key.clone());
            }
        });
        for key in keys_for_delete {
            let remove_result = cache.remove(&key);
            if remove_result.is_none() {
                let err = format!("Message with key {key} not found in cache",);
                return Err(ComponentError::Execution(err));
            }
        }
    }
}

/// Сохраняем сообщение в кеше
///
/// Возвращает `Option<Message>`:
/// - None - сообщение не нужно отправлять дальше
/// - Some(Message) - сообщение нужно отправить на вход всех компонентов
async fn save_msg_in_cache<TMsg>(msg: Message<TMsg>, cache: &Cache<TMsg>) -> Option<Message<TMsg>>
where
    TMsg: MsgDataBound,
{
    // Фильтруем сообщения авторизации
    if let MsgData::System(data) = &msg.data {
        match data {
            System::AuthRequestByLogin(_) => return Some(msg),
            System::AuthRequestByToken(_) => return Some(msg),
            System::AuthResponseErr(_) => return Some(msg),
            System::AuthResponseOk(_) => return Some(msg),
            System::EspWifiConnected => return Some(msg),
            System::Ping(_) => return None,
            System::Pong(_) => return None,
        }
    }
    // Время жизни сообщения истекло
    if !msg.is_alive() {
        return Some(msg);
    }
    let key = msg.key.clone();
    let value = msg.clone();
    {
        let mut lock = cache.write().await;
        let value_from_cache = lock.get(&key);
        if let Some(_value_from_cache) = value_from_cache {
            // если в кеше более новое сообщение, отбрасываем
            // if value.ts <= value_from_cache.ts {
            //     return None;
            // }
        }
        lock.insert(key, value);
    }
    Some(msg)
}
