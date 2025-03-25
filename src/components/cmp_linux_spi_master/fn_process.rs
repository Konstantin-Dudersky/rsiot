use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
    time::sleep,
};

use linux_embedded_hal::spidev::{Spidev, SpidevOptions, SpidevTransfer};
use tracing::trace;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    components_config::spi_master,
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::Config;

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
        devices: config.devices,
    };
    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация SPI ----------------------------------------------------------------------------
    let task = SpiComm {
        input: ch_rx_devices_to_fieldbus,
        output: ch_tx_fieldbus_to_devices,
        spi_adapter_path: config.spi_adapter_path,
        baudrate: config.baudrate,
    };
    join_set_spawn(&mut task_set, task.spawn());

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}

struct SpiComm {
    pub input: mpsc::Receiver<spi_master::FieldbusRequest>,
    pub output: broadcast::Sender<spi_master::FieldbusResponse>,
    pub spi_adapter_path: Vec<&'static str>,
    pub baudrate: u32,
}

impl SpiComm {
    pub async fn spawn(mut self) -> super::Result<()> {
        let options = SpidevOptions::new().max_speed_hz(self.baudrate).build();

        let mut spi_devices = vec![];
        for spi_device in self.spi_adapter_path {
            let mut spi_comm_device = Spidev::open(spi_device).unwrap();
            spi_comm_device.configure(&options).unwrap();
            spi_devices.push(spi_comm_device);
        }

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
            Some(read_data)
        }
        spi_master::Operation::Write(write_data) => {
            let mut transaction = [SpidevTransfer::write(write_data)];
            device.transfer_multiple(&mut transaction).unwrap();
            None
        }
    }
}
