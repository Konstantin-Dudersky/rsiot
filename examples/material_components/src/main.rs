mod app;
mod components;
mod material_components;
mod messages;
mod plc;

use std::time::Duration;

use leptos::*;
use rsiot::{
    components::{cmp_leptos, cmp_plc},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::Message,
};
use tokio::task::LocalSet;

use app::*;
use messages::*;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();

    configure_logging("info").unwrap();

    // cmp_leptos ----------------------------------------------------------------------------------
    let config_leptos = cmp_leptos::Config {
        body_component: || view! { <App/> },
        hostname: "localhost".into(),
    };

    // cmp_plc -------------------------------------------------------------------------------------
    let config_plc = cmp_plc::Config {
        fn_cycle_init: |input: &mut plc::fb_main::I| {
            input.m1_command = Default::default();
        },
        fn_input: |input: &mut plc::fb_main::I, msg: &Message<Custom>| {
            let Some(msg) = msg.get_custom_data() else {
                return;
            };
            match msg {
                Custom::m1_command(data) => input.m1_command = data,
                _ => (),
            }
        },
        fn_output: |output: &plc::fb_main::Q| {
            vec![Message::new_custom(Custom::m1_status(output.m1_status))]
        },
        fb_main: plc::fb_main::FB::new(),
        period: Duration::from_secs(2),
        retention: None,
    };

    // executor ------------------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "material_components".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let context = LocalSet::new();
    context.spawn_local(async move {
        ComponentExecutor::<Custom>::new(config_executor)
            .add_cmp(cmp_leptos::Cmp::new(config_leptos))
            .add_cmp(cmp_plc::Cmp::new(config_plc))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    spawn_local(context);
    Ok(())
}
