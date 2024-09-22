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
    pub async fn spawn(mut self) -> super::Result<()> {
        let spi_master_driver = SpiDriver::new(
            self.config.spi,
            self.config.pin_sck,
            self.config.pin_mosi,
            Some(self.config.pin_miso),
            &SpiDriverConfig::new(),
        )
        .unwrap();

        let mut spi_slaves = vec![];
        let mut spi_slaves_config = vec![];

        for device_config in self.config.devices {
            let config = config::Config::new().baudrate(13.MHz().into());
            let spi_slave =
                SpiDeviceDriver::new(&spi_master_driver, Some(device_config.pin_cs), &config)
                    .unwrap();
            spi_slaves.push(spi_slave);
            spi_slaves_config.push(SpiSlaveConfig {
                fn_init: device_config.fn_init,
                fn_input: device_config.fn_input,
                fn_output: device_config.fn_output,
            })
        }

        for (index, spi_slave_config) in spi_slaves_config.iter().enumerate() {
            (spi_slave_config.fn_init)(&mut spi_slaves[index]);
        }

        while let Some(msg) = self.input.recv().await {
            match msg {
                InnerMessage::Message(msg) => {
                    for (index, spi_slave_config) in spi_slaves_config.iter().enumerate() {
                        (spi_slave_config.fn_input)(&msg, &mut spi_slaves[index]);
                    }
                }
                InnerMessage::Periodic => {
                    for (index, spi_slave_config) in spi_slaves_config.iter().enumerate() {
                        let msgs = (spi_slave_config.fn_output)(&mut spi_slaves[index]);
                        for msg in msgs {
                            self.output.send(msg).await.unwrap();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

struct SpiSlaveConfig<TMsg> {
    pub fn_init: for<'a> fn(&mut SpiDeviceDriver<'a, &SpiDriver<'a>>),
    pub fn_input: for<'a> fn(&Message<TMsg>, &mut SpiDeviceDriver<'a, &SpiDriver<'a>>),
    pub fn_output: for<'a> fn(&mut SpiDeviceDriver<'a, &SpiDriver<'a>>) -> Vec<Message<TMsg>>,
}
