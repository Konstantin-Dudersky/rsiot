use std::{sync::Arc, time::Duration};

use esp_idf_svc::{
    hal::{
        gpio::{Input, InputPin, PinDriver},
        peripherals::Peripherals,
    },
    log::EspLogger,
    sys::link_patches,
};
use tokio::{main, sync::Mutex};

use rsiot::component::{cmp_logger, ComponentChain};
use rsiot_gpio_read_esp::cmp_gpio_read_esp;

use message::Message;
use tracing::Level;

mod hal;
mod message;

#[main(flavor = "current_thread")]
async fn main() {
    link_patches();
    EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let button = PinDriver::input(peripherals.pins.gpio9).unwrap();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "Logger: ".into(),
    };

    let gpio_read_config = cmp_gpio_read_esp::Config {
        period: Duration::from_millis(100),
        fn_output: |_| vec![],
    };

    let mut chain = ComponentChain::<Message>::new(10)
        .add_cmp(cmp_gpio_read_esp::new(gpio_read_config))
        .add_cmp(cmp_logger::create(logger_config));

    chain.spawn().await;
}
