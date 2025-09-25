use serde::{Serialize, de::DeserializeOwned};

use crate::serde_utils::{self, SerdeAlg};

use super::MqttMsgSend;

/// Генератор сообщений MQTT-брокера
#[derive(Clone)]
pub struct MqttMsgGen {
    /// Алгоритм сериализации
    pub serde_alg: SerdeAlg,
}

impl MqttMsgGen {
    /// Сериализация сообщений перед отправкой в MQTT-сервер
    pub fn ser<TPayload>(
        &self,
        topic: impl Into<String>,
        retain: bool,
        payload: &TPayload,
    ) -> Result<MqttMsgSend, serde_utils::Error>
    where
        TPayload: Serialize,
    {
        let payload = self.serde_alg.serialize(payload)?;
        let mqtt_msg = MqttMsgSend::Publish {
            topic: topic.into(),
            retain,
            payload,
        };
        Ok(mqtt_msg)
    }

    /// Десериализация сообщения от MQTT-брокера
    pub fn de<TPayload>(&self, payload: &[u8]) -> Result<TPayload, serde_utils::Error>
    where
        TPayload: DeserializeOwned,
    {
        self.serde_alg.deserialize(payload)
    }
}
