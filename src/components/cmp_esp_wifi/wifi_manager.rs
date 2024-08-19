use esp_idf_svc::{
    netif::NetifStatus,
    wifi::{AsyncWifi, ClientConfiguration, Configuration, EspWifi, NonBlocking},
};

use crate::{
    executor::CmpInOut,
    message::{system_messages::System, Message, MsgData, MsgDataBound},
};

use super::Config;

pub struct WifiManager<TMsg> {
    instance: WifiInstance,
    configuration: Configuration,
    in_out: CmpInOut<TMsg>,
}

impl<TMsg> WifiManager<TMsg> {
    pub fn new_nonlocking(config: Config, in_out: CmpInOut<TMsg>) -> Self {
        let configuration = prepare_wifi_config(&config);

        let driver = EspWifi::new(config.peripherals, config.event_loop.clone(), None).unwrap();

        let mut wifi = AsyncWifi::wrap(driver, config.event_loop, config.timer_service).unwrap();

        Self {
            instance: kind,
            configuration,
            in_out,
        }
    }
}

enum WifiInstance {
    Blocking,
    Nonblocking(AsyncWifi<T>),
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
