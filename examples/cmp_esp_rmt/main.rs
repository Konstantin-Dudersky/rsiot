#[cfg(feature = "cmp_esp")]
mod message;

#[cfg(feature = "cmp_esp")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use esp_idf_svc::hal::adc::attenuation::DB_11;
    use esp_idf_svc::hal::adc::oneshot::config::AdcChannelConfig;
    use esp_idf_svc::hal::adc::oneshot::{AdcChannelDriver, AdcDriver};
    use esp_idf_svc::hal::gpio::PinDriver;
    use esp_idf_svc::hal::spi::{self, SpiDeviceDriver, SpiDriver, SpiDriverConfig};
    use esp_idf_svc::hal::units::*;
    use esp_idf_svc::{hal::prelude::Peripherals, sys::link_patches};

    use tokio::sync::mpsc;

    use embedded_graphics::{
        mono_font::{MonoTextStyleBuilder, ascii::FONT_10X20},
        pixelcolor::BinaryColor,
        prelude::*,
        text::{Baseline, Text},
    };
    use rsiot::{
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::LogConfig,
    };
    use ssd1306::Ssd1306;
    use ssd1306::prelude::*;
    use ssd1306::prelude::{DisplayRotation, SPIInterface};
    use ssd1306::size::DisplaySize128x64;
    use tokio::{task::LocalSet, time::sleep};
    use tracing::info;

    use message::*;
    use tracing::level_filters::LevelFilter;

    // ESP
    link_patches();

    LogConfig {
        esp_filter_level: LevelFilter::INFO,
    }
    .run()
    .unwrap();

    let peripherals = Peripherals::take()?;

    let pin_generator = peripherals.pins.gpio7;
    let pin_sck = peripherals.pins.gpio9;
    let pin_mosi = peripherals.pins.gpio8;
    let pin_analog = peripherals.pins.gpio1;
    let pin_dc = peripherals.pins.gpio4;
    let pin_cs = peripherals.pins.gpio5;
    let pin_reset = peripherals.pins.gpio6;
    let pin_miso = peripherals.pins.gpio20;

    // let mut channel = LedcDriver::new(
    //     peripherals.ledc.channel0,
    //     LedcTimerDriver::new(
    //         peripherals.ledc.timer0,
    //         &config::TimerConfig::new()
    //             .frequency(500.kHz().into())
    //             .resolution(config::Resolution::Bits6),
    //     )?,
    //     pin_generator,
    // )?;
    // channel.set_duty(32)?;

    let local_set = LocalSet::new();

    let (ch_tx, mut ch_rx) = mpsc::channel::<u16>(10);

    local_set.spawn_local(async move {
        let adc = AdcDriver::new(peripherals.adc1).unwrap();
        let config = AdcChannelConfig {
            attenuation: DB_11,
            ..Default::default()
        };

        let mut adc_pin = AdcChannelDriver::new(&adc, pin_analog, &config).unwrap();
        loop {
            let adc_read = adc.read(&mut adc_pin).unwrap();
            ch_tx.send(adc_read).await.unwrap();

            sleep(Duration::from_millis(100)).await;
        }
    });

    local_set.spawn_local(async move {
        // Настройка SPI для дисплея
        let spi_master_driver = SpiDriver::new(
            peripherals.spi2,
            pin_sck,
            pin_mosi,
            Some(pin_miso),
            &SpiDriverConfig::new(),
        )
        .unwrap();

        let spi_config = spi::config::Config::new().baudrate(100_000.Hz());
        let spi_device =
            SpiDeviceDriver::new(&spi_master_driver, Some(pin_cs), &spi_config).unwrap();
        let pin_dc = PinDriver::output(pin_dc).unwrap();
        let interface = SPIInterface::new(spi_device, pin_dc);

        let mut pin_reset = PinDriver::output(pin_reset).unwrap();
        pin_reset.set_low().unwrap();
        sleep(Duration::from_millis(500)).await;
        pin_reset.set_high().unwrap();

        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        while let Some(new_value) = ch_rx.recv().await {
            info!("ADC: {}", new_value);

            display.clear(BinaryColor::Off).unwrap();

            let text_style = MonoTextStyleBuilder::new()
                .font(&FONT_10X20)
                .text_color(BinaryColor::On)
                .build();

            let text = format!("{}", new_value);
            Text::with_baseline(&text, Point::zero(), text_style, Baseline::Top)
                .draw(&mut display)
                .unwrap();

            display.flush().unwrap();
        }
    });

    local_set.await;

    // Prepare the config.

    let executor_config = ComponentExecutorConfig {
        buffer_size: 10,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    let local_set = LocalSet::new();

    local_set.spawn_local(async {
        ComponentExecutor::<Msg>::new(executor_config)
            .wait_result()
            .await?;

        Ok(()) as anyhow::Result<()>
    });
    local_set.await;

    Ok(())
}

#[cfg(not(feature = "cmp_esp"))]
fn main() {
    unimplemented!()
}
