use sysinfo::{Components, Disks, Networks, System};
use tokio::time::sleep;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{Config, Error, SystemInfo, SystemInfoDisk, SystemInfoNetwork};

const B_IN_MB: f32 = 1048576.0;

const B_IN_GB: f32 = 1073741824.0;

pub async fn fn_process<TMsg, TService>(
    config: Config<TMsg>,
    in_out: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let mut sys = System::new_all();
    let mut system_info = SystemInfo::default();

    system_info.os_version = match System::long_os_version() {
        Some(value) => value.to_string(),
        None => return raise_error("os_version"),
    };

    system_info.host_name = match System::host_name() {
        Some(value) => value.to_string(),
        None => return raise_error("host_name"),
    };

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
            let name = disk.name().to_str().unwrap().to_string();
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
            in_out.send_output(msg).await.unwrap();
        }

        sleep(config.period).await;
    }
}

fn raise_error(field: &str) -> super::Result<()> {
    let err = Error::CannotDefine {
        field: field.into(),
    };
    Err(err)
}
