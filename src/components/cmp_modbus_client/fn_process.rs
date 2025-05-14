use std::{net::SocketAddr, sync::Arc};

use tokio::{
    sync::{mpsc, Semaphore},
    task::JoinSet,
    time::{sleep, timeout, Duration},
};
use tokio_modbus::prelude::*;
use tokio_util::task::TaskTracker;
use tracing::{debug, warn};

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::master_device::{FieldbusRequestWithIndex, FieldbusResponseWithIndex},
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{
    config::{Config, ConfigDevicesCommSettings, RequestContent, ResponseContent},
    error::Error,
    ClientType, FieldbusRequest, FieldbusResponse,
};

const MAX_TASKS_PER_DEVICE: usize = 10;

pub async fn fn_process<TMessage>(
    config: Config<TMessage>,
    msg_bus: CmpInOut<TMessage>,
) -> Result<(), Error>
where
    TMessage: MsgDataBound + 'static,
{
    const BUFFER_SIZE: usize = 500;

    let mut task_set = JoinSet::new();

    if !config.enabled {
        loop {
            warn!("Service disabled");
            sleep(Duration::from_secs(u64::MAX)).await
        }
    }

    let config_fn_process_master = FnProcessMaster {
        msg_bus: msg_bus.clone(),
        buffer_size: BUFFER_SIZE,
        task_set: &mut task_set,
        error_msgbus_to_broadcast: Error::TaskMsgbusToBroadcast,
        error_filter: Error::TaskFilter,
        error_mpsc_to_msgbus: Error::TaskMpscToMsgBus,
        error_master_device: Error::Device,
        error_tokiompscsend: || Error::TokioSyncMpsc,
        devices: config.devices,
    };
    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация Modbus
    let task = ModbusComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        comm_settings: config.devices_comm_settings,
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??;
    }
    Ok(())
}

struct ModbusComm {
    pub input: mpsc::Receiver<FieldbusRequestWithIndex<FieldbusRequest>>,
    pub output: mpsc::Sender<FieldbusResponseWithIndex<FieldbusResponse>>,
    pub comm_settings: Vec<ConfigDevicesCommSettings>,
}
impl ModbusComm {
    pub async fn spawn(mut self) -> super::Result<()> {
        let task_sets = vec![TaskTracker::new(); self.comm_settings.len()];

        // Создаем вектор кол-ва параллельных подключений для каждого устройства
        let available_connections: Vec<Arc<Semaphore>> = self
            .comm_settings
            .iter()
            .map(|comm_stg| -> Arc<Semaphore> {
                let sem = Semaphore::new(comm_stg.concurrent_connections as usize);
                Arc::new(sem)
            })
            .collect();

        while let Some(request_with_index) = self.input.recv().await {
            let device_index = request_with_index.device_index;

            if task_sets[device_index].len() >= MAX_TASKS_PER_DEVICE {
                warn!(
                    " The number of running tasks exceeds the maximum level: {}",
                    MAX_TASKS_PER_DEVICE
                );
                continue;
            }

            let task = ModbusCommSingleRequest {
                output: self.output.clone(),
                comm_settings: self.comm_settings[device_index],
                request_with_index,
                available_connections: available_connections[device_index].clone(),
            };

            task_sets[device_index].spawn(task.spawn());
        }

        Ok(())
    }
}

struct ModbusCommSingleRequest {
    pub output: mpsc::Sender<FieldbusResponseWithIndex<FieldbusResponse>>,
    pub comm_settings: ConfigDevicesCommSettings,
    pub request_with_index: FieldbusRequestWithIndex<FieldbusRequest>,
    pub available_connections: Arc<Semaphore>,
}
impl ModbusCommSingleRequest {
    pub async fn spawn(self) -> super::Result<()> {
        let _permit = self.available_connections.acquire().await.unwrap();

        let fieldbus_response_with_index = self.internal().await;

        let fieldbus_response_with_index = match fieldbus_response_with_index {
            Ok(v) => v,
            Err(e) => {
                warn!("Error: {:?}", e);
                return Ok(());
            }
        };

        self.output
            .send(fieldbus_response_with_index)
            .await
            .map_err(|_| Error::TokioSyncMpsc)?;

        Ok(())
    }

    async fn internal(&self) -> super::Result<FieldbusResponseWithIndex<FieldbusResponse>> {
        let fieldbus_request = self.request_with_index.request.clone();

        let mut ctx = match self.comm_settings.client_type {
            ClientType::Tcp { host, port } => {
                let socket_addr = SocketAddr::new(host, port);
                debug!("Try to establish connection to socket: {:?}", socket_addr);
                let slave = Slave(self.comm_settings.unit_id);

                let task = tcp::connect_slave(socket_addr, slave);
                let ctx = timeout(self.comm_settings.timeout, task).await??;

                debug!("Connection established: {:?}", ctx);
                ctx
            }
            ClientType::Rtu => {
                unimplemented!()
            }
        };

        let response_content = match self.request_with_index.request.operation {
            RequestContent::ReadCoils {
                start_address,
                count,
            } => {
                let task = ctx.read_coils(start_address, count);
                let response = timeout(self.comm_settings.timeout, task).await??;
                ResponseContent::BitVector(response)
            }
            RequestContent::ReadHoldingRegisters {
                start_address,
                count,
            } => {
                let task = ctx.read_holding_registers(start_address, count);
                let response = timeout(self.comm_settings.timeout, task).await??;
                ResponseContent::WordVector(response)
            }
            RequestContent::ReadInputRegisters {
                start_address,
                count,
            } => {
                let task = ctx.read_input_registers(start_address, count);
                let response = timeout(self.comm_settings.timeout, task).await??;
                ResponseContent::WordVector(response)
            }
            RequestContent::WriteSingleRegister {
                address: start_address,
                value,
            } => {
                let task = ctx.write_single_register(start_address, value);
                timeout(self.comm_settings.timeout, task).await??;
                ResponseContent::Unit
            }
        };

        let fieldbus_response = FieldbusResponse::from_request(fieldbus_request, response_content);

        let fieldbus_response_with_index = FieldbusResponseWithIndex {
            device_index: self.request_with_index.device_index,
            response: fieldbus_response,
        };

        Ok(fieldbus_response_with_index)
    }
}
