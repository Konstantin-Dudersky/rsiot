use std::time::Duration;

use esp_idf_svc::wifi::EspWifi;
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(mut config: Config, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    wifi_setup(
        &mut config.driver,
        config.event_loop,
        // Configuration::Mixed(
        //     ClientConfiguration {
        //         ssid: "Fermenter".into(),
        //         password: "k33n3+Ik".into(),
        //         auth_method: AuthMethod::None,

        //         ..Default::default()
        //     },
        //     AccessPointConfiguration {
        //         ssid: "test_esp_ap".into(),
        //         ..Default::default()
        //     },
        // ),
        Configuration::AccessPoint(esp_idf_svc::wifi::AccessPointConfiguration {
            ssid: heapless::String::try_from("test_esp").unwrap(),
            ..Default::default()
        }),
    );

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}

use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    wifi::{BlockingWifi, Configuration},
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
