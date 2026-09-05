use std::{process::Command, time::Duration};

use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};
use tracing::{error, info, warn};

use crate::executor::{join_set_spawn, join_set_spawn_blocking, sleep};

use super::{
    CanFilter, CanFrame, CanSettings, Error,
    task_recv_from_can::{RecvFromCanAsync, RecvFromCanSync},
    task_send_to_can::{SendToCanAsync, SendToCanSync},
};

pub struct TaskSetupSendRecv {
    pub input: broadcast::Receiver<CanFrame>,
    pub output: mpsc::Sender<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
    pub filters: Vec<CanFilter>,

    /// true - асинхронная версия
    ///
    /// Синхронная версия запускается в отдельных потоках, производительность в ~5 раз выше
    pub async_version: bool,
}

impl TaskSetupSendRecv {
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            // Настройка интерфейса через ip-link
            interface_setup(&self.ifname, &self.can_settings).await?;

            let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

            // Задача отправки кадров в CAN
            if self.async_version {
                let task = SendToCanAsync {
                    input: self.input.resubscribe(),
                    ifname: self.ifname.clone(),
                    can_settings: self.can_settings.clone(),
                };
                join_set_spawn(&mut task_set, "cmp_linux_can | send_to_can", task.spawn());
            } else {
                let task = SendToCanSync {
                    input: self.input.resubscribe(),
                    ifname: self.ifname.clone(),
                    can_settings: self.can_settings.clone(),
                };
                join_set_spawn_blocking(&mut task_set, "cmp_linux_can | send_to_can", || {
                    task.spawn()
                });
            }

            // Задача получения кадров из CAN

            if self.async_version {
                let task = RecvFromCanAsync {
                    output: self.output.clone(),
                    ifname: self.ifname.clone(),
                    can_settings: self.can_settings.clone(),
                    filters: self.filters.clone(),
                };
                join_set_spawn(&mut task_set, "cmp_linux_can | recv_from_can", task.spawn());
            } else {
                let task = RecvFromCanSync {
                    output: self.output.clone(),
                    ifname: self.ifname.clone(),
                    can_settings: self.can_settings.clone(),
                    filters: self.filters.clone(),
                };
                join_set_spawn_blocking(&mut task_set, "cmp_linux_can | recv_from_can", || {
                    task.spawn()
                });
            }

            while let Some(res) = task_set.join_next().await {
                warn!("{res:?}");
                task_set.abort_all();
            }

            sleep(Duration::from_millis(1_000)).await;
        }
    }
}

async fn interface_setup(ifname: &str, can_settings: &CanSettings) -> Result<(), Error> {
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

    // Длина очереди
    let cmd: Vec<_> = "ip link set can0 txqueuelen 256".split(" ").collect();
    let cmd = Command::new(cmd[0])
        .args(&cmd[1..])
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
