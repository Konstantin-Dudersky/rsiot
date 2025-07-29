use linux_embedded_hal::i2cdev::{
    core::{I2CMessage, I2CTransfer},
    linux::{LinuxI2CBus, LinuxI2CError, LinuxI2CMessage},
};
use tokio::{sync::mpsc, task::JoinSet, time::sleep};
use tracing::trace;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::{
        i2c_master,
        master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
    },
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{Config, Error};

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

    let task = I2cComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        dev_i2c: config.dev_i2c,
    };

    join_set_spawn(&mut task_set, "cmp_linux_i2c_master", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}

pub struct I2cComm {
    pub input: mpsc::Receiver<FieldbusRequestWithIndex<i2c_master::FieldbusRequest>>,
    pub output: mpsc::Sender<FieldbusResponseWithIndex<i2c_master::FieldbusResponse>>,
    pub dev_i2c: String,
}
impl I2cComm {
    pub async fn spawn(mut self) -> super::Result<()> {
        let mut bus = LinuxI2CBus::new(self.dev_i2c)?;

        while let Some(fieldbus_request) = self.input.recv().await {
            trace!("New i2c request: {:?}", fieldbus_request);

            let device_index = fieldbus_request.device_index;
            let request = fieldbus_request.request;

            // Выполняем все операции в цикле
            let response_payload = {
                // Ответы от слейва
                let mut responses = vec![];
                let mut error = "".to_string();

                for operation in request.operations {
                    let response = make_i2c_operation(&mut bus, request.address, &operation).await;
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => {
                            trace!("Error during i2c operation: {:?}", err);
                            error = err.to_string();
                            break;
                        }
                    };
                    if let Some(response) = response {
                        responses.push(response);
                    }
                }

                if error.is_empty() {
                    Ok(responses)
                } else {
                    Err(error)
                }
            };

            let response = i2c_master::FieldbusResponse {
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
async fn make_i2c_operation(
    bus: &mut LinuxI2CBus,
    address: u8,
    operation: &i2c_master::Operation,
) -> Result<Option<Vec<u8>>, LinuxI2CError> {
    match operation {
        i2c_master::Operation::Delay { delay } => {
            sleep(*delay).await;
            Ok(None)
        }
        i2c_master::Operation::WriteRead {
            write_data,
            read_size,
        } => {
            let mut read_data = vec![0; *read_size as usize];
            let mut transaction = [
                LinuxI2CMessage::write(write_data).with_address(address as u16),
                LinuxI2CMessage::read(&mut read_data).with_address(address as u16),
            ];
            bus.transfer(&mut transaction)?;
            trace!("Read data: {:x?}", read_data);
            Ok(Some(read_data))
        }
        i2c_master::Operation::Write { write_data } => {
            let mut transaction = [LinuxI2CMessage::write(write_data).with_address(address as u16)];
            bus.transfer(&mut transaction)?;
            Ok(None)
        }
        i2c_master::Operation::Read { read_size } => {
            let mut read_data = vec![0; *read_size as usize];
            let mut transaction =
                [LinuxI2CMessage::read(&mut read_data).with_address(address as u16)];
            bus.transfer(&mut transaction)?;
            trace!("Read data: {:x?}", read_data);
            Ok(Some(read_data))
        }
    }
}
