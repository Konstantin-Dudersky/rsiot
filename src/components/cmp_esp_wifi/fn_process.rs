use std::{sync::Arc, time::Duration};

use embedded_svc::wifi::Wifi;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::{
    netif::NetifStatus,
    wifi::{AsyncWifi, ClientConfiguration, Configuration, EspWifi, NonBlocking},
};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};
use tracing::info;

use crate::{
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::Config;

pub async fn fn_process<TMsg>(config: Config, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut task_set = JoinSet::new();

    let wifi_config = prepare_wifi_config(&config);

    let driver = EspWifi::new(config.peripherals, config.event_loop.clone(), None).unwrap();

    let wifi = AsyncWifi::wrap(driver, config.event_loop, config.timer_service).unwrap();
    let wifi = Arc::new(Mutex::new(wifi));

    join_set_spawn(&mut task_set, test_connection(wifi.clone()));

    join_set_spawn(&mut task_set, wifi_setup(wifi, wifi_config));

    while let Some(res) = task_set.join_next().await {
        res.unwrap()
    }

    Ok(())
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
pub async fn wifi_setup<T>(
    // wifi: &mut EspWifi<'static>,
    // sys_loop: EspEventLoop<System>,
    // timer_service: EspTaskTimerService,
    wifi: Arc<Mutex<AsyncWifi<T>>>,
    configuration: Configuration,
) where
    T: Wifi<Error = EspError> + NonBlocking + NetifStatus,
{
    let mut wifi = wifi.lock().await;
    // let mut wifi = BlockingWifi::wrap(wifi, sys_loop).unwrap();
    wifi.set_configuration(&configuration).unwrap();
    wifi.start().await.unwrap();
    info!("is wifi started: {:?}", wifi.is_started());
    info!("{:?}", wifi.get_capabilities());

    // Подключаемся к внешней точке Wi-Fi
    if matches!(configuration, Configuration::Client(_))
        || matches!(configuration, Configuration::Mixed(_, _))
    {
        wifi.connect().await.unwrap();
        info!("Wifi connected to external AP");
        wifi.wait_netif_up().await.unwrap();
        info!("Wifi netif up");
        // let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
        // info!("Wifi DHCP info: {:?}", ip_info);
    }
}

async fn test_connection<T>(wifi: Arc<Mutex<AsyncWifi<T>>>)
where
    T: Wifi<Error = EspError> + NonBlocking + NetifStatus,
{
    loop {
        let wifi = wifi.lock().await;
        let wifi_connected = wifi.is_connected().unwrap();
        info!("Wifi connected: {}", wifi_connected);
        sleep(Duration::from_secs(2)).await
    }
}
