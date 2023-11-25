//! Настройки подключения к HTTP-серверу

use url::Url;

pub struct ConnectionConfig {
    pub url: Url,
}
