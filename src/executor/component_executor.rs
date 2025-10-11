use std::time::Duration;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::{debug, error, info, trace, warn};

use crate::message::{system_messages::*, *};

use super::{
    Cache, LessInPeriod, MsgBusLinker, TokioRuntimeMetrics, component::IComponent,
    error::ComponentError, join_set_spawn, sleep, types::FnAuth,
};

#[cfg(feature = "log_tokio")]
const RUNTIME_METRICS_PERIOD: Duration = Duration::from_millis(100);

/// Уровень переполненности канала. Чем ближе к 1.0, тем раньше появится сообщение переполнения
const BRAODCAST_LAGGED_THRESHOLD: f64 = 0.3;

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
        let task_internal_handle = task_internal(
            component_output_recv,
            component_input_send.clone(),
            cache.clone(),
            config.delay_publish,
            config.buffer_size,
        );
        join_set_spawn(&mut task_set, "internal_task", task_internal_handle);

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

async fn task_internal<TMsg>(
    mut input: mpsc::Receiver<Message<TMsg>>,
    output: broadcast::Sender<Message<TMsg>>,
    cache: Cache<TMsg>,
    delay_publish: Duration,
    buffer_size: usize,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    debug!("Internal task of ComponentExecutor: starting");

    // Задержка, чтобы компоненты успели запуститься и подписаться на получение сообщений
    sleep(delay_publish).await;

    let stat = format!(
        r#"
MsgBus statistics:
Connected to input:           {}
Connected to output (strong): {}
Channel capacity:             {}
"#,
        output.receiver_count(),
        input.sender_strong_count(),
        buffer_size
    );

    info!("{stat}");

    let buffer_size = buffer_size as f64;

    let mut less_in_period = LessInPeriod::new(Duration::from_millis(100));

    while let Some(msg) = input.recv().await {
        trace!("ComponentExecutor: new message: {:?}", msg);
        let msg = save_msg_in_cache(msg, &cache).await;
        let Some(msg) = msg else { continue };

        // Проверяем переполненность канала
        check_broadcast_lagged(&output, buffer_size, &mut less_in_period)?;

        output
            .send(msg)
            .map_err(|_| ComponentError::TaskInternalSend)?;
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
            System::Lagged => return Some(msg),
            System::AuthRequestByLogin(_) => return Some(msg),
            System::AuthRequestByToken(_) => return Some(msg),
            System::AuthResponseErr(_) => return Some(msg),
            System::AuthResponseOk(_) => return Some(msg),
            System::Ping(_) => return None,
            System::Pong(_) => return None,
        }
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

/// Функция проверки переполненности канала
fn check_broadcast_lagged<TMsg>(
    output: &broadcast::Sender<Message<TMsg>>,
    buffer_size: f64,
    less_in_period: &mut LessInPeriod,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    let capacity = output.len() as f64;
    let percent = 1.0 - capacity / buffer_size;

    if percent <= BRAODCAST_LAGGED_THRESHOLD && less_in_period.check() {
        warn!(
            "MsgBus input buffer is full; current free space: {}",
            percent * 100.0
        );
        let msg = Message::new(MsgData::System(System::Lagged));
        output
            .send(msg)
            .map_err(|_| ComponentError::TaskInternalSend)?;
    }

    Ok(())
}
