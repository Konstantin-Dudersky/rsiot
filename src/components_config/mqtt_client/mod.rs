use crate::message::{Message, MsgDataBound};

pub type ConfigFnInput<TMsg> = fn(Message<TMsg>) -> anyhow::Result<Option<Vec<u8>>>;
pub type ConfigFnOutput<TMsg> = fn(&[u8]) -> anyhow::Result<Option<Message<TMsg>>>;

/// Конфигурация cmp_mqtt_client
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
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

    /// Функция преобразования входящих сообщений в данные для публикации в брокере
    ///
    /// **Примеры**
    ///
    /// - ничего не отправлять в брокер
    ///
    /// ```rust
    /// fn_input: |_| Ok(None)
    /// ```
    ///
    /// - сериализация данных без фильтрации
    ///
    /// ```rust
    /// fn_input: |msg| Ok(Some(msg.serialize()?.into_bytes()))
    /// ```
    pub fn_input: ConfigFnInput<TMsg>,

    /// Функция преобразования данных, полученных из брокера, в исходящие сообщения
    ///
    /// **Примеры**
    ///
    /// - ничего не получать из брокера
    ///
    /// ```rust
    /// fn_output: |_| Ok(None)
    /// ```
    ///
    /// - десериализация данных без фильтрации
    ///
    /// ```rust
    /// fn_output: |payload: &[u8]| {
    ///     let payload = String::from_utf8_lossy(&payload);
    ///     let msg = Message::deserialize(&payload)?;
    ///     Ok(Some(msg))
    /// }
    /// ```
    pub fn_output: ConfigFnOutput<TMsg>,
}
