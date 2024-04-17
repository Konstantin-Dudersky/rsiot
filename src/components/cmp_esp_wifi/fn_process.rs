use std::time::Duration;

use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    wifi::{BlockingWifi, ClientConfiguration, Configuration, EspWifi},
};
use tokio::time::sleep;
use tracing::info;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(config: Config, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let wifi_config = prepare_wifi_config(&config);

    let mut driver = EspWifi::new(config.peripherals, config.event_loop.clone(), None).unwrap();

    wifi_setup(&mut driver, config.event_loop, wifi_config);

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}

fn prepare_wifi_config(config: &Config) -> Configuration {
    let access_point_config =
        config
            .access_point
            .as_ref()
            .map(|ap| esp_idf_svc::wifi::AccessPointConfiguration {
                ssid: heapless::String::try_from(ap.ssid.as_str()).unwrap(),
                ..Default::default()
            });

    let client_config: Option<ClientConfiguration> =
        config.client.as_ref().map(|cl| ClientConfiguration {
            ssid: heapless::String::try_from(cl.ssid.as_str()).unwrap(),
            password: heapless::String::try_from(cl.password.as_str()).unwrap(),
            auth_method: cl.auth_method,
            ..Default::default()
        });

    if let Some(apc) = access_point_config {
        if let Some(cc) = client_config {
            Configuration::Mixed(cc, apc)
        } else {
            Configuration::AccessPoint(apc)
        }
    } else if let Some(cc) = client_config {
        Configuration::Client(cc)
    } else {
        todo!()
    }
}

/// Запустить Wi-Fi в комбинированном режиме.
///
/// TODO - обработка ошибок
/// TODO - перезапуск
/// TODO - AsyncWifi
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
        info!("Wifi connected to external AP");
        wifi.wait_netif_up().unwrap();
        info!("Wifi netif up");
        let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
        info!("Wifi DHCP info: {:?}", ip_info);
    }
}
