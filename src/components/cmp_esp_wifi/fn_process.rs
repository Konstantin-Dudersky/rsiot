use std::time::Duration;

use embedded_svc::wifi::Wifi;
use esp_idf_svc::hal::sys::EspError;
use esp_idf_svc::{
    netif::NetifStatus,
    wifi::{AsyncWifi, ClientConfiguration, Configuration, EspWifi, NonBlocking},
};
use tokio::time::sleep;
use tracing::{info, warn};

use crate::{
    executor::CmpInOut,
    message::{system_messages, Message, MsgData, MsgDataBound, ServiceBound},
};

use super::Config;

pub async fn fn_process<TMsg, TService>(
    config: Config,
    in_out: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let wifi_config = prepare_wifi_config(&config);

    let driver = EspWifi::new(config.peripherals, config.event_loop.clone(), None).unwrap();

    let mut wifi = AsyncWifi::wrap(driver, config.event_loop, config.timer_service).unwrap();

    let mut state = ConnectionState::PreLaunch;

    loop {
        state = match state {
            ConnectionState::PreLaunch => state_prelaunch(&mut wifi, &wifi_config).await,
            ConnectionState::Connect => state_connect(&mut wifi, &in_out).await,
            ConnectionState::Check => state_check(&mut wifi).await,
            ConnectionState::Disconnect => state_disconnect(&mut wifi).await,
            ConnectionState::OnlyAP => state_onlyap(&in_out).await,
        };
    }
}

// pub fn wifi_setup(
//     wifi: &mut EspWifi<'static>,
//     sys_loop: EspEventLoop<System>,
//     configuration: Configuration,
// ) {
//     let mut wifi = BlockingWifi::wrap(wifi, sys_loop).unwrap();
//     wifi.set_configuration(&configuration).unwrap();
//     wifi.start().unwrap();
//     info!("is wifi started: {:?}", wifi.is_started());
//     info!("{:?}", wifi.get_capabilities());

//     // Подключаемся к внешней точке Wi-Fi
//     if matches!(configuration, Configuration::Client(_))
//         || matches!(configuration, Configuration::Mixed(_, _))
//     {
//         wifi.connect().unwrap();

//         info!("Wifi connected to external AP");

//         wifi.wait_netif_up().unwrap();
//         info!("Wifi netif up");
//         let ip_info = wifi.wifi().sta_netif().get_ip_info().unwrap();
//         info!("Wifi DHCP info: {:?}", ip_info);
//     }
// }

// async fn start_wifi<TMsg>(config: Config, in_out: CmpInOut<TMsg>)
// where
//     TMsg: MsgDataBound,
// {
//     let wifi_config = prepare_wifi_config(&config);

//     let driver = EspWifi::new(config.peripherals, config.event_loop.clone(), None).unwrap();

//     let mut wifi = AsyncWifi::wrap(driver, config.event_loop, config.timer_service).unwrap();

//     let mut state = ConnectionState::PreLaunch;

//     loop {
//         state = match state {
//             ConnectionState::PreLaunch => state_prelaunch(&mut wifi, &wifi_config).await,
//             ConnectionState::Connect => state_connect(&mut wifi, &in_out).await,
//             ConnectionState::Check => state_check(&mut wifi).await,
//             ConnectionState::Disconnect => state_disconnect(&mut wifi).await,
//             // ConnectionState::OnlyAP => state_onlyap(&in_out).await,
//             ConnectionState::OnlyAP => break,
//         };
//     }

//     wifi_connected(&in_out).await.unwrap();
// }

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

async fn state_prelaunch<T>(wifi: &mut AsyncWifi<T>, wifi_config: &Configuration) -> ConnectionState
where
    T: Wifi<Error = EspError> + NonBlocking,
{
    info!("Wifi state: prelaunch");
    wifi.set_configuration(wifi_config).unwrap();
    wifi.start().await.unwrap();
    info!("is wifi started: {:?}", wifi.is_started());
    info!("{:?}", wifi.get_capabilities());

    if matches!(wifi_config, Configuration::Client(_))
        || matches!(wifi_config, Configuration::Mixed(_, _))
    {
        ConnectionState::Connect
    } else {
        ConnectionState::OnlyAP
    }
}

async fn state_connect<T, TMsg, TService>(
    wifi: &mut AsyncWifi<T>,
    in_out: &CmpInOut<TMsg, TService>,
) -> ConnectionState
where
    T: Wifi<Error = EspError> + NonBlocking + NetifStatus,
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    info!("Wifi state: connect");
    let res = wifi.connect().await;
    if let Err(err) = res {
        warn!("Wifi connect error: {}", err);
        return ConnectionState::Disconnect;
    }
    info!("Wifi connected to external AP");
    wifi.wait_netif_up().await.unwrap();
    info!("Wifi netif up");

    wifi_connected(in_out).await.unwrap();

    ConnectionState::Check
}

async fn state_check<T>(wifi: &mut AsyncWifi<T>) -> ConnectionState
where
    T: Wifi<Error = EspError> + NonBlocking,
{
    info!("Wifi state: check");

    loop {
        let wifi_connected = wifi.is_connected().unwrap();
        if !wifi_connected {
            return ConnectionState::Disconnect;
        } else {
            sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn state_disconnect<T>(wifi: &mut AsyncWifi<T>) -> ConnectionState
where
    T: Wifi<Error = EspError> + NonBlocking + NetifStatus,
{
    info!("Wifi state: disconnect");
    wifi.disconnect().await.unwrap();
    ConnectionState::Connect
}

async fn state_onlyap<TMsg, TService>(in_out: &CmpInOut<TMsg, TService>) -> ConnectionState
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    info!("Wifi state: only AP");
    wifi_connected(in_out).await.unwrap();
    loop {
        sleep(Duration::from_secs(10)).await
    }
}

/// Состояние соединения
enum ConnectionState {
    /// Подготовка. Запускает точку доступа, если настроена
    PreLaunch,
    /// Подключение к внешней точке доступа
    Connect,
    /// Проверка соединения
    Check,
    /// Отключение
    Disconnect,
    /// Настроен режим только точки доступа
    OnlyAP,
}

async fn wifi_connected<TMsg, TService>(in_out: &CmpInOut<TMsg, TService>) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    // Рассылаем сообщение - wifi подключен
    let msg = Message::new(MsgData::System(system_messages::System::EspWifiConnected));
    in_out.send_output(msg).await.unwrap();

    Ok(())
}
