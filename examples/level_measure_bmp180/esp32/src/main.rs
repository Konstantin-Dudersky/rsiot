use std::time::Duration;

use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{i2c::I2cDriver, peripherals::Peripherals, units::FromValueType},
};
use rsiot::{
    components::{cmp_esp_i2c_master, cmp_esp_wifi, cmp_http_server_esp},
    drivers_i2c,
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::Message,
};
use tokio::task::LocalSet;

use messages::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take().unwrap();
    let event_loop = EspSystemEventLoop::take().unwrap();
    let i2c_config = esp_idf_svc::hal::i2c::config::Config::new().baudrate(100_u32.kHz().into());
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        &i2c_config,
    )
    .unwrap();

    // cmp_esp_i2c_master --------------------------------------------------------------------------
    let i2c_devices = vec![
        drivers_i2c::I2cDevices::BMP180 {
            address: drivers_i2c::I2cSlaveAddress::Mux {
                mux_address: 0x70,
                channel: 0,
                slave_address: 0x77,
            },
            fn_output: |data| vec![Message::new_custom(Custom::Pressure1(data.pressure))],
            oversampling: drivers_i2c::BMP180Oversampling::HighResolution,
        },
        drivers_i2c::I2cDevices::BMP180 {
            address: drivers_i2c::I2cSlaveAddress::Mux {
                mux_address: 0x70,
                channel: 1,
                slave_address: 0x77,
            },
            fn_output: |data| vec![Message::new_custom(Custom::Pressure2(data.pressure))],
            oversampling: drivers_i2c::BMP180Oversampling::HighResolution,
        },
    ];

    let config_esp_i2c_master = cmp_esp_i2c_master::Config {
        timeout: Duration::from_secs(1),
        i2c_driver: i2c,
        devices: i2c_devices,
    };

    // cmp_esp_wifi --------------------------------------------------------------------------------
    let config_esp_wifi = cmp_esp_wifi::Config {
        peripherals: peripherals.modem,
        event_loop,
        access_point: Some(cmp_esp_wifi::ConfigAccessPoint { ssid: "esp".into() }),
        client: None,
    };

    // cmp_http_server_esp -------------------------------------------------------------------------
    let config_http_server = cmp_http_server_esp::Config {
        port: 8000,
        fn_input: |msg: &Message<Custom>| {
            let text = msg.serialize()?;
            Ok(Some(text))
        },
        fn_output: |text: &str| {
            let msg = Message::deserialize(text)?;
            Ok(Some(msg))
        },
    };

    // config_executor -----------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig::<Custom> {
        buffer_size: 10,
        executor_name: "esp32".into(),
        fn_auth: |msg, _| Some(msg),
    };

    // executor ------------------------------------------------------------------------------------
    let local_set = LocalSet::new();
    local_set.spawn_local(async {
        ComponentExecutor::<Custom>::new(config_executor)
            .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
            .add_cmp(cmp_http_server_esp::Cmp::new(config_http_server))
            .add_cmp(cmp_esp_wifi::Cmp::new(config_esp_wifi))
            .wait_result()
            .await
            .unwrap()
    });
    local_set.await;
}
