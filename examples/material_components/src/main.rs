mod app;
mod leptos_components;
mod material_components;
mod messages;
mod plc;

use std::time::Duration;

use any_spawner::Executor;
use leptos::{prelude::*, task::spawn_local};
use rsiot::{
    components::{cmp_leptos, cmp_plc, cmp_webstorage},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::{example_service::Service, Message},
};
use tokio::task::LocalSet;

use app::*;
use messages::*;

fn main() -> anyhow::Result<()> {
    configure_logging("info").unwrap();

    // cmp_leptos ----------------------------------------------------------------------------------
    let config_leptos = cmp_leptos::Config {
        body_component: || view! { <App/> },
        hostname: "localhost".into(),
    };

    // cmp_plc -------------------------------------------------------------------------------------
    let config_plc = cmp_plc::Config {
        fn_cycle_init: |input: &mut plc::fb_main::I| {
            input.motor_hmi_command = Default::default();
            input.valve_analog_hmi_command = Default::default();
            input.valve_hmi_command = Default::default();
        },
        fn_input: |input: &mut plc::fb_main::I, msg: &Message<Custom>| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            match msg {
                Custom::m1_command(data) => input.motor_hmi_command = data,
                Custom::valve_analog_command(data) => input.valve_analog_hmi_command = data,
                Custom::valve_hmi_command(data) => input.valve_hmi_command = data,
                _ => (),
            }
        },
        fn_output: |output: &plc::fb_main::Q| {
            vec![
                Message::new_custom(Custom::m1_status(output.motor_hmi_status)),
                Message::new_custom(Custom::valve_analog_status(output.valve_analog_hmi_status)),
                Message::new_custom(Custom::valve_hmi_status(output.valve_hmi_status)),
            ]
        },
        fb_main: plc::fb_main::FB::new(Duration::from_millis(500)),
        period: Duration::from_millis(500),
        retention: None,
    };

    // cmp_webstorage ------------------------------------------------------------------------------
    let config_webstorage = cmp_webstorage::Config {
        fn_input: Some,
        fn_output: |_| None,
        default_messages: vec![],
        storage_kind: cmp_webstorage::ConfigStorageKind::SessionStorage,
    };

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    Executor::init_wasm_bindgen().expect("executor should only be initialized once");

    let context = LocalSet::new();

    context.spawn_local(async move {
        ComponentExecutor::<Custom>::new(config_executor)
            .add_cmp(cmp_leptos::Cmp::new(config_leptos))
            .add_cmp(cmp_plc::Cmp::new(config_plc))
            .add_cmp(cmp_webstorage::Cmp::new(config_webstorage))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });

    spawn_local(context);

    Ok(())
}
