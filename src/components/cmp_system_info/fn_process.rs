use sysinfo::{Components, Disks, Networks, System};
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, Error, SystemInfo, SystemInfoDisk, SystemInfoNetwork};

const B_IN_MB: f32 = 1048576.0;

const B_IN_GB: f32 = 1073741824.0;

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut sys = System::new_all();
    let mut system_info = SystemInfo {
        ..Default::default()
    };

    system_info.os_version =
        System::long_os_version().ok_or(Error::CannotDefine("os_version".to_string()))?;

    system_info.host_name =
        System::host_name().ok_or(Error::CannotDefine("host_name".to_string()))?;

    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        system_info.networks.insert(
            interface_name.to_string(),
            SystemInfoNetwork {
                name: interface_name.to_string(),
                mac_address: data.mac_address().to_string(),
            },
        );
    }

    let msgbus_output = msgbus_linker.output();
    msgbus_linker.close();

    loop {
        sys.refresh_all();

        // Memory
        system_info.memory.total_memory_mb = sys.total_memory() as f32 / B_IN_MB;
        system_info.memory.used_memory_mb = sys.used_memory() as f32 / B_IN_MB;
        system_info.memory.total_swap_mb = sys.total_swap() as f32 / B_IN_MB;
        system_info.memory.used_swap_mb = sys.used_swap() as f32 / B_IN_MB;

        // CPU usage
        let cpus = sys
            .cpus()
            .iter()
            .map(|c| c.cpu_usage())
            .collect::<Vec<f32>>();
        system_info.cpu_usage = cpus;

        for disk in Disks::new_with_refreshed_list().iter() {
            let used_space_gb = (disk.total_space() - disk.available_space()) as f32 / B_IN_GB;
            let total_space_gb = disk.total_space() as f32 / B_IN_GB;
            let name = disk
                .name()
                .to_str()
                .ok_or(Error::CannotDefine("disk.name".to_string()))?
                .to_string();
            system_info.disks.insert(
                name.clone(),
                SystemInfoDisk {
                    name,
                    used_space_gb,
                    total_space_gb,
                },
            );
        }

        // температура компонентов
        system_info.temperatures = Components::new_with_refreshed_list()
            .iter()
            .map(|c| (c.label().to_string(), c.temperature()))
            .collect();

        let msgs = (config.fn_output)(&system_info);
        for msg in msgs {
            msgbus_output
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }

        sleep(config.period).await;
    }
}
