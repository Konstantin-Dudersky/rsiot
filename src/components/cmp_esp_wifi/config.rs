pub use esp_idf_svc::wifi::AuthMethod as ConfigAuthMethod;
use esp_idf_svc::{eventloop::EspSystemEventLoop, hal::modem::Modem, timer::EspTaskTimerService};

/// Конфигурация cmp_esp_wifi
pub struct Config {
    /// Модем для подключения
    pub peripherals: Modem,

    /// Ссылка на цикл событий
    pub event_loop: EspSystemEventLoop,

    /// Ссылка на таймер
    pub timer_service: EspTaskTimerService,

    /// Настройка WiFi как точки доступа
    ///
    /// None - точка доступа не настраиваeтся
    pub access_point: Option<ConfigAccessPoint>,

    /// Настройка WiFi как клиента
    ///
    /// None - подключение к внешней точке доступа не настраивается
    pub client: Option<ConfigClient>,
}

/// Настройка WiFi как точки доступа
pub struct ConfigAccessPoint {
    /// Название точки доступа
    pub ssid: String,
}

/// Настройка WiFi как клиента
pub struct ConfigClient {
    /// Название точки доступа
    pub ssid: String,

    /// Пароль точки доступа
    pub password: String,

    /// Защита сети
    pub auth_method: ConfigAuthMethod,
}
