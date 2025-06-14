/// Сообщение для передачи по MQTT
#[derive(Debug)]
pub struct MqttMsg {
    /// Топик
    pub topic: String,
    /// Требуется ли сохранение
    pub retain: bool,
    /// Данные сообщения
    pub payload: Vec<u8>,
}
