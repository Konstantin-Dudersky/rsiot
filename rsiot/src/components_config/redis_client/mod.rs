//! Конфигурация Redis-клиента
//!
//! Тестирование:
//!
//! ```bash
//! cargo test -p rsiot-components-config --doc redis_client
//! ```

use url::Url;

use crate::message::*;

pub type FnInput<TMsg, TMessageChannel> =
    fn(&Message<TMsg>) -> anyhow::Result<Option<Vec<ConfigFnInputItem<TMessageChannel>>>>;
pub type FnOutput<TMsg> = fn(&str) -> anyhow::Result<Option<Vec<Message<TMsg>>>>;

#[derive(Clone, Debug)]
pub struct Config<TMsg, TMessageChannel>
where
    TMessageChannel: IMessageChannel,
{
    /// Адрес сервера Redis
    ///
    /// # Примеры
    ///
    /// ```rust
    /// # use rsiot_components_config::redis_client::Config;
    /// # use rsiot_components_config::redis_client as cmp_redis_client;
    /// # // insert from tests::stub
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # use url::Url;
    /// # Config::<Custom, ExampleMessageChannel> {
    /// url: Url::parse("redis://redis:6379").unwrap(),
    /// #     subscription_channel: ExampleMessageChannel::Output,
    /// #     fn_input: |_| Ok(None),
    /// #     fn_output: |_| Ok(None),
    /// # };
    /// ```
    pub url: Url,

    /// Название канала для подписки Pub/Sub и хеша, где хранятся сообщения
    ///
    /// # Примеры
    ///
    /// ```rust
    /// # use rsiot_components_config::redis_client::Config;
    /// # use rsiot_components_config::redis_client as cmp_redis_client;
    /// # // insert from tests::stub
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # use url::Url;
    /// # Config::<Custom, ExampleMessageChannel> {
    /// #     url: Url::parse("redis://redis:6379").unwrap(),
    /// subscription_channel: ExampleMessageChannel::Output,
    /// #     fn_input: |_| Ok(None),
    /// #     fn_output: |_| Ok(None),
    /// # };
    /// ```
    pub subscription_channel: TMessageChannel,

    /// Функция преобразования входящего потока сообщений в данные для отправки в Redis
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # use rsiot_components_config::redis_client::Config;
    /// # use rsiot_components_config::redis_client as cmp_redis_client;
    /// # // insert from tests::stub
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # use url::Url;
    /// # Config::<Custom, ExampleMessageChannel> {
    /// #     url: Url::parse("redis://redis:6379").unwrap(),
    /// #     subscription_channel: ExampleMessageChannel::Output,
    /// fn_input: |_| Ok(None),
    /// #     fn_output: |_| Ok(None),
    /// # };
    /// ```
    ///
    /// ## Сериализация в json
    ///
    /// ```rust
    /// # use rsiot_components_config::redis_client::Config;
    /// # use rsiot_components_config::redis_client as cmp_redis_client;
    /// # // insert from tests::fn_input_json
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # use url::Url;
    /// # Config::<Custom, ExampleMessageChannel> {
    /// #     url: Url::parse("redis://redis:6379").unwrap(),
    /// #     subscription_channel: ExampleMessageChannel::Output,
    /// fn_input: |msg: &Message<Custom>| {
    ///     let channel = ExampleMessageChannel::Output;
    ///     let key = msg.key.clone();
    ///     let value = msg.serialize()?;
    ///     Ok(Some(vec![cmp_redis_client::ConfigFnInputItem {
    ///         channel,
    ///         key,
    ///         value,
    ///     }]))
    /// },
    /// #     fn_output: |_| Ok(None),
    /// # };
    /// ```
    ///
    /// Возможность рассылки в несколько каналов нужна для организации роутинга сообщений
    pub fn_input: FnInput<TMsg, TMessageChannel>,

    /// Функция преобразования данных из Redis в исходящий поток сообщений
    ///
    /// # Примеры
    ///
    /// ## Заглушка
    ///
    /// ```rust
    /// # use rsiot_components_config::redis_client::Config;
    /// # use rsiot_components_config::redis_client as cmp_redis_client;
    /// # // insert from tests::stub
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # use url::Url;
    /// # Config::<Custom, ExampleMessageChannel> {
    /// #     url: Url::parse("redis://redis:6379").unwrap(),
    /// #     subscription_channel: ExampleMessageChannel::Output,
    /// #     fn_input: |_| Ok(None),
    /// fn_output: |_| Ok(None),
    /// # };
    /// ```
    ///
    /// ## Десериализация из json
    ///
    /// ```rust
    /// # use rsiot_components_config::redis_client::Config;
    /// # use rsiot_components_config::redis_client as cmp_redis_client;
    /// # // insert from tests::fn_output_json
    /// # use rsiot_messages_core::{example_message::*, *};
    /// # use url::Url;
    /// # Config::<Custom, ExampleMessageChannel> {
    /// #     url: Url::parse("redis://redis:6379").unwrap(),
    /// #     subscription_channel: ExampleMessageChannel::Output,
    /// #     fn_input: |_| Ok(None),
    /// fn_output: |text: &str| {
    ///     let msg = Message::deserialize(text)?;
    ///     Ok(Some(vec![msg]))
    /// },
    /// # };
    /// ```
    pub fn_output: FnOutput<TMsg>,
}

pub struct ConfigFnInputItem<TMessageChannel>
where
    TMessageChannel: IMessageChannel,
{
    pub channel: TMessageChannel,
    pub key: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    // use super::*;
    use super::super::redis_client as cmp_redis_client;
    use super::Config;

    #[test]
    pub fn stub() {
        use crate::message::{example_message::*, *};
        use url::Url;
        let _ = Config::<Custom, ExampleMessageChannel> {
            url: Url::parse("redis://redis:6379").unwrap(),
            subscription_channel: ExampleMessageChannel::Output,
            fn_input: |_| Ok(None),
            fn_output: |_| Ok(None),
        };
    }

    #[test]
    pub fn fn_input_json() {
        use crate::message::{example_message::*, *};
        use url::Url;
        let _ = Config::<Custom, ExampleMessageChannel> {
            url: Url::parse("redis://redis:6379").unwrap(),
            subscription_channel: ExampleMessageChannel::Output,
            fn_input: |msg: &Message<Custom>| {
                let channel = ExampleMessageChannel::Output;
                let key = msg.key.clone();
                let value = msg.serialize()?;
                Ok(Some(vec![cmp_redis_client::ConfigFnInputItem {
                    channel,
                    key,
                    value,
                }]))
            },
            fn_output: |_| Ok(None),
        };
    }

    #[test]
    pub fn fn_output_json() {
        use crate::message::{example_message::*, *};
        use url::Url;
        let _ = Config::<Custom, ExampleMessageChannel> {
            url: Url::parse("redis://redis:6379").unwrap(),
            subscription_channel: ExampleMessageChannel::Output,
            fn_input: |_| Ok(None),
            fn_output: |text: &str| {
                let msg = Message::deserialize(text)?;
                Ok(Some(vec![msg]))
            },
        };
    }
}
