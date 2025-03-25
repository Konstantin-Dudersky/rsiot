use std::time::Duration;

use leptos::{
    prelude::*,
    task::{spawn_local, Executor},
};
use reactive_stores::Store;
use tokio::task::LocalSet;

use crate::{
    components::cmp_leptos,
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::example_message::*,
};

#[test]
#[allow(dead_code)]
fn test_wasm() {
    fn main() -> anyhow::Result<()> {
        #[component]
        fn App() -> impl IntoView {}

        configure_logging("info").unwrap();

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
}
