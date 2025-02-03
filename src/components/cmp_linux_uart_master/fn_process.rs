use tokio::task::JoinSet;

use crate::{
    components::shared_tasks::fn_process_master::FnProcessMaster,
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{uart_comm::UartComm, Config};

pub async fn fn_process<TMsg, TService, const MESSAGE_LEN: usize>(
    config: Config<TMsg, MESSAGE_LEN>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
{
    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    const BUFFER_SIZE: usize = 1000;

    let config_fn_process_master = FnProcessMaster {
        msg_bus,
        buffer_size: BUFFER_SIZE,
        task_set: &mut task_set,
        error_msgbus_to_broadcast: super::Error::TaskMsgbusToBroadcast,
        error_filter: super::Error::TaskFilterIdenticalData,
        error_mpsc_to_msgbus: super::Error::TaskMpscToMsgBus,
        error_master_device: super::Error::Device,
        devices: config.devices,
    };

    let (ch_rx_devices_to_fieldbus, ch_tx_fieldbus_to_devices) = config_fn_process_master.spawn();

    // Коммуникация UART ---------------------------------------------------------------------------
    let task = UartComm {
        wait_after_write: config.wait_after_write,
        pin_rts: config.pin_rts,
        ch_rx_devices_to_fieldbus,
        ch_tx_fieldbus_to_devices,
        port: config.port,
        baudrate: config.baudrate,
        data_bits: config.data_bits,
        parity: config.parity,
        stop_bits: config.stop_bits,
        gpio_chip: config.gpio_chip,
    };
    join_set_spawn(&mut task_set, task.spawn::<MESSAGE_LEN>());

    // Ожидание выполнения -------------------------------------------------------------------------
    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Ok(())
}

// pub async fn fn_process<TMsg, TService, const MESSAGE_LEN: usize>(
//     config: Config<TMsg, MESSAGE_LEN>,
//     msg_bus: CmpInOut<TMsg, TService>,
// ) -> super::Result<()>
// where
//     TMsg: MsgDataBound + 'static,
//     TService: ServiceBound + 'static,
// {
//     let serial_port_builder = serialport::new("", 0)
//         .path(config.port)
//         .baud_rate(config.baudrate.into())
//         .data_bits(config.data_bits.into())
//         .parity(config.parity.into())
//         .stop_bits(config.stop_bits.into())
//         .timeout(Duration::from_millis(100));
//     let port = serial_port_builder
//         .open()
//         .map_err(|e| super::Error::OpenSerialPort(e.to_string()))?;
//     let port = Arc::new(Mutex::new(port));

//     // Настраиваем пин для сигнала RTS
//     let pin_rts = match config.pin_rts {
//         Some(pin_rts) => {
//             let mut chip =
//                 Chip::new(config.gpio_chip).map_err(|e| super::Error::GpioSetup(e.to_string()))?;
//             let pin_rts = chip
//                 .get_line(pin_rts)
//                 .map_err(|e| super::Error::GpioSetup(e.to_string()))?;
//             let pin_rts = pin_rts
//                 .request(LineRequestFlags::OUTPUT, 0, "uart-rts")
//                 .map_err(|e| super::Error::GpioSetup(e.to_string()))?;
//             Some(pin_rts)
//         }
//         None => None,
//     };

//     let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

//     // Канал передачи данных из драйверов в канал UART
//     let (ch_tx_device_to_uart, ch_rx_device_to_uart) = mpsc::channel(1000);
//     // Канал передачи из канала UART всем драйверам
//     let (ch_tx_uart_to_device, ch_rx_uart_to_device) = broadcast::channel(1000);
//     // Канал передачи со входа компонента на устройства
//     let (ch_tx_msgbus_to_device, ch_rx_msgbus_to_device) = broadcast::channel(1000);
//     // Канал передачи из устройств на выход компонента
//     let (ch_rx_device_to_msgbus, ch_tx_device_to_msgbus) = mpsc::channel(1000);

//     // Задача записи в UART ------------------------------------------------------------------------
//     let task = tasks::UartWrite {
//         input: ch_rx_device_to_uart,
//         port: port.clone(),
//         wait_after_write: config.wait_after_write,
//         pin_rts,
//     };
//     task_set.spawn_blocking(|| task.spawn());

//     // Задача чтения из UART -----------------------------------------------------------------------
//     let task = tasks::UartRead {
//         output: ch_tx_uart_to_device,
//         port: port.clone(),
//     };
//     task_set.spawn_blocking(|| task.spawn());

//     // Задача перенаправления входящих сообщений на все устройства ---------------------------------
//     let task = shared_tasks::msgbus_to_broadcast::MsgBusToBroadcast {
//         msgbus: msg_bus.clone(),
//         output: ch_tx_msgbus_to_device,
//     };
//     join_set_spawn(
//         &mut task_set,
//         task.spawn().map_err(super::Error::TaskMsgbusToBroadcast),
//     );

//     // Задача выполнения драйверов устройств -------------------------------------------------------
//     for device in config.devices {
//         let ch_rx_uart_to_device = ch_rx_uart_to_device.resubscribe();
//         let ch_rx_msgbus_to_device = ch_rx_msgbus_to_device.resubscribe();
//         join_set_spawn(
//             &mut task_set,
//             device.spawn(
//                 ch_tx_device_to_uart.clone(),
//                 ch_rx_uart_to_device,
//                 ch_rx_msgbus_to_device,
//                 ch_rx_device_to_msgbus.clone(),
//             ),
//         );
//     }

//     // Задача передачи сообщений на выход компонента -----------------------------------------------
//     let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
//         input: ch_tx_device_to_msgbus,
//         cmp_in_out: msg_bus,
//     };
//     join_set_spawn(
//         &mut task_set,
//         task.spawn().map_err(super::Error::TaskMpscToMsgBus),
//     );

//     // Ожидание выполнения -------------------------------------------------------------------------
//     while let Some(res) = task_set.join_next().await {
//         res??;
//     }

//     Ok(())
// }
