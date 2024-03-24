use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::modem::Modem};

/// Конфигурация cmp_esp_wifi
pub struct Config {
    /// Модем для подключения
    pub peripherals: Modem,
    /// Ссылка на цикл событий
    pub event_loop: EspSystemEventLoop,
}
