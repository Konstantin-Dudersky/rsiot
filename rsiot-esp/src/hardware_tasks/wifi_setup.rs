//! Функции для запуска wi-fi
//!
//! Пока не использую асинхронные - какая-то структура не поддерживает Send.
//! Не понятно, что делить с перезапуском при обрыве wi-fi.

use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    wifi::{BlockingWifi, Configuration, EspWifi},
};
use tracing::info;

/// Запустить Wi-Fi в комбинированном режиме.
///
/// TODO - обработка ошибок, перезапуск
pub fn wifi_setup(
    wifi: &mut EspWifi<'static>,
    sys_loop: EspEventLoop<System>,
    configuration: Configuration,
) {
    let mut wifi = BlockingWifi::wrap(wifi, sys_loop).unwrap();
    wifi.set_configuration(&configuration).unwrap();
    wifi.start().unwrap();
    info!("is wifi started: {:?}", wifi.is_started());
    info!("{:?}", wifi.get_capabilities());

    // Подключаемся к внешней точке Wi-Fi
    if matches!(configuration, Configuration::Client(_))
        || matches!(configuration, Configuration::Mixed(_, _))
    {
        wifi.connect().unwrap();
        info!("Wifi connected");
        wifi.wait_netif_up().unwrap();
        info!("Wifi netif up");
        let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
        info!("Wifi DHCP info: {:?}", ip_info);
    }
}
