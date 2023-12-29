use std::time::Duration;

use esp_idf_svc::{log::EspLogger, sys::link_patches};
use tokio::main;

use rsiot::{
    cmp_plc,
    component::{cmp_cache, cmp_external_fn_process, cmp_logger, ComponentCollection},
    message::msg_types::Value,
};
use rsiot_esp::cmp_http_server_esp;

use message::Message;
use tracing::Level;

mod fb_main;
mod hal;
mod message;
mod ws2812rmt;

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let cache = cmp_cache::create_cache();

    let _logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb_main::I, msg: &Message| match msg {
            Message::Button(_) => (),
            Message::SetLedColor(_) => (),
            Message::TestFromHttpServer(_) => (),
            Message::Relay2(_) => (),
        },
        fn_output: |output: &fb_main::Q| {
            let msg1 = Message::SetLedColor(Value::new(output.color));
            vec![msg1]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
        buffer_size: 10,
    };

    let cache_config = cmp_cache::Config {
        cache: cache.clone(),
    };

    let http_config = cmp_http_server_esp::Config {
        cache: cache.clone(),
    };

    let mut chain = ComponentCollection::<Message>::new(
        10,
        vec![
            cmp_plc::new(plc_config),
            // cmp_logger::new(logger_config),
            cmp_cache::new(cache_config),
            cmp_http_server_esp::new(http_config),
            cmp_external_fn_process::new(hal::Config {}, hal::hal),
        ],
    );

    chain.spawn().await;
}
