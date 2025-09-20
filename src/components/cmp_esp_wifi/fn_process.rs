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
    message::{Message, MsgDataBound},
};

use super::{Config, Error};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let wifi_config = prepare_wifi_config(&config)?;

    let driver = EspWifi::new(config.peripherals, config.event_loop.clone(), None)
        .map_err(Error::CreateEspWifi)?;

    let mut wifi = AsyncWifi::wrap(driver, config.event_loop, config.timer_service)
        .map_err(Error::CreateAsyncWifi)?;

    let mut state = ConnectionState::PreLaunch;

    loop {
        state = match state {
            ConnectionState::PreLaunch => state_prelaunch(&mut wifi, &wifi_config).await?,
            ConnectionState::Connect => {
                state_connect(&mut wifi, &in_out, config.fn_wifi_connected).await?
            }
            ConnectionState::Check => state_check(&mut wifi).await?,
            ConnectionState::Disconnect => state_disconnect(&mut wifi).await?,
            ConnectionState::OnlyAP => state_onlyap(&in_out, config.fn_wifi_connected).await?,
        };
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

fn prepare_wifi_config<TMsg>(config: &Config<TMsg>) -> Result<Configuration, Error> {
    let access_point_config = match &config.access_point {
        Some(ap) => {
            let ssid = heapless::String::try_from(ap.ssid.as_str())
                .map_err(|_| Error::HeaplessString(ap.ssid.clone()))?;
            let apc = esp_idf_svc::wifi::AccessPointConfiguration {
                ssid,
                ..Default::default()
            };
            Some(apc)
        }
        None => None,
    };

    let client_config = match &config.client {
        Some(c) => {
            let ssid = heapless::String::try_from(c.ssid.as_str())
                .map_err(|_| Error::HeaplessString(c.ssid.clone()))?;
            let password = heapless::String::try_from(c.password.as_str())
                .map_err(|_| Error::HeaplessString(c.password.clone()))?;
            let cc = ClientConfiguration {
                ssid,
                password,
                auth_method: c.auth_method,
                ..Default::default()
            };
            Some(cc)
        }
        None => None,
    };

    let configuration = match (access_point_config, client_config) {
        (Some(apc), Some(cc)) => Configuration::Mixed(cc, apc),
        (Some(apc), None) => Configuration::AccessPoint(apc),
        (None, Some(cc)) => Configuration::Client(cc),
        (None, None) => todo!(),
    };

    Ok(configuration)
}

async fn state_prelaunch<T>(
    wifi: &mut AsyncWifi<T>,
    wifi_config: &Configuration,
) -> Result<ConnectionState, Error>
where
    T: Wifi<Error = EspError> + NonBlocking,
{
    info!("Wifi state: prelaunch");

    wifi.set_configuration(wifi_config)
        .map_err(Error::SetConfiguration)?;
    wifi.start().await.map_err(Error::WiFiStart)?;

    info!("is wifi started: {:?}", wifi.is_started());
    info!("{:?}", wifi.get_capabilities());

    let new_state = if matches!(wifi_config, Configuration::Client(_))
        || matches!(wifi_config, Configuration::Mixed(_, _))
    {
        ConnectionState::Connect
    } else {
        ConnectionState::OnlyAP
    };
    Ok(new_state)
}

async fn state_connect<T, TMsg>(
    wifi: &mut AsyncWifi<T>,
    in_out: &CmpInOut<TMsg>,
    fn_wifi_status: fn(bool) -> TMsg,
) -> Result<ConnectionState, Error>
where
    T: Wifi<Error = EspError> + NonBlocking + NetifStatus,
    TMsg: MsgDataBound,
{
    info!("Wifi state: connect");
    let res = wifi.connect().await;
    if let Err(err) = res {
        warn!("Wifi connect error: {}", err);
        return Ok(ConnectionState::Disconnect);
    }
    info!("Wifi connected to external AP");
    wifi.wait_netif_up().await.map_err(Error::WaitNetifUp)?;
    info!("Wifi netif up");

    wifi_connected(in_out, fn_wifi_status).await?;

    Ok(ConnectionState::Check)
}

async fn state_check<T>(wifi: &mut AsyncWifi<T>) -> Result<ConnectionState, Error>
where
    T: Wifi<Error = EspError> + NonBlocking,
{
    info!("Wifi state: check");

    loop {
        let wifi_connected = wifi.is_connected().map_err(Error::WiFiIsConnected)?;
        if !wifi_connected {
            return Ok(ConnectionState::Disconnect);
        } else {
            sleep(Duration::from_secs(5)).await;
        }
    }
}

async fn state_disconnect<T>(wifi: &mut AsyncWifi<T>) -> Result<ConnectionState, Error>
where
    T: Wifi<Error = EspError> + NonBlocking + NetifStatus,
{
    info!("Wifi state: disconnect");
    wifi.disconnect().await.map_err(Error::WiFiDisconnect)?;
    Ok(ConnectionState::Connect)
}

async fn state_onlyap<TMsg>(
    in_out: &CmpInOut<TMsg>,
    fn_wifi_status: fn(bool) -> TMsg,
) -> Result<ConnectionState, Error>
where
    TMsg: MsgDataBound,
{
    info!("Wifi state: only AP");
    wifi_connected(in_out, fn_wifi_status).await?;
    loop {
        sleep(Duration::from_secs(10)).await
    }
}

async fn wifi_connected<TMsg>(
    in_out: &CmpInOut<TMsg>,
    fn_wifi_status: fn(bool) -> TMsg,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    // Рассылаем сообщение - wifi подключен
    let msg = (fn_wifi_status)(true);
    let msg = Message::new_custom(msg);
    in_out
        .send_output(msg)
        .await
        .map_err(|_| Error::TokioSyncMpscSend)?;

    Ok(())
}
