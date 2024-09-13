use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{config, Spi, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    units::FromValueType,
};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::message::{Message, MsgDataBound};

use super::{Config, InnerMessage};

pub struct SpiComm<TMsg, TSpi, TPeripheral>
where
    TMsg: MsgDataBound,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins,
{
    pub input: Receiver<InnerMessage<TMsg>>,
    pub output: Sender<Message<TMsg>>,
    pub config: Config<TMsg, TSpi, TPeripheral>,
}

impl<TMsg, TSpi, TPeripheral> SpiComm<TMsg, TSpi, TPeripheral>
where
    TMsg: MsgDataBound,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins,
{
    pub async fn spawn(mut self) {
        let spi_master_driver = SpiDriver::new(
            self.config.spi,
            self.config.pin_sck,
            self.config.pin_mosi,
            Some(self.config.pin_miso),
            &SpiDriverConfig::new(),
        )
        .unwrap();

        let mut spi_slaves = vec![];

        for device in self.config.devices {
            let config = config::Config::new().baudrate(13.MHz().into());
            let spi_slave =
                SpiDeviceDriver::new(&spi_master_driver, Some(device.pin_cs), &config).unwrap();
            let spi_slave = SpiSlave {
                spi_slave,
                fn_init: device.fn_init,
                fn_input: device.fn_input,
                fn_output: device.fn_output,
            };
            spi_slaves.push(spi_slave);
        }

        for spi_slave in spi_slaves.iter() {
            (spi_slave.fn_init)(&spi_slave.spi_slave);
        }

        while let Some(msg) = self.input.recv().await {
            match msg {
                InnerMessage::Message(_) => todo!(),
                InnerMessage::Periodic => todo!(),
            }
        }
    }
}

struct SpiSlave<'a, TMsg> {
    pub spi_slave: SpiDeviceDriver<'a, &'a SpiDriver<'a>>,
    pub fn_init: fn(&SpiDeviceDriver<'a, &SpiDriver<'a>>),
    pub fn_input: fn(&Message<TMsg>, &SpiDeviceDriver<'a, &SpiDriver<'a>>),
    pub fn_output: fn() -> Vec<Message<TMsg>>,
}
