mod app;

use leptos::*;
use rsiot::{
    components::cmp_leptos,
    executor::{ComponentExecutor, ComponentExecutorConfig},
};
use tokio::task::LocalSet;

use message::*;

use app::*;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();

    let config_leptos = cmp_leptos::Config {
        body_component: || view! { <App/> },
        hostname: "localhost".into(),
    };

    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "android_tauri".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let context = LocalSet::new();
    context.spawn_local(async move {
        ComponentExecutor::<Custom>::new(config_executor)
            .add_cmp(cmp_leptos::Cmp::new(config_leptos))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    spawn_local(context);
    Ok(())
}
