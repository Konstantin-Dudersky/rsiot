/// Сообщения, отправляемые MQTT брокеру
#[derive(Debug, Clone)]
pub enum MqttMsgSend {
    /// Публикация сообщения
    Publish {
        /// Топик
        topic: String,
        /// Требуется ли сохранение
        retain: bool,
        /// Данные сообщения
        payload: Vec<u8>,
    },

    /// Подписка на топик
    Subscribe {
        /// Топик
        topic: String,
    },
}
