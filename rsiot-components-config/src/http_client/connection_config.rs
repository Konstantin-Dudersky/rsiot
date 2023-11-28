//! Настройки подключения к HTTP-серверу

use url::Url;

/// Параметры HTTP-сервера, к которому отправляются запросы
#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    pub base_url: Url,
}
