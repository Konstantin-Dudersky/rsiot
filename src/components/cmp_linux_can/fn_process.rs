use std::{process::Command, time::Duration};

use tokio::task::JoinSet;
use tracing::{error, info};

use crate::{
    components::shared_tasks::cmp_can_general::CanGeneralTasks,
    components_config::can_general::BufferBound,
    executor::{CmpInOut, join_set_spawn},
    message::MsgDataBound,
};

use super::{
    CanSettings, Config, Error, task_interface_info::InterfaceInfo,
    task_recv_from_can::RecvFromCan, task_send_to_can::SendToCan,
};

pub async fn fn_process<TMsg, TBuffer>(
    config: Config<TMsg, TBuffer>,
    msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TBuffer: 'static + BufferBound,
{
    // Настройка интерфейса через ip-link
    interface_setup(&config.ifname, &config.can_settings)?;

    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    let (ch_rx_send_to_can, ch_tx_recv_from_can) = CanGeneralTasks {
        msg_bus,
        buffer_default: config.buffer_default,
        buffer_size: 1000,
        task_set: &mut task_set,
        fn_input: config.fn_input,
        period: config.period,
        fn_periodic: config.fn_periodic,
        fn_output: config.fn_output,
        error_task_end_input: || Error::TaskEndInput,
        error_task_end_output: || Error::TaskEndOutput,
        error_tokio_mpsc_send: || Error::TokioSyncMpscSend,
    }
    .spawn();

    // Задача отправки кадров в CAN
    let task = SendToCan {
        input: ch_rx_send_to_can,
        ifname: config.ifname.clone(),
        can_settings: config.can_settings.clone(),
    };
    join_set_spawn(&mut task_set, "cmp_linux_can | send_to_can", task.spawn());

    // Задача получения кадров из CAN
    let task = RecvFromCan {
        output: ch_tx_recv_from_can,
        ifname: config.ifname.clone(),
        can_settings: config.can_settings,
        filters: config.filters,
    };
    join_set_spawn(&mut task_set, "cmp_linux_can | recv_from_can", task.spawn());

    let task = InterfaceInfo {
        ifname: config.ifname,
        period: Duration::from_millis(1000),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_linux_can | interface_info",
        task.spawn(),
    );

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::TaskEnd)
}

fn interface_setup(ifname: &str, can_settings: &CanSettings) -> Result<(), Error> {
    // Настройка интерфейса должна выполняться с правами суперпользователя
    sudo::escalate_if_needed().map_err(|e| Error::Sudo(e.to_string()))?;

    // Подключаемся к интерфейсу
    let interface =
        socketcan::CanInterface::open(ifname).map_err(|e| Error::InterfaceOpen(e.to_string()))?;

    // Останавливаем интерфейс
    interface
        .bring_down()
        .map_err(|e| Error::InterfaceDown(e.to_string()))?;

    // Формируем команду для настройки интерфейса через ip-link
    let command = can_settings.into_ip_link_command(ifname);
    let cmd = Command::new(&command[0])
        .args(&command[1..])
        .output()
        .map_err(Error::ProcessExecution)?;
    let err = cmd.stderr;
    if !err.is_empty() {
        error!("Command output: {:?}", String::from_utf8_lossy(&err));
        return Err(Error::TaskEnd);
    }

    // Запускаем интерфейс
    interface
        .bring_up()
        .map_err(|e| Error::InterfaceUp(e.to_string()))?;

    // Выводим информацию об интерфейсе
    let details = interface
        .details()
        .map_err(|e| Error::InterfaceDetails(e.to_string()))?;
    info!("CAN interface details: {:?}", details);

    // Выводим состояние интерфейса
    let state = interface
        .state()
        .map_err(|e| Error::InterfaceState(e.to_string()))?;
    info!("State: {:?}", state);

    Ok(())
}
