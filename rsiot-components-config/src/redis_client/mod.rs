use url::Url;

use rsiot_messages_core::IMessageChannel;

#[derive(Clone, Debug)]
pub struct Config<TMessage, TMessageChannel>
where
    TMessageChannel: IMessageChannel,
{
    /// Адрес сервера Redis
    pub url: Url,

    /// Название канала для подписки Pub/Sub и хеша, где хранятся сообщения
    pub subscription_channel: TMessageChannel,

    /// Функция определения канала Pub/Sub, в который отсылаются сообщения
    ///
    /// Если все сообщения нужно отправлять только в один канал, то можно задать:
    ///
    /// ```rust
    /// |_| vec![MessageChannel::Output]
    /// ```
    ///
    /// Возможность рассылки в несколько каналов нужна для организации роутинга сообщений
    pub fn_input: fn(&TMessage) -> Vec<TMessageChannel>,
}
