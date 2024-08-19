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
    CmpInOut,
};

const UPDATE_TTL_PERIOD: Duration = Duration::from_millis(200);

/// Запуск коллекции компонентов в работу
pub struct ComponentExecutor<TMsg> {
    task_set: JoinSet<Result<(), ComponentError>>,
    cmp_in_out: CmpInOut<TMsg>,
}

/// Настройка исполнителя
pub struct ComponentExecutorConfig<TMsg, TService>
where
    TService: ServiceBound,
{
    /// Размер буфера канала сообщения
    pub buffer_size: usize,

    /// Название сервиса
    pub service: TService,

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
}

impl<TMsg> ComponentExecutor<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    /// Создание коллекции компонентов
    pub fn new<TService>(config: ComponentExecutorConfig<TMsg, TService>) -> Self
    where
        TService: ServiceBound + 'static,
    {
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
            config.service.clone(),
            id,
        );
        join_set_spawn(&mut task_set, task_internal_handle);

        // Запускаем задачу обновления TTL сообщений
        let task_update_ttl_in_cache_handle = task_update_ttl_in_cache(cache.clone());
        join_set_spawn(&mut task_set, task_update_ttl_in_cache_handle);

        let cmp_in_out = CmpInOut::new(
            component_input,
            component_output,
            cache.clone(),
            &config.service.trace_name(),
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

async fn task_internal<TMsg, TService>(
    mut input: mpsc::Receiver<Message<TMsg>>,
    output: broadcast::Sender<Message<TMsg>>,
    cache: Cache<TMsg>,
    service: TService,
    executor_id: Uuid,
) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    debug!("Internal task of ComponentExecutor: starting");
    let service_name = service.trace_name();
    while let Some(mut msg) = input.recv().await {
        trace!("ComponentExecutor: new message: {:?}", msg);
        msg.add_trace_item(&executor_id, &format!("{}::internal_bus", service_name));
        msg.set_service_origin(&service_name);
        let msg = save_msg_in_cache(msg, &cache).await;
        let Some(msg) = msg else { continue };
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

/// Обновить значения времени жизни сообщений. Удаляет сообщения, время которых истекло
async fn task_update_ttl_in_cache<TMsg>(cache: Cache<TMsg>) -> Result<(), ComponentError>
where
    TMsg: MsgDataBound,
{
    loop {
        sleep(UPDATE_TTL_PERIOD).await;
        let mut cache = cache.write().await;
        let mut keys_for_delete = vec![];
        for (key, msg) in cache.iter_mut() {
            msg.update_time_to_live(UPDATE_TTL_PERIOD);
            if !msg.is_alive() {
                keys_for_delete.push(key.clone());
            }
        }
        for key in keys_for_delete {
            let remove_result = cache.remove(&key);
            if remove_result.is_none() {
                let err = format!("Message with key {} not found in cache", key);
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

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(all(wasm32_unknown_unknown, feature = "cmp_leptos"))]
    #[allow(dead_code)]
    fn test_wasm() {
        use super::*;
        use crate::message::example_message::*;

        // ANCHOR: wasm_leptos
        use leptos::*;
        use tokio::task::LocalSet;

        use crate::components::cmp_leptos;
        fn main() -> anyhow::Result<()> {
            #[component]
            fn App() -> impl IntoView {
                view! {}
            }

            // cmp_leptos --------------------------------------------------------------------------
            let config_leptos = cmp_leptos::Config {
                body_component: || view! { <App/> },
                hostname: "localhost".into(),
            };

            // config_executor ---------------------------------------------------------------------
            let config_executor = ComponentExecutorConfig {
                buffer_size: 100,
                executor_name: "example_leptos".into(),
                fn_auth: |msg, _| Some(msg),
            };

            // executor ----------------------------------------------------------------------------

            let context = LocalSet::new();
            context.spawn_local(async move {
                ComponentExecutor::<Custom>::new(config_executor)
                    .add_cmp(cmp_leptos::Cmp::new(config_leptos))
                    .wait_result()
                    .await?;
                Ok(()) as anyhow::Result<()>
            });
            spawn_local(context);
            Ok(())
        }
        // ANCHOR_END: wasm_leptos
    }
}
