use esp_idf_svc::hal::{
    gpio::AnyIOPin,
    peripheral::Peripheral,
    spi::{Operation, Spi, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig, config},
    units::FromValueType,
};
use tokio::{sync::mpsc, task::JoinSet, time::sleep};
use tracing::trace;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::{
        master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
        spi_master,
    },
    executor::{MsgBusInput, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, config::ConfigDevicesCommSettings};

pub async fn fn_process<TMsg, TSpi, TPeripheral>(
    config: Config<TMsg, TSpi, TPeripheral>,
    input: MsgBusInput<TMsg>,
    output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins + 'static,
{
    let mut task_set = JoinSet::new();

    let config_fn_process_master = FnProcessMaster {
        input,
        output,
        task_set: &mut task_set,
        error_filter: super::Error::TaskFilter,
        error_mpsc_to_msgbus: super::Error::TaskMpscToMsgBus,
        error_master_device: super::Error::DeviceError,
        error_tokiompscsend: || super::Error::TokioMpscSend,
        devices: config.devices,
    };
    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация SPI ----------------------------------------------------------------------------
    let task = SpiComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        spi: config.spi,
        pin_miso: config.pin_miso,
        pin_mosi: config.pin_mosi,
        pin_sck: config.pin_sck,
        devices_comm_settings: config.devices_comm_settings,
    };
    join_set_spawn(&mut task_set, "cmp_esp_spi_master", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}

struct SpiComm<TSpi, TPeripheral>
where
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins,
{
    pub input: mpsc::Receiver<FieldbusRequestWithIndex<spi_master::FieldbusRequest>>,
    pub output: mpsc::Sender<FieldbusResponseWithIndex<spi_master::FieldbusResponse>>,
    pub spi: TSpi,
    pub pin_miso: AnyIOPin,
    pub pin_mosi: AnyIOPin,
    pub pin_sck: AnyIOPin,
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,
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

        let mut spi_devices: Vec<SpiDeviceDriver<'_, &SpiDriver<'_>>> = self
            .devices_comm_settings
            .into_iter()
            .map(|dvc| {
                let config = config::Config::new()
                    .baudrate(dvc.baudrate.Hz())
                    .data_mode(dvc.spi_mode.into());
                SpiDeviceDriver::new(&spi_master_driver, Some(dvc.pin_cs), &config).unwrap()
            })
            .collect();

        while let Some(request_with_index) = self.input.recv().await {
            trace!("New spi request: {:?}", request_with_index);

            let device_index = request_with_index.device_index;
            let request = request_with_index.request;

            // Номер CS недоступен
            if device_index >= spi_devices.len() {
                let err = super::Error::CsNotAvailable {
                    cs: device_index as u8,
                    max_cs: spi_devices.len() as u8,
                };
                return Err(err);
            }

            let selected_device = &mut spi_devices[device_index];

            // Ответы от слейва
            let mut response_payload = vec![];

            // Выполняем все операции в цикле
            for operation in request.operations {
                let response = make_spi_operation(selected_device, &operation).await;
                if let Some(response) = response {
                    response_payload.push(response);
                }
            }

            let response = spi_master::FieldbusResponse {
                request_creation_time: request.request_creation_time,
                request_kind: request.request_kind,
                payload: response_payload,
            };
            let response_with_index = FieldbusResponseWithIndex {
                device_index,
                response,
            };

            trace!("Response: {:?}", response_with_index);

            self.output.send(response_with_index).await.unwrap();
        }
        Ok(())
    }
}

/// Выполняем обмен данными
///
/// Если присутствует операция чтения, то возвращаем данные
async fn make_spi_operation<'a>(
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
            trace!("Write SPI data: {:x?}", write_data);
            let mut transaction = [
                Operation::Write(write_data),
                Operation::Read(&mut read_data),
            ];
            device.transaction(&mut transaction).unwrap();
            trace!("Read SPI data: {:x?}", read_data);
            Some(read_data)
        }
        spi_master::Operation::Write(write_data) => {
            trace!("Write SPI data: {:x?}", write_data);
            let mut transaction = [Operation::Write(write_data)];
            device.transaction(&mut transaction).unwrap();
            None
        }
        spi_master::Operation::Read { read_size } => {
            let mut read_data = vec![0; *read_size as usize];
            let mut transaction = [Operation::Read(&mut read_data)];
            device.transaction(&mut transaction).unwrap();
            trace!("Read SPI data: {:x?}", read_data);
            Some(read_data)
        }
    }
}
