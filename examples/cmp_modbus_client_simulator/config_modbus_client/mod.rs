mod device_simulator;

use std::{
    net::{IpAddr, Ipv4Addr},
    time::Duration,
};

use rsiot::components::cmp_modbus_client::*;

use crate::message::*;

pub fn cmp() -> rsiot::executor::Component<Config<Msg>, Msg> {
    let config = Config {
        enabled: true,
        devices_comm_settings: vec![ConfigDevicesCommSettings {
            client_type: ClientType::Tcp {
                host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                port: 5020,
            },
            unit_id: 1,
            timeout: Duration::from_millis(200),
            concurrent_connections: 1,
        }],
        devices: vec![Box::new(device_simulator::Device {
            update_period: Duration::from_millis(500),
            fn_input: |msg, buffer| {
                if let Msg::ValueWrite(v) = msg {
                    buffer.value_write = *v
                }
            },
            fn_output: |buffer| {
                let msg = Msg::ValueRead(buffer.value_read);
                vec![msg]
            },
        })],
    };

    Cmp::new(config)
}
