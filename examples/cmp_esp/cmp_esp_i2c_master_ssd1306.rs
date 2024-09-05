//! Пример работы с экраном SSD1306 по I2C
//!
//! ```bash
//! cargo run --example cmp_esp_i2c_master_ssd1306 --target="riscv32imc-esp-espidf" --features="cmp_esp, logging" --release
//! ```

// #[cfg(not(feature = "cmp_esp"))]
#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    use std::sync::Arc;
    use std::time::Duration;

    use esp_idf_svc::{hal::peripherals::Peripherals, sys::link_patches};
    use tokio::{sync::Mutex, task::LocalSet, time::sleep};
    use tracing::{level_filters::LevelFilter, Level};

    use esp_idf_hal::i2c::*;
    use esp_idf_hal::prelude::*;
    use rsiot::{
        components::{cmp_esp_i2c_master, cmp_logger},
        drivers_i2c,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::configure_logging,
        message::*,
    };

    link_patches();
    configure_logging(LevelFilter::INFO).unwrap();

    // service -------------------------------------------------------------------------------------
    #[allow(non_camel_case_types)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Service {
        cmp_esp_example,
    }

    impl ServiceBound for Service {}

    // message -------------------------------------------------------------------------------------
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    pub enum Custom {
        VoltageA0(f64),
    }

    impl MsgDataBound for Custom {
        type TService = Service;

        fn define_enabled_routes(&self) -> Vec<(Option<Self::TService>, Option<Self::TService>)> {
            vec![]
        }

        fn define_time_to_live(&self) -> rsiot::message::TimeToLiveValue {
            TimeToLiveValue::Infinite
        }
    }

    // cmp_logger ----------------------------------------------------------------------------------
    let logger_config = cmp_logger::Config::<Custom> {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    use embedded_graphics::{
        mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Baseline, Text},
    };
    use ssd1306::{
        mode::BufferedGraphicsMode, prelude::*, size::DisplaySize64x32, I2CDisplayInterface,
        Ssd1306,
    };

    let peripherals = Peripherals::take().unwrap();
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio1;

    println!("Starting I2C SSD1306 test");

    let config = I2cConfig::new()
        .baudrate(100.kHz().into())
        .sda_enable_pullup(true)
        .scl_enable_pullup(true);
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config).unwrap();

    let i2c = Arc::new(Mutex::new(i2c));
    let i2c_clone = i2c.clone();

    loop {
        let mut lock = i2c.lock().await;
        let interface = I2CDisplayInterface::new(lock);
        let mut display = Ssd1306::new(interface, DisplaySize64x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        drop(lock);
        sleep(Duration::from_secs(5)).await;
    }

    // executor ------------------------------------------------------------------------------------

    // let executor_config = ComponentExecutorConfig {
    //     buffer_size: 10,
    //     service: Service::cmp_esp_example,
    //     fn_auth: |msg, _| Some(msg),
    // };

    // let local_set = LocalSet::new();

    // local_set.spawn_local(async {
    //     ComponentExecutor::<Custom>::new(executor_config)
    //         .add_cmp(cmp_logger::Cmp::new(logger_config))
    //         .add_cmp(cmp_esp_i2c_master::Cmp::new(config_esp_i2c_master))
    //         .wait_result()
    //         .await
    //         .unwrap()
    // });
    // local_set.await;
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {}

#[allow(unused)]
#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main1() -> anyhow::Result<()> {
    use std::time::Duration;

    use esp_idf_hal::delay::{FreeRtos, BLOCK};
    use esp_idf_hal::i2c::*;
    use esp_idf_hal::prelude::*;
    use tokio::time::sleep;

    const SSD1306_ADDRESS: u8 = 0x3C;

    esp_idf_hal::sys::link_patches();

    let peripherals = Peripherals::take()?;
    let i2c = peripherals.i2c0;
    let sda = peripherals.pins.gpio0;
    let scl = peripherals.pins.gpio1;

    println!("Starting I2C SSD1306 test");

    let config = I2cConfig::new()
        .baudrate(100.kHz().into())
        .sda_enable_pullup(true)
        .scl_enable_pullup(true);
    let mut i2c = I2cDriver::new(i2c, sda, scl, &config)?;

    // initialze the display - don't worry about the meaning of these bytes - it's specific to SSD1306
    i2c.write(SSD1306_ADDRESS, &[0, 0xae], BLOCK)?;

    loop {
        println!("Test");
        sleep(Duration::from_secs(1)).await;
    }
    println!("Send 0xae");
    i2c.write(SSD1306_ADDRESS, &[0, 0xd4], BLOCK)?;
    println!("Send 0xd4");
    i2c.write(SSD1306_ADDRESS, &[0, 0x80], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xa8], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x3f], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xd3], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x00], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x40], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x8d], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x14], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xa1], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xc8], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xda], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x12], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x81], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xcf], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xf1], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xdb], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x40], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xa4], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xa6], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0xaf], BLOCK)?;
    i2c.write(SSD1306_ADDRESS, &[0, 0x20, 0x00], BLOCK)?;

    // fill the display
    for _ in 0..64 {
        let data: [u8; 17] = [
            0x40, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff,
        ];
        i2c.write(SSD1306_ADDRESS, &data, BLOCK)?;
    }

    loop {
        // we are sleeping here to make sure the watchdog isn't triggered
        FreeRtos::delay_ms(500);
        i2c.write(SSD1306_ADDRESS, &[0, 0xa6], BLOCK)?;
        println!("Send 0xa6");

        FreeRtos::delay_ms(500);
        i2c.write(SSD1306_ADDRESS, &[0, 0xa7], BLOCK)?;
        println!("Send 0xa7");
    }
}
