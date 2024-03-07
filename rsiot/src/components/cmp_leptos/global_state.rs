use leptos::*;

use rsiot_component_core::Cache;
use rsiot_messages_core::*;

#[derive(Clone)]
pub struct GlobalState<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    /// hostname
    pub hostname: String,

    /// Сигнал для входных сообщений
    pub input: RwSignal<Option<Message<TMsg>>>,

    /// Сигнал для выходных сообщений
    pub output: RwSignal<Option<Message<TMsg>>>,

    /// Кеш сообщений
    pub cache: Cache<TMsg>,

    /// Разрешения
    pub auth_perm: RwSignal<AuthPermissions>,
}
