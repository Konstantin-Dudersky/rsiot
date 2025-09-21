use esp_idf_svc::{
    eventloop::{EspEventLoop, System},
    hal::modem::Modem,
    timer::{EspTimerService, Task},
};
use rsiot::{components::cmp_esp_wifi::*, executor::Component};

use crate::message::*;

pub fn cmp(
    modem: Modem,
    event_loop: EspEventLoop<System>,
    timer_service: EspTimerService<Task>,
) -> Component<Config<Msg>, Msg> {
    let config = Config {
        peripherals: modem,
        event_loop,
        timer_service,
        access_point: Some(ConfigAccessPoint {
            ssid: "test_esp".into(),
        }),
        // client: None,
        client: Some(ConfigClient {
            ssid: "internet".into(),
            password: "k33n3+Ik".into(),
            auth_method: ConfigAuthMethod::WPA,
        }),
        fn_wifi_connected: |v| Msg::WifiConnected(v),
    };

    Cmp::new(config)
}
