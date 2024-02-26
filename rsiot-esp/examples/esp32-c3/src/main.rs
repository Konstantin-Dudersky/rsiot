use std::time::Duration;

use esp_idf_svc::{hal::peripherals::Peripherals, log::EspLogger, sys::link_patches};
use serde::{Deserialize, Serialize};
use tokio::{main, task::LocalSet};

use rsiot::{
    component_core::ComponentExecutor,
    components::{cmp_logger, cmp_plc},
    message::*,
};
use rsiot_esp::cmp_storage_esp;

use message::*;
use tracing::Level;

mod cmp_gpio_input;
mod fb_main;
mod hal;
mod message;
mod ws2812rmt;

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
struct StorageData {
    pub test_i32: i32,
}

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb_main::I, msg: &Message<Custom>| match &msg.data {
            MsgType::Custom(msg) => match msg {
                Custom::Button(_) => (),
                // Message::SetLedColor(_) => (),
                Custom::TestFromHttpServer(_) => (),
                Custom::Relay2(_) => (),
                Custom::StorageI32(_) => (),
            },
            MsgType::System(_) => (),
        },
        fn_output: |_output: &fb_main::Q| {
            // let msg1 = Message::SetLedColor(MsgContent::new(output.color));
            // vec![msg1]
            vec![]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
    };

    // let http_config = cmp_http_server_esp::Config {};

    let storage_config = cmp_storage_esp::Config {
        fn_input: |data: &StorageData, msg| match &msg.data {
            MsgType::Custom(msg) => match msg {
                Custom::StorageI32(value) => Some(StorageData {
                    test_i32: *value,
                    ..*data
                }),
                Custom::Button(_) => None,
                // Message::SetLedColor(_) => None,
                Custom::TestFromHttpServer(_) => None,
                Custom::Relay2(_) => None,
            },
            MsgType::System(_) => None,
        },
        fn_output: |data: &StorageData| {
            vec![Message::new_custom(Custom::StorageI32(data.test_i32))]
        },
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        let mut chain = ComponentExecutor::<Custom>::new(10, "example-esp32-c3")
            .add_cmp(cmp_plc::Cmp::new(plc_config))
            .add_cmp(cmp_storage_esp::Cmp::new(storage_config))
            .add_cmp(cmp_logger::Cmp::new(logger_config));
        chain.wait_result().await.unwrap();
    });

    // cmp_external_fn_process::new(hal::Config {}, hal::hal),

    local_set.await;
}
