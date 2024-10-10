use std::time::Duration;

use leptos::*;
use tokio::task::LocalSet;

use crate::{
    components::cmp_leptos,
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::{example_message::*, example_service::Service},
};

#[test]
#[allow(dead_code)]
fn test_wasm() {
    fn main() -> anyhow::Result<()> {
        #[component]
        fn App() -> impl IntoView {
            view! {}
        }
        configure_logging("info").unwrap();

        // cmp_leptos ------------------------------------------------------------------------------
        let config_leptos = cmp_leptos::Config {
            body_component: || view! { <App/> },
            hostname: "localhost".into(),
        };

        // config_executor -------------------------------------------------------------------------
        let config_executor = ComponentExecutorConfig {
            buffer_size: 100,
            service: Service::example_service,
            fn_auth: |msg, _| Some(msg),
            delay_publish: Duration::from_millis(100),
        };

        // executor --------------------------------------------------------------------------------
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
