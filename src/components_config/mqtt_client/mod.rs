//! Настройки коммуникации с MQTT-брокером

mod mqtt_msg_gen;
mod mqtt_msg_recv;
mod mqtt_msg_send;

pub use {mqtt_msg_gen::MqttMsgGen, mqtt_msg_recv::MqttMsgRecv, mqtt_msg_send::MqttMsgSend};

use crate::{message::MsgDataBound, serde_utils::SerdeAlgKind};

/// Преобразование входящих сообщений в данные для публикации в брокере
pub type FnPublish<TMsg> = fn(&TMsg, &MqttMsgGen) -> anyhow::Result<Option<MqttMsgSend>>;

/// Преобразование данных от брокера сообщений в исходящие сообщения
pub type FnSubscribe<TMsg> = fn(&str, &[u8], &MqttMsgGen) -> anyhow::Result<Option<Vec<TMsg>>>;

// ANCHOR: Config
/// Конфигурация cmp_mqtt_client
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlgKind,

    /// Идентификатор клиента MQTT
    pub client_id: String,

    /// Адрес брокера
    ///
    /// Примеры:
    /// - `localhost`
    pub host: String,

    /// Порт брокера.
    ///
    /// По-умолчанию - 1883
    pub port: u16,

    /// Ёмкость клиента.
    ///
    /// Можно задать 100
    pub client_capacity: usize,

    /// Настройка публикации данных в брокере
    pub publish: ConfigPublish<TMsg>,

    /// Настройка подписки на данные из брокера
    pub subscribe: ConfigSubscribe<TMsg>,
}
// ANCHOR: Config

// ANCHOR: ConfigPublish
/// Конфигурация настроек публикации на брокере
#[derive(Clone)]
pub enum ConfigPublish<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Не публиковать
    NoPublish,

    /// Публиковать
    Publish {
        /// Часть топика, которая будет добавлена к каждому сообщению
        base_topic: String,

        /// Функция принимает входящие сообщения и возвращает возможную структуру для публикации в
        /// брокере
        fn_publish: FnPublish<TMsg>,
    },
}
// ANCHOR: ConfigPublish

// ANCHOR: ConfigSubscribe
/// Конфигурация настроек подписки на сообщения из брокера
#[derive(Clone)]
pub enum ConfigSubscribe<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Не подписываться
    NoSubscribe,

    /// Подписаться
    Subscribe {
        /// Токен
        topic: String,

        /// Функция принимает сообщения из брокера и формирует возможный массив исходящих сообщений
        fn_subscribe: FnSubscribe<TMsg>,
    },
}
// ANCHOR: ConfigSubscribe
