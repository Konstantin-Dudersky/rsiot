use rsiot_extra_components::cmp_cache::CacheType;
use rsiot_messages_core::IMessage;

/// Конфигурация компонента http-server
#[derive(Clone, Debug)]
pub struct Config<TMessage>
where
    TMessage: IMessage,
{
    /// Порт, через который доступен сервер
    pub port: u16,

    pub cache: CacheType<TMessage>,
}
