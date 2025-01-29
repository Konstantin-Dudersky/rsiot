use esp_idf_svc::hal::{
    gpio::AnyIOPin,
    peripheral::Peripheral,
    spi::{config, Operation, Spi, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    units::Hertz,
};
use tokio::{
    sync::{broadcast, mpsc},
    time::sleep,
};
use tracing::trace;

use crate::components_config::spi_master;

const MAX_SLAVES: usize = 4;

pub struct SpiComm<TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins,
{
    pub input: mpsc::Receiver<spi_master::Request>,
    pub output: broadcast::Sender<spi_master::Response>,
    pub spi: TSpi,
    pub pin_miso: AnyIOPin,
    pub pin_mosi: AnyIOPin,
    pub pin_sck: AnyIOPin,
    pub pin_cs0: Option<AnyIOPin>,
    pub pin_cs1: Option<AnyIOPin>,
    pub pin_cs2: Option<AnyIOPin>,
    pub pin_cs3: Option<AnyIOPin>,
    pub baudrate: Hertz,
}

impl<TSpi, TPeripheral> SpiComm<TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins,
{
    pub async fn spawn(mut self) -> super::Result<()> {
        let spi_master_driver = SpiDriver::new(
            self.spi,
            self.pin_sck,
            self.pin_mosi,
            Some(self.pin_miso),
            &SpiDriverConfig::new(),
        )
        .unwrap();

        let config = config::Config::new().baudrate(self.baudrate);
        let mut spi_devices = [
            SpiDeviceDriver::new(&spi_master_driver, self.pin_cs0, &config).unwrap(),
            SpiDeviceDriver::new(&spi_master_driver, self.pin_cs1, &config).unwrap(),
            SpiDeviceDriver::new(&spi_master_driver, self.pin_cs2, &config).unwrap(),
            SpiDeviceDriver::new(&spi_master_driver, self.pin_cs3, &config).unwrap(),
        ];

        while let Some(request) = self.input.recv().await {
            trace!("New spi request: {:?}", request);

            let pin_cs = request.pin_cs as usize;

            if pin_cs >= MAX_SLAVES {
                panic!("Maximum supported slaves: {}", MAX_SLAVES);
            }

            let selected_device = &mut spi_devices[pin_cs];

            let mut response_payload = vec![];

            for operation in request.operations {
                let response = made_spi_operation(selected_device, &operation).await;
                if let Some(response) = response {
                    response_payload.push(response);
                }
            }

            let response = spi_master::Response {
                pin_cs: request.pin_cs,
                request_creation_time: request.request_creation_time,
                request_kind: request.request_kind,
                payload: response_payload,
            };

            self.output.send(response).unwrap();
        }
        Ok(())
    }
}

/// Выполнеям обмен данными
///
/// Если присутствует операция чтения, то возвращаем данные
async fn made_spi_operation<'a>(
    device: &mut SpiDeviceDriver<'a, &SpiDriver<'a>>,
    operation: &spi_master::Operation,
) -> Option<Vec<u8>> {
    match operation {
        spi_master::Operation::Delay(duration) => {
            sleep(*duration).await;
            None
        }
        spi_master::Operation::WriteRead(write_data, read_len) => {
            let mut read_data = vec![0; *read_len as usize];
            let mut transaction = [
                Operation::Write(write_data),
                Operation::Read(&mut read_data),
            ];
            device.transaction(&mut transaction).unwrap();
            Some(read_data)
        }
        spi_master::Operation::Write(write_data) => {
            let mut transaction = [Operation::Write(write_data)];
            device.transaction(&mut transaction).unwrap();
            None
        }
    }
}
