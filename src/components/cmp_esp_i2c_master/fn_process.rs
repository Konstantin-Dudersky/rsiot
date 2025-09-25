use std::time::Duration;

use esp_idf_svc::hal::delay::TickType;
use esp_idf_svc::hal::{i2c, units::FromValueType};
use esp_idf_svc::hal::{
    i2c::{I2c, I2cDriver, Operation as EspOperation},
    peripheral::Peripheral,
};
use esp_idf_svc::sys::EspError;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio::time::sleep;
use tracing::{trace, warn};

use crate::components::shared_tasks::fn_process_master::FnProcessMaster;
use crate::components_config::i2c_master::{self, Operation};
use crate::components_config::master_device::{
    FieldbusRequestWithIndex, FieldbusResponseWithIndex,
};
use crate::{
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::{Config, ConfigBaudrate, Error};

pub async fn fn_process<TMsg, TI2c, TPeripheral>(
    config: Config<TMsg, TI2c, TPeripheral>,
    msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    const BUFFER_SIZE: usize = 500;

    // Настраиваем I2C
    let baudrate = match config.baudrate {
        ConfigBaudrate::Standard => 100_u32.kHz(),
        ConfigBaudrate::Fast => todo!(),
    };
    let i2c_config = i2c::config::Config::new()
        .baudrate(baudrate.into())
        .sda_enable_pullup(config.pullup_enable)
        .scl_enable_pullup(config.pullup_enable);
    let i2c = I2cDriver::new(config.i2c, config.sda, config.scl, &i2c_config)
        .map_err(Error::I2cDriverCreation)?;

    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

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
        i2c_driver: i2c,
    };
    join_set_spawn(&mut task_set, "cmp_esp_i2c_master | i2c_comm", task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Err(Error::FnProcessEnd)
}

pub struct I2cComm {
    pub input: mpsc::Receiver<FieldbusRequestWithIndex<i2c_master::FieldbusRequest>>,
    pub output: mpsc::Sender<FieldbusResponseWithIndex<i2c_master::FieldbusResponse>>,
    pub i2c_driver: I2cDriver<'static>,
}

impl I2cComm {
    pub async fn spawn(mut self) -> super::Result<()> {
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
                    let response = make_i2c_operation(
                        &mut self.i2c_driver,
                        request.address,
                        &operation,
                        Duration::from_millis(10),
                    )
                    .await;
                    let response = match response {
                        Ok(response) => response,
                        Err(err) => {
                            warn!("Error during i2c operation: {:?}", err);
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

        Err(Error::TaskEndI2cComm)
    }
}

/// Выполняем обмен данными
///
/// Если присутствует операция чтения, то возвращаем данные
async fn make_i2c_operation<'a>(
    i2c_driver: &mut I2cDriver<'a>,
    address: u8,
    operation: &Operation,
    timeout: Duration,
) -> Result<Option<Vec<u8>>, EspError> {
    match operation {
        Operation::Delay { delay } => {
            sleep(*delay).await;
            Ok(None)
        }
        Operation::WriteRead {
            write_data,
            read_size,
        } => {
            let mut read_data = vec![0; *read_size as usize];
            let mut transaction = [
                EspOperation::Write(write_data),
                EspOperation::Read(&mut read_data),
            ];

            i2c_driver.transaction(address, &mut transaction, millis_to_ticks(timeout))?;

            trace!("Read data: {:x?}", read_data);
            Ok(Some(read_data))
        }
        Operation::Write { write_data } => {
            let mut transaction = [EspOperation::Write(write_data)];
            i2c_driver.transaction(address, &mut transaction, millis_to_ticks(timeout))?;
            Ok(None)
        }
        Operation::Read { read_size } => {
            let mut read_data = vec![0; *read_size as usize];
            let mut transaction = [EspOperation::Read(&mut read_data)];
            i2c_driver.transaction(address, &mut transaction, millis_to_ticks(timeout))?;
            trace!("Read data: {:x?}", read_data);
            Ok(Some(read_data))
        }
    }
}

fn millis_to_ticks(millis: Duration) -> u32 {
    let millis = millis.as_millis() as u64;
    let tick = TickType::new_millis(millis);
    tick.ticks()
}
