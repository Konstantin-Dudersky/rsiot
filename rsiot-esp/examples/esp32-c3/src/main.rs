use std::time::Duration;

use esp_idf_svc::{log::EspLogger, sys::link_patches};
use serde::{Deserialize, Serialize};
use tokio::{main, task::LocalSet};

use rsiot::{
    component_core::ComponentExecutor,
    components::{cmp_logger, cmp_plc},
    message::MsgContent,
};
use rsiot_esp::cmp_storage_esp;

use message::Message;
use tracing::Level;

mod fb_main;
// mod hal;
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
        fn_input: |_input: &mut fb_main::I, msg: &Message| match msg {
            Message::Button(_) => (),
            // Message::SetLedColor(_) => (),
            Message::TestFromHttpServer(_) => (),
            Message::Relay2(_) => (),
            Message::StorageI32(_) => (),
        },
        fn_output: |output: &fb_main::Q| {
            // let msg1 = Message::SetLedColor(MsgContent::new(output.color));
            // vec![msg1]
            vec![]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
    };

    // let http_config = cmp_http_server_esp::Config {};

    let storage_config = cmp_storage_esp::Config {
        fn_input: |data: &StorageData, msg| match msg {
            Message::StorageI32(value) => Some(StorageData {
                test_i32: value.value,
                ..*data
            }),
            Message::Button(_) => None,
            // Message::SetLedColor(_) => None,
            Message::TestFromHttpServer(_) => None,
            Message::Relay2(_) => None,
        },
        fn_output: |data: &StorageData| vec![Message::StorageI32(MsgContent::new(data.test_i32))],
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        let mut chain = ComponentExecutor::<Message>::new(10)
            .add_cmp(cmp_plc::Cmp::new(plc_config))
            .add_cmp(cmp_storage_esp::Cmp::new(storage_config))
            .add_cmp(cmp_logger::Cmp::new(logger_config));
        chain.wait_result().await.unwrap();
    });

    // cmp_external_fn_process::new(hal::Config {}, hal::hal),

    local_set.await;
}
