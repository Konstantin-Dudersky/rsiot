use url::Url;

use rsiot_messages_core::{IMessage, IMessageChannel};

#[derive(Clone, Debug)]
pub struct Config<TMessage, TMessageChannel>
where
    TMessage: IMessage,
    TMessageChannel: IMessageChannel,
{
    /// Адрес сервера Redis
    pub url: Url,

    /// Функция определения канала Pub/Sub, в который отсылаются сообщения
    ///
    /// Если все сообщения нужно отправлять только в один канал, то можно задать:
    ///
    /// ```
    /// |_| vec![MessageChannel::Output]
    /// ```
    ///
    /// Возможность рассылки в несколько каналов нужна для организации роутинга сообщений
    pub fn_input: fn(&TMessage) -> Vec<TMessageChannel>,
}
