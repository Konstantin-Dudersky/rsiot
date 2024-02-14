use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use rsiot_messages_core::IMessage;
use tracing::{debug, error, info, trace, warn};

use crate::{error::ComponentError, Cache, CmpOutput, ComponentInput, IComponent};

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
///     ComponentExecutor::<Message>::new(100)
///         .add_cmp(cmp_websocket_client_wasm::Cmp::new(ws_client_config))
///         .add_cmp(cmp_leptos::Cmp::new(leptos_config))
///         .wait_result()
///         .await?;
///     Ok(()) as anyhow::Result<()>
/// });
/// spawn_local(context);
/// Ok(())
/// ```
pub struct ComponentExecutor<TMessage>
where
    TMessage: IMessage,
{
    task_set: JoinSet<Result<(), ComponentError>>,
    component_input: ComponentInput<TMessage>,
    component_output: CmpOutput<TMessage>,
    cache: Cache<TMessage>,
}

impl<TMessage> ComponentExecutor<TMessage>
where
    TMessage: IMessage + 'static,
{
    /// Создание коллекции компонентов
    pub fn new(buffer_size: usize) -> Self {
        info!("ComponentExecutor start creation");
        let (component_input_send, component_input) = broadcast::channel::<TMessage>(buffer_size);
        let (component_output, component_output_recv) = mpsc::channel::<TMessage>(buffer_size);
        let cache: Cache<TMessage> = Cache::new();
        let mut task_set: JoinSet<Result<(), ComponentError>> = JoinSet::new();

        let task_internal_handle = task_internal(
            component_output_recv,
            component_input_send.clone(),
            cache.clone(),
        );

        if cfg!(feature = "single-thread") {
            task_set.spawn_local(task_internal_handle);
        } else {
            task_set.spawn(task_internal_handle);
        }

        let component_output = CmpOutput::new(component_output);
        Self {
            task_set,
            component_input,
            component_output,
            cache,
        }
    }

    /// Добавить компонент
    #[cfg(not(feature = "single-thread"))]
    pub fn add_cmp(mut self, mut component: impl IComponent<TMessage> + Send + 'static) -> Self
    where
        TMessage: IMessage,
    {
        component.set_interface(
            self.component_input.resubscribe(),
            self.component_output.clone(),
            self.cache.clone(),
        );

        self.task_set.spawn(async move { component.spawn().await });

        self
    }
    /// Добавить компонент (?Send)
    #[cfg(feature = "single-thread")]
    pub fn add_cmp(mut self, mut component: impl IComponent<TMessage> + 'static) -> Self
    where
        TMessage: IMessage,
    {
        component.set_interface(
            self.component_input.resubscribe(),
            self.component_output.clone(),
            self.cache.clone(),
        );

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

async fn task_internal<TMessage>(
    mut input: mpsc::Receiver<TMessage>,
    output: broadcast::Sender<TMessage>,
    cache: Cache<TMessage>,
) -> Result<(), ComponentError>
where
    TMessage: IMessage,
{
    debug!("Internal task of ComponentExecutor: starting");
    while let Some(msg) = input.recv().await {
        trace!("Internal task of ComponentExecutor: new message: {:?}", msg);
        let key = msg.key().clone();
        let value = msg.clone();
        {
            let mut lock = cache.write().await;
            let value_from_cache = lock.get(&key);
            if let Some(value_from_cache) = value_from_cache {
                // если в кеше более новое сообщение, отбрасываем
                if value.ts() <= value_from_cache.ts() {
                    continue;
                }
            }

            lock.insert(key, value);
        }
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
