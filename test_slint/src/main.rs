use std::env;
use std::{sync::Arc, time::Duration};

use example_service::Service;
use rsiot::components::cmp_logger;
use rsiot::{
    components::{cmp_raspberrypi_gpio, cmp_slint, cmp_system_info},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::*,
};
use slint::{ComponentHandle, SharedString, Weak};
use tokio::sync::Mutex;
use tracing::{info, Level};

slint::include_modules!();
fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let main_window = MainWindow::new().unwrap();

    let main_window_link = main_window.as_weak();
    main_window
        .global::<VirtualKeyboardHandler>()
        .on_key_pressed({
            let weak = main_window_link.clone();
            move |key| {
                weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyPressed { text: key.clone() });
                weak.unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::KeyReleased { text: key });
            }
        });

    std::thread::spawn(move || main_executor(main_window_link));
    main_window.run().unwrap();
}

mod message;
use message::*;

#[tokio::main]
async fn main_executor(slint_inst: Weak<MainWindow>) {
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    let slint_config =
        cmp_slint::Config {
            instance: Arc::new(Mutex::new(slint_inst)),
            fn_input: |msg, window| match msg.data {
                MsgData::Custom(data) => match data {
                    Custom::HostName(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_host_name(SharedString::from(value.to_string()));
                        })
                        .unwrap(),
                    Custom::OsVesion(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_os_version(SharedString::from(value.to_string()));
                        })
                        .unwrap(),
                    Custom::Eth0Mac(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_eth0_mac(SharedString::from(value.to_string()));
                        })
                        .unwrap(),
                    Custom::Wlan0Mac(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_wlan0_mac(SharedString::from(value.to_string()));
                        })
                        .unwrap(),
                    Custom::CpuUsage(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_cpu_load(SharedString::from(value));
                        })
                        .unwrap(),
                    Custom::CpuTemp(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_cpu_temp(SharedString::from(value));
                        })
                        .unwrap(),
                    // память
                    Custom::Memory(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_memory(SharedString::from(value));
                        })
                        .unwrap(),
                    Custom::Swap(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>().set_swap(SharedString::from(value));
                        })
                        .unwrap(),
                    // диски
                    Custom::DiskDevSda1(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_dev_sda_1(SharedString::from(value));
                        })
                        .unwrap(),
                    Custom::DiskDevSda2(value) => window
                        .upgrade_in_event_loop(move |h| {
                            h.global::<GlobalData>()
                                .set_dev_sda_2(SharedString::from(value));
                        })
                        .unwrap(),
                    Custom::GpioTab(value) => {
                        if !value {
                            return;
                        }
                        let key: SharedString = slint::platform::Key::Tab.into();
                        window
                            .upgrade_in_event_loop(move |h| {
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyPressed { text: key.clone() },
                                );
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyReleased { text: key },
                                )
                            })
                            .unwrap()
                    }
                    Custom::Gpio1(value) => {
                        if !value {
                            return;
                        }
                        // let key: SharedString = slint::platform::Key;
                        let key: SharedString = SharedString::from("1");
                        window
                            .upgrade_in_event_loop(move |h| {
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyPressed { text: key.clone() },
                                );
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyReleased { text: key },
                                )
                            })
                            .unwrap()
                    }
                    Custom::Gpio2(value) => {
                        if !value {
                            return;
                        }
                        // let key: SharedString = slint::platform::Key;
                        let key: SharedString = SharedString::from("2");
                        window
                            .upgrade_in_event_loop(move |h| {
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyPressed { text: key.clone() },
                                );
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyReleased { text: key },
                                )
                            })
                            .unwrap()
                    }
                    Custom::GpioBackspace(value) => {
                        if !value {
                            return;
                        }
                        let key: SharedString = slint::platform::Key::Backspace.into();
                        // let key: SharedString = SharedString::from("1");
                        window
                            .upgrade_in_event_loop(move |h| {
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyPressed { text: key.clone() },
                                );
                                h.window().dispatch_event(
                                    slint::platform::WindowEvent::KeyReleased { text: key },
                                )
                            })
                            .unwrap()
                    }
                },
                _ => (),
            },
            fn_output: |_window, _tx| {},
        };

    let config_system_info = cmp_system_info::Config {
        period: Duration::from_secs(2),
        fn_output: |info| {
            vec![
                Message::new_custom(Custom::HostName(info.host_name.clone())),
                Message::new_custom(Custom::OsVesion(info.os_version.clone())),
                Message::new_custom(Custom::Eth0Mac(
                    info.networks.get("eth0").unwrap().mac_address.clone(),
                )),
                Message::new_custom(Custom::Wlan0Mac(
                    info.networks.get("wlan0").unwrap().mac_address.clone(),
                )),
                Message::new_custom(Custom::CpuUsage(format!(
                    "{:.1} | {:.1} | {:.1} | {:.1} %",
                    info.cpu_usage[0], info.cpu_usage[1], info.cpu_usage[2], info.cpu_usage[3],
                ))),
                Message::new_custom(Custom::CpuTemp(format!(
                    "{:.1} ℃",
                    info.temperatures.get("cpu_thermal temp1").unwrap()
                ))),
                // память
                Message::new_custom(Custom::Memory(format!(
                    "{:.0} MB / {:.0} MB",
                    info.memory.used_memory_mb, info.memory.total_memory_mb
                ))),
                Message::new_custom(Custom::Swap(format!(
                    "{:.0} MB / {:.0} MB",
                    info.memory.used_swap_mb, info.memory.total_swap_mb
                ))),
                // диски
                Message::new_custom(Custom::DiskDevSda1(format!(
                    "{:.1} GB / {:.1} GB",
                    info.disks.get("/dev/sda1").unwrap().used_space_gb,
                    info.disks.get("/dev/sda1").unwrap().total_space_gb
                ))),
                Message::new_custom(Custom::DiskDevSda2(format!(
                    "{:.1} GB / {:.1} GB",
                    info.disks.get("/dev/sda2").unwrap().used_space_gb,
                    info.disks.get("/dev/sda2").unwrap().total_space_gb
                ))),
            ]
        },
    };

    // cmp_raspberrypi_gpio ------------------------------------------------------------------------
    let config_raspberrypy = cmp_raspberrypi_gpio::Config {
        inputs: vec![
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 17,
                fn_output: |value| Message::new_custom(Custom::GpioTab(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 27,
                fn_output: |value| Message::new_custom(Custom::Gpio1(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 22,
                fn_output: |value| Message::new_custom(Custom::Gpio2(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
            cmp_raspberrypi_gpio::ConfigInput {
                pin_number: 23,
                fn_output: |value| Message::new_custom(Custom::GpioBackspace(value)),
                pull_mode: cmp_raspberrypi_gpio::PullMode::Down,
            },
        ],
        outputs: vec![],
    };

    // cmp_logger ----------------------------------------------------------------------------------
    let _config_logger = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| {
            let Some(msg) = msg.get_custom_data() else {
                return Ok(None);
            };
            let text = match msg {
                Custom::GpioTab(value) => format!("Gpio: {}", value),
                _ => return Ok(None),
            };
            Ok(Some(text))
        },
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_slint::Cmp::new(slint_config))
        .add_cmp(cmp_system_info::Cmp::new(config_system_info))
        .add_cmp(cmp_raspberrypi_gpio::Cmp::new(config_raspberrypy))
        // .add_cmp(cmp_logger::Cmp::new(config_logger))
        .wait_result()
        .await
        .unwrap();
}
