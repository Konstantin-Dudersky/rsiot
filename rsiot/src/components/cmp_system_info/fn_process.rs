use sysinfo::{Components, Disks, Networks, System};
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, Error, SystemInfo, SystemInfoNetwork};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
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

        println!("=> system:");
        // RAM and swap information:
        println!("total memory: {} bytes", sys.total_memory());
        println!("used memory : {} bytes", sys.used_memory());
        println!("total swap  : {} bytes", sys.total_swap());
        println!("used swap   : {} bytes", sys.used_swap());

        // Number of CPUs:
        println!("NB CPUs: {}", sys.cpus().len());

        // We display all disks' information:
        println!("=> disks:");
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");
        }

        // Network interfaces name, total data received and total data transmitted:

        // Components temperature:
        let components = Components::new_with_refreshed_list();
        println!("=> components:");
        for component in &components {
            println!("{component:?}");
        }

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
