/// Сообщения, полученные от MQTT брокера
#[derive(Debug, Clone)]
pub enum MqttMsgRecv {
    /// Клиент подключён
    Connected,

    /// Клиент отключён
    Disconnected,

    /// Получено сообщение
    Received {
        /// Топик
        topic: String,
        /// Данные
        data: Vec<u8>,
    },
}
