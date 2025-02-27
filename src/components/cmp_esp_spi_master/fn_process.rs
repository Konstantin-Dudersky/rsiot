use esp_idf_svc::hal::{
    gpio::AnyIOPin,
    peripheral::Peripheral,
    spi::{config, Operation, Spi, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig},
    units::FromValueType,
};
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
    time::sleep,
};
use tracing::trace;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::spi_master,
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{config::ConfigDevicesCommSettings, Config};

pub async fn fn_process<TMsg, TService, TSpi, TPeripheral>(
    config: Config<TMsg, TSpi, TPeripheral>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins + 'static,
{
    const BUFFER_SIZE: usize = 500;

    let mut task_set = JoinSet::new();

    let config_fn_process_master = FnProcessMaster {
        msg_bus: msg_bus.clone(),
        buffer_size: BUFFER_SIZE,
        task_set: &mut task_set,
        error_msgbus_to_broadcast: super::Error::TaskMsgbusToBroadcast,
        error_filter: super::Error::TaskFilter,
        error_mpsc_to_msgbus: super::Error::TaskMpscToMsgBus,
        error_master_device: super::Error::DeviceError,
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
    join_set_spawn(&mut task_set, task.spawn());

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
    pub input: mpsc::Receiver<spi_master::FieldbusRequest>,
    pub output: broadcast::Sender<spi_master::FieldbusResponse>,
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

        // TODO - добавить в конфигурацию

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

        while let Some(request) = self.input.recv().await {
            trace!("New spi request: {:?}", request);

            let pin_cs = request.pin_cs as usize;

            // Номер CS недоступен
            if pin_cs >= spi_devices.len() {
                let err = super::Error::CsNotAvailable {
                    cs: request.pin_cs,
                    max_cs: spi_devices.len() as u8,
                };
                return Err(err);
            }

            let selected_device = &mut spi_devices[pin_cs];

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
