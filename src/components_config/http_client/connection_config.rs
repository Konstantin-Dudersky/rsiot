//! Настройки подключения к HTTP-серверу

/// Параметры HTTP-сервера, к которому отправляются запросы
#[derive(Clone, Debug)]
pub struct ConnectionConfig {
    /// URL сервера
    pub base_url: String,
}
