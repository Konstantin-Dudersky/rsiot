use tokio::{sync::mpsc, task::JoinSet, time::sleep};

use linux_embedded_hal::{
    gpio_cdev::{Chip, LineHandle, LineRequestFlags},
    spidev::{Spidev, SpidevOptions, SpidevTransfer},
};
use tracing::trace;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::{
        master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
        spi_master,
    },
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{
    config::{ConfigDevicesCommSettings, LinuxDevice},
    Config, Error,
};

pub async fn fn_process<TMsg>(config: Config<TMsg>, msg_bus: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    const BUFFER_SIZE: usize = 500;

    let mut task_set = JoinSet::new();

    let config_fn_process_master = FnProcessMaster {
        msg_bus: msg_bus.clone(),
        buffer_size: BUFFER_SIZE,
        task_set: &mut task_set,
        error_msgbus_to_broadcast: Error::TaskMsgbusToBroadcast,
        error_filter: Error::TaskFilter,
        error_mpsc_to_msgbus: Error::TaskMpscToMsgBus,
        error_master_device: Error::DeviceError,
        error_tokiompscsend: || Error::TokioSyncMpsc,
        devices: config.devices,
    };
    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация SPI ----------------------------------------------------------------------------
    let task = SpiComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        devices_comm_settings: config.devices_comm_settings,
    };

    join_set_spawn(&mut task_set, "cmp_linux_spi_master", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}

struct SpiComm {
    pub input: mpsc::Receiver<FieldbusRequestWithIndex<spi_master::FieldbusRequest>>,
    pub output: mpsc::Sender<FieldbusResponseWithIndex<spi_master::FieldbusResponse>>,
    pub devices_comm_settings: Vec<ConfigDevicesCommSettings>,
}

struct SpidevWithCS {
    pub spidev: Spidev,
    pub cs: Option<LineHandle>,
}

impl SpiComm {
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut spi_devices: Vec<SpidevWithCS> = self
            .devices_comm_settings
            .into_iter()
            .enumerate()
            .map(|(index, dvc)| {
                let spidev = match &dvc.linux_device {
                    LinuxDevice::Spi { dev_spi } => dev_spi,
                    LinuxDevice::SpiWithCs { dev_spi, .. } => dev_spi,
                };
                let mut spidev = Spidev::open(spidev).unwrap();
                let spi_options = SpidevOptions::new()
                    .max_speed_hz(dvc.baudrate)
                    .mode(dvc.spi_mode.into())
                    .build();
                spidev.configure(&spi_options).unwrap();

                let cs = match dvc.linux_device {
                    LinuxDevice::SpiWithCs {
                        dev_spi,
                        dev_gpio,
                        gpio_line,
                        ..
                    } => {
                        let mut chip = Chip::new(dev_gpio)
                            .map_err(|e| Error::GpioSetup(e.to_string()))
                            .unwrap();
                        let cs = chip
                            .get_line(gpio_line as u32)
                            .map_err(|e| Error::GpioSetup(e.to_string()))
                            .unwrap();
                        let consumer = format!("CS{index} for {dev_spi}");
                        let cs = cs
                            .request(LineRequestFlags::OUTPUT, 1, &consumer)
                            .map_err(|e| Error::GpioSetup(e.to_string()))
                            .unwrap();
                        Some(cs)
                    }
                    LinuxDevice::Spi { .. } => None,
                };

                SpidevWithCS { spidev, cs }
            })
            .collect();

        while let Some(fieldbus_request) = self.input.recv().await {
            trace!("New spi request: {:?}", fieldbus_request);

            let device_index = fieldbus_request.device_index;

            // Номер CS недоступен
            if device_index >= spi_devices.len() {
                let err = Error::CsNotAvailable {
                    cs: device_index as u8,
                    max_cs: spi_devices.len() as u8,
                };
                return Err(err);
            }

            let selected_device = &mut spi_devices[device_index];
            let request = fieldbus_request.request;

            // Ответы от слейва
            let mut response_payload = vec![];

            // Выполняем все операции в цикле
            //
            // Сигналом CS управляем после каждой операции. В противном случае в Luckfox например
            // коммуникация работает плохо
            for operation in request.operations {
                // Устанавливаем CS
                if let Some(pin_cs) = &selected_device.cs {
                    pin_cs.set_value(0).unwrap();
                }

                let response = make_spi_operation(&mut selected_device.spidev, &operation).await;
                if let Some(response) = response {
                    response_payload.push(response);
                }

                // Сбрасываем CS
                if let Some(pin_cs) = &selected_device.cs {
                    pin_cs.set_value(1).unwrap();
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

            self.output
                .send(response_with_index)
                .await
                .map_err(|_| Error::TokioSyncMpsc)?;
        }
        Ok(())
    }
}

/// Выполняем обмен данными
///
/// Если присутствует операция чтения, то возвращаем данные
async fn make_spi_operation(
    device: &mut Spidev,
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
                SpidevTransfer::write(write_data),
                SpidevTransfer::read(&mut read_data),
            ];
            device.transfer_multiple(&mut transaction).unwrap();
            trace!("Read data: {:x?}", read_data);
            Some(read_data)
        }
        spi_master::Operation::Write(write_data) => {
            let mut transaction = [SpidevTransfer::write(write_data)];
            device.transfer_multiple(&mut transaction).unwrap();
            None
        }
    }
}
