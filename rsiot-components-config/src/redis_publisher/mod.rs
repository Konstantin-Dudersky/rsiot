use url::Url;

#[derive(Clone, Debug)]
pub struct Config {
    /// Адрес сервера Redis
    pub url: Url,
    /// Название канала Pub/Sub и хеша, где хранятся сообщения
    pub redis_channel: String,
}
