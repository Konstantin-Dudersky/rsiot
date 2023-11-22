/// Конфигурация Websocket-сервера
#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования перед отправкой клиенту
    ///
    /// Если нужно просто переделать в json:
    /// ```
    /// |msg: TMessage| msg.to_json().ok()
    /// ```
    pub fn_send_to_client: fn(TMessage) -> Option<String>,

    /// Функция преобразования данных, полученных от клиента
    ///
    /// Если нужно просто переделать в json:
    /// ```
    /// |data: &str| TMessage::from_json(data).ok()
    /// ```
    pub fn_recv_from_client: fn(&str) -> Option<TMessage>,
}
