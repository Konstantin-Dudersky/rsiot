mod app;
mod message;
mod plc;

use std::time::Duration;

use leptos::*;
use rsiot::{
    component_core::ComponentExecutor,
    components::{cmp_leptos, cmp_plc},
    message::MsgContent,
};
use tokio::task::LocalSet;

use app::App;
use message::Message;
use plc::fb_main;

fn main() -> anyhow::Result<()> {
    let hostname = cmp_leptos::utils::define_hostname().unwrap();

    let leptos_config = cmp_leptos::Config {
        body_component: || view! { <App/> },
        hostname,
    };

    let plc_config = cmp_plc::Config {
        fn_input: |_input: &mut fb_main::I, msg: &Message| match msg {
            _ => (),
        },
        fn_output: |output: &fb_main::Q| {
            let counter_u16 = output.counter_u16_0_100;
            let counter_u16 = Message::U16_0_100(MsgContent::new(counter_u16));
            vec![counter_u16]
        },
        fb_main: fb_main::FB::new(),
        period: Duration::from_millis(2000),
    };

    let context = LocalSet::new();
    context.spawn_local(async move {
        ComponentExecutor::<Message>::new(100)
            .add_cmp(cmp_leptos::Cmp::new(leptos_config))
            .add_cmp(cmp_plc::Cmp::new(plc_config))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    spawn_local(context);
    Ok(())
}
 