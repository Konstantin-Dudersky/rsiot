use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use rsiot_messages_core::{system_messages::*, *};
use tracing::{debug, error, info, trace, warn};
use uuid::Uuid;

use crate::{error::ComponentError, types::FnAuth, Cache, CmpInOut, IComponent};

/// Запуск коллекции компонентов в работу
///
/// # Примеры
///
/// ## Многопоточное окружение
///
/// TODO
///
/// ## Однопоточное окружение
///
/// TODO
///
/// ## Однопоточное окружение - WASM (Leptos)
///
/// ```rust
/// use leptos::*;
///
/// let context = LocalSet::new();
/// context.spawn_local(async move {
///     ComponentExecutor::<Message>::new(100, "example")
///         .add_cmp(cmp_websocket_client_wasm::Cmp::new(ws_client_config))
///         .add_cmp(cmp_leptos::Cmp::new(leptos_config))
///         .wait_result()
///         .await?;
///     Ok(()) as anyhow::Result<()>
/// });
/// spawn_local(context);
/// Ok(())
/// ```
pub struct ComponentExecutor<TMsg> {
    task_set: JoinSet<Result<(), ComponentError>>,
    cmp_in_out: CmpInOut<TMsg>,
}

/// Настройка исполнителя
pub struct ComponentExecutorConfig<TMsg> {
    /// Размер буфера канала сообщения
    pub buffer_size: usize,

    /// Название исполнителя
    pub executor_name: String,

    /// Функция фильтрации сообщений в зависимости от текущей авторизации
    ///
    /// # Примеры
    ///
    /// ## Все сообщения блокируются
    ///
    /// ```rust
    /// |_, _| None
    /// ```
    ///
    /// ## Все сообщения разрешены
    ///
    /// ```rust
    /// |msg, _| Some(msg)
    /// ```
    pub fn_auth: FnAuth<TMsg>,
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

        let task_internal_handle = task_internal(
            component_output_recv,
            component_input_send.clone(),
            cache.clone(),
            config.executor_name.clone(),
            id,
        );

        if cfg!(feature = "single-thread") {
            task_set.spawn_local(task_internal_handle);
        } else {
            task_set.spawn(task_internal_handle);
        }

        let cmp_in_out = CmpInOut::new(
            component_input,
            component_output,
            cache.clone(),
            &config.executor_name,
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
                        msg = format!("Component has finished executing with error: {:?}", err);
                    }
                },
                Err(err) => {
                    msg = format!("Component has finished executing with error: {:?}", err);
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
    executor_name: String,
    executor_id: Uuid,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    debug!("Internal task of ComponentExecutor: starting");
    while let Some(mut msg) = input.recv().await {
        trace!("ComponentExecutor: new message: {:?}", msg);
        msg.add_trace_item(&executor_id, &format!("{executor_name}::internal_bus"));
        save_msg_in_cache(&msg, &cache).await;
        output.send(msg).map_err(|err| {
            let err = format!(
                "Internal task of ComponentExecutor: send to channel error, {:?}",
                err
            );
            ComponentError::Initialization(err)
        })?;
    }
    warn!("Internal task: stop");
    Ok(())
}

/// Сохраняем сообщение в кеше
async fn save_msg_in_cache<TMsg>(msg: &Message<TMsg>, cache: &Cache<TMsg>)
where
    TMsg: MsgDataBound,
{
    // Фильтруем сообщения авторизации
    if let MsgData::System(data) = &msg.data {
        match data {
            System::AuthRequestByLogin(_) => return,
            System::AuthRequestByToken(_) => return,
            System::AuthResponseErr(_) => return,
            System::AuthResponseOk(_) => return,
        }
    }
    let key = msg.key.clone();
    let value = msg.clone();
    {
        let mut lock = cache.write().await;
        // let value_from_cache = lock.get(&key);
        // if let Some(value_from_cache) = value_from_cache {
        //     // если в кеше более новое сообщение, отбрасываем
        //     if value.ts <= value_from_cache.ts {
        //         continue;
        //     }
        // }
        lock.insert(key, value);
    }
}
