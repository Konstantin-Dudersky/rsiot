use std::time::Duration;

use esp_idf_svc::{log::EspLogger, sys::link_patches};
use tokio::{main, spawn, sync::mpsc};

use rsiot::{
    cmp_plc,
    component::{
        cmp_add_input_stream, cmp_add_output_stream, cmp_delay, cmp_logger, ComponentChain,
    },
    message::msg_types::Value,
};

use message::Message;
use rgb::RGB8;
use tracing::Level;

mod fb_main;
mod hal;
mod message;
mod ws2812rmt;

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let (input_hal_tx, input_hal_rx) = mpsc::channel(10);
    let (output_hal_tx, output_hal_rx) = mpsc::channel(10);

    spawn(hal::hal(Some(input_hal_rx), Some(output_hal_tx)));

    input_hal_tx
        .send(Message::SetLedColor(Value::new(RGB8::new(128, 128, 128))))
        .await
        .unwrap();

    let output_hal_config = cmp_add_input_stream::Config {
        channel: output_hal_rx,
    };

    let input_hal_config = cmp_add_output_stream::Config {
        channel: input_hal_tx,
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb_main::I, msg: &Message| match msg {
            Message::Button(_) => (),
            Message::SetLedColor(_) => (),
        },
        fn_output: |output: &fb_main::Q| {
            let msg1 = Message::SetLedColor(Value::new(output.color));
            vec![msg1]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(100),
        buffer_size: 10,
    };

    let delay_config = cmp_delay::Config {
        delay: Duration::from_secs(2),
    };

    let mut chain = ComponentChain::<Message>::new(10)
        .add_cmp(cmp_add_input_stream::new(output_hal_config))
        .add_cmp(cmp_plc::new(plc_config))
        .add_cmp(cmp_add_output_stream::new(input_hal_config))
        .add_cmp(cmp_delay::new(delay_config))
        .add_cmp(cmp_logger::new(logger_config));

    chain.spawn().await;
}
