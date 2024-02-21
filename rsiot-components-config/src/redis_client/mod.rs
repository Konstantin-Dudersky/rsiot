use url::Url;

use rsiot_messages_core::{message_v2::Message, IMessageChannel};

#[derive(Clone, Debug)]
pub struct Config<TMessage, TMessageChannel>
where
    TMessageChannel: IMessageChannel,
{
    /// Адрес сервера Redis
    pub url: Url,

    /// Название канала для подписки Pub/Sub и хеша, где хранятся сообщения
    pub subscription_channel: TMessageChannel,

    /// Функция преобразования входящего потока сообщений в данные для отправки в Redis
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// |_| Ok(None)
    /// ```
    ///
    /// ## Сериализация в json
    ///
    /// ```rust
    /// |msg: &Message<ExampleMessage>| {
    ///     let channel = ExampleMessageChannel::Output;
    ///     let key = msg.key.clone();
    ///     let value = msg.serialize()?;
    ///     Ok(Some(vec![cmp_redis_client::ConfigFnInputItem {
    ///         channel,
    ///         key,
    ///         value,
    ///     }]))
    /// }
    /// ```
    ///
    /// Возможность рассылки в несколько каналов нужна для организации роутинга сообщений
    pub fn_input:
        fn(&Message<TMessage>) -> anyhow::Result<Option<Vec<ConfigFnInputItem<TMessageChannel>>>>,

    /// Функция преобразования данных из Redis в исходящий поток сообщений
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// |_| Ok(None)
    /// ```
    ///
    /// ## Десериализация из json
    ///
    /// ```rust
    /// |text: &str| {
    ///     let msg = Message::deserialize(text)?;
    ///     Ok(Some(vec![msg]))
    /// },
    /// ```
    pub fn_output: fn(&str) -> anyhow::Result<Option<Vec<Message<TMessage>>>>,
}

pub struct ConfigFnInputItem<TMessageChannel>
where
    TMessageChannel: IMessageChannel,
{
    pub channel: TMessageChannel,
    pub key: String,
    pub value: String,
}
