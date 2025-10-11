// #[allow(dead_code)]

#[cfg(all(
    wasm32_unknown_unknown,
    feature = "cmp_leptos",
    feature = "log_webconsole"
))]
fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use any_spawner::Executor;
    use leptos::{prelude::*, task::spawn_local};
    use reactive_stores::Store;
    use tokio::task::LocalSet;

    use rsiot::{
        components::cmp_leptos,
        executor::{ComponentExecutor, ComponentExecutorConfig},
        logging::{LogConfig, LogConfigFilter},
        message::example_message::*,
    };

    #[component]
    fn App() -> impl IntoView {}

    LogConfig {
        filter: LogConfigFilter::String("info"),
    }
    .run()?;

    #[derive(Default, Clone, Store)]
    struct GlobalStore {}

    impl cmp_leptos::StoreBound for GlobalStore {}

    // cmp_leptos ------------------------------------------------------------------------------
    let config_leptos = cmp_leptos::Config {
        body_component: || view! { <App/>}.into_any(),
        input_store: GlobalStore::default(),
        output_store: GlobalStore::default(),
        fn_input: |_, _| (),
        fn_output: |_, _| (),
    };

    // config_executor -------------------------------------------------------------------------
    let config_executor = ComponentExecutorConfig {
        buffer_size: 100,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
        fn_tokio_metrics: |_| None,
    };

    // executor --------------------------------------------------------------------------------
    Executor::init_wasm_bindgen().expect("executor should only be initialized once");

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

#[cfg(not(all(
    wasm32_unknown_unknown,
    feature = "cmp_leptos",
    feature = "log_webconsole"
)))]
fn main() {
    unimplemented!()
}
