use std::io::{Read, Write};

use tokio::{sync::mpsc, task::JoinSet, time::sleep};

use linux_embedded_hal::spidev::{Spidev, SpidevOptions, SpidevTransfer};
use tracing::{info, trace};

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::{
        master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
        spi_master,
    },
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{config::ConfigDevicesCommSettings, Config};

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
        error_msgbus_to_broadcast: super::Error::TaskMsgbusToBroadcast,
        error_filter: super::Error::TaskFilter,
        error_mpsc_to_msgbus: super::Error::TaskMpscToMsgBus,
        error_master_device: super::Error::DeviceError,
        error_tokiompscsend: || super::Error::TokioSyncMpsc,
        devices: config.devices,
    };
    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация SPI ----------------------------------------------------------------------------
    let task = SpiComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        devices_comm_settings: config.devices_comm_settings,
    };
    join_set_spawn(&mut task_set, task.spawn());

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

impl SpiComm {
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut spi_devices: Vec<Spidev> = self
            .devices_comm_settings
            .into_iter()
            .map(|dvc| {
                let mut device = Spidev::open(dvc.spi_adapter_path).unwrap();
                let options = SpidevOptions::new()
                    .max_speed_hz(dvc.baudrate)
                    .mode(dvc.spi_mode.into())
                    .lsb_first(false)
                    // .bits_per_word(8)
                    .build();
                device.configure(&options).unwrap();
                device
            })
            .collect();

        while let Some(fieldbus_request) = self.input.recv().await {
            trace!("New spi request: {:?}", fieldbus_request);

            let device_index = fieldbus_request.device_index;

            // Номер CS недоступен
            if device_index >= spi_devices.len() {
                let err = super::Error::CsNotAvailable {
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

            self.output
                .send(response_with_index)
                .await
                .map_err(|_| super::Error::TokioSyncMpsc)?;
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
            // let mut transaction = [
            //     SpidevTransfer::write(write_data),
            //     SpidevTransfer::read(&mut read_data),
            // ];
            // device.transfer_multiple(&mut transaction).unwrap();
            //
            device.write_all(write_data).unwrap();
            device.read_exact(&mut read_data).unwrap();

            trace!("Read data: {:?}", read_data);
            Some(read_data)
        }
        spi_master::Operation::Write(write_data) => {
            // let mut transaction = [SpidevTransfer::write(write_data)];
            // device.transfer_multiple(&mut transaction).unwrap();\
            //
            device.write_all(write_data).unwrap();
            //
            None
        }
    }
}
