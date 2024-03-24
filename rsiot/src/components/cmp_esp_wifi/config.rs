use esp_idf_svc::{eventloop::EspSystemEventLoop, wifi::EspWifi};

/// Конфигурация cmp_esp_wifi
pub struct Config {
    /// Ссылка на драйвер для подключения
    pub driver: EspWifi<'static>,
    /// Ссылка на цикл событий
    pub event_loop: EspSystemEventLoop,
}
