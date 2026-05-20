use linux_embedded_hal::i2cdev::{
    core::{I2CMessage, I2CTransfer},
    linux::{LinuxI2CBus, LinuxI2CError, LinuxI2CMessage},
};
use tokio::{sync::mpsc, task::JoinSet, time::sleep};
use tracing::trace;

use crate::{
    components::shared_tasks::fieldbus_execution::FieldbusExecution,
    components_config::{
        i2c_master,
        master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
    },
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, Error, I2cAddress};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set = JoinSet::new();

    let config_fn_process_master = FieldbusExecution {
        msgbus_linker,
        task_set: &mut task_set,
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
    join_set_spawn(
        &mut task_set,
        "cmp_linux_i2c_master | i2c_comm",
        task.spawn(),
    );

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
        let mut bus = LinuxI2CBus::new(self.dev_i2c.clone())?;

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
                            error = err.to_string();
                            break;
                        }
                    };
                    responses.push(response);
                }

                if error.is_empty() {
                    Ok(responses)
                } else {
                    Err(error)
                }
            };

            let response = i2c_master::FieldbusResponse {
                request_creation_time: request.request_creation_time,
                request_duration: request.request_creation_time.elapsed(),
                request_kind: request.request_kind,
                payload: response_payload,
            };
            let response_with_index = FieldbusResponseWithIndex {
                device_index,
                response,
            };

            trace!("I2C response: {:?}", response_with_index);
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
/// Возвращаем ответ в виде вектора байтов
async fn make_i2c_operation(
    bus: &mut LinuxI2CBus,
    i2c_address: I2cAddress,
    operation: &i2c_master::Operation,
) -> Result<Vec<u8>, LinuxI2CError> {
    // Определяем адрес
    let address = match i2c_address {
        I2cAddress::Direct { address } => address as u16,
        I2cAddress::Mux {
            mux_address,
            channel,
            address,
        } => {
            // Открываем канал на мультиплексоре
            let mux_data = [channel];
            let mut transaction =
                [LinuxI2CMessage::write(&mux_data).with_address(mux_address as u16)];
            bus.transfer(&mut transaction)?;

            address as u16
        }
    };

    let read_result = match operation {
        i2c_master::Operation::Delay { delay } => {
            sleep(*delay).await;
            Ok(vec![])
        }

        i2c_master::Operation::WriteRead {
            write_data,
            read_size,
        } => {
            let mut read_data = vec![0; *read_size as usize];

            let mut transaction = [
                LinuxI2CMessage::write(write_data).with_address(address),
                LinuxI2CMessage::read(&mut read_data).with_address(address),
            ];

            bus.transfer(&mut transaction)?;
            trace!("Read data: {:x?}", read_data);
            Ok(read_data)
        }

        i2c_master::Operation::Write { write_data } => {
            let mut transaction = [LinuxI2CMessage::write(write_data).with_address(address)];
            bus.transfer(&mut transaction)?;
            Ok(vec![])
        }

        i2c_master::Operation::Read { read_size } => {
            let mut read_data = vec![0; *read_size as usize];

            let mut transaction = [LinuxI2CMessage::read(&mut read_data).with_address(address)];
            bus.transfer(&mut transaction)?;
            trace!("Read data: {:x?}", read_data);
            Ok(read_data)
        }
    };

    match i2c_address {
        I2cAddress::Direct { .. } => (),
        I2cAddress::Mux { mux_address, .. } => {
            // Закрываем все каналы на мультиплексоре
            let mut transaction =
                [LinuxI2CMessage::write(&[0x00]).with_address(mux_address as u16)];
            bus.transfer(&mut transaction)?;
        }
    };

    read_result
}
