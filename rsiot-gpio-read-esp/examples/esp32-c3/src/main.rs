use esp_idf_svc::{log::EspLogger, sys::link_patches};
use tokio::{main, spawn, sync::mpsc};

use rsiot::component::{cmp_add_input_stream, cmp_logger, ComponentChain};

use message::Message;
use tracing::Level;

mod hal;
mod message;

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let (output_hal_tx, output_hal_rx) = mpsc::channel(10);

    spawn(hal::hal(None, Some(output_hal_tx)));

    let output_hal_config = cmp_add_input_stream::Config {
        channel: output_hal_rx,
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let mut chain = ComponentChain::<Message>::new(10)
        .add_cmp(cmp_add_input_stream::new(output_hal_config))
        .add_cmp(cmp_logger::new(logger_config));

    chain.spawn().await;
}
