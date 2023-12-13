use url::Url;

use rsiot_messages_core::IMessageChannel;

#[derive(Clone, Debug)]
pub struct Config<TMessageChannel>
where
    TMessageChannel: IMessageChannel,
{
    /// Адрес сервера Redis
    pub url: Url,
    /// Название канала Pub/Sub и хеша, где хранятся сообщения
    pub redis_channel: TMessageChannel,
}
