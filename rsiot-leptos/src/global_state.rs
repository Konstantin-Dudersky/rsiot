use leptos::*;
use rsiot_component_core::Cache;
use rsiot_messages_core::{msg_meta::ExecutorId, IMessage};

#[derive(Clone)]
pub struct GlobalState<TMsg>
where
    TMsg: IMessage + 'static,
{
    /// Идентификатор клиента
    pub service_id: ExecutorId,

    /// hostname
    pub hostname: String,

    /// Сигнал для входных сообщений
    pub input: RwSignal<Option<TMsg>>,

    /// Сигнал для выходных сообщений
    pub output: RwSignal<Option<TMsg>>,

    /// Кеш сообщений
    pub cache: Cache<TMsg>,
}
