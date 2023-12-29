use rsiot_extra_components::cmp_cache::CacheType;
use rsiot_messages_core::IMessage;

/// Конфигурация Websocket-сервера
#[derive(Clone, Debug)]
pub struct Config<TMessage>
where
    TMessage: IMessage,
{
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования перед отправкой клиенту
    ///
    /// Если нужно просто переделать в json:
    /// ```
    /// |msg: &TMessage| msg.to_json().ok()
    /// ```
    pub fn_input: fn(&TMessage) -> Option<String>,

    /// Функция преобразования данных, полученных от клиента
    ///
    /// Если нужно просто переделать в json:
    /// ```
    /// |data: &str| TMessage::from_json(data).ok()
    /// ```
    pub fn_output: fn(&str) -> Option<TMessage>,

    /// Кеш
    pub cache: CacheType<TMessage>,
}
