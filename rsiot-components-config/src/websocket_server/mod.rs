/// Конфигурация Websocket-сервера
#[derive(Clone, Debug)]
pub struct Config<TMessage> {
    /// Порт, через который доступен сервер
    pub port: u16,

    /// Функция преобразования перед отправкой клиенту
    ///
    /// Если нужно просто переделать в json:
    /// ```
    /// |msg: &TMessage| {
    ///     let text = msg.to_json()?;
    ///     Ok(Some(text))
    /// }
    /// ```
    pub fn_input: fn(&TMessage) -> anyhow::Result<Option<String>>,

    /// Функция преобразования данных, полученных от клиента
    ///
    /// Если нужно просто переделать из json:
    /// ```
    /// |data: &str| {
    ///     let msg = TMessage::from_json(data)?;
    ///     Ok(Some(msg))
    /// }
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Option<TMessage>>,
}
