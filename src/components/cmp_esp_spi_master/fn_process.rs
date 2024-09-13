use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{config, Spi, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    units::FromValueType,
};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg, TSpi, TPeripheral>(
    config: Config<TMsg, TSpi, TPeripheral>,
    msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins,
{
    let spi_master_driver = SpiDriver::new(
        config.spi,
        config.pin_sck,
        config.pin_mosi,
        Some(config.pin_miso),
        &SpiDriverConfig::new(),
    )
    .unwrap();

    for device in config.devices {
        let config = config::Config::new().baudrate(13.MHz().into());
        let spi_slave =
            SpiDeviceDriver::new(spi_master_driver, Some(device.pin_cs), &config).unwrap();
        (device.fn_init)(&spi_slave)
    }

    Ok(())
}
