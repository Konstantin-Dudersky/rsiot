mod app;

use std::time::Duration;

use leptos::*;
use rsiot::{
    components::{cmp_http_client_wasm, cmp_leptos},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::*,
};
use tokio::task::LocalSet;
use url::Url;

use message::*;

use app::*;

fn main() -> anyhow::Result<()> {
    console_error_panic_hook::set_once();
    configure_logging("info").unwrap();

    let config_leptos = cmp_leptos::Config {
        body_component: || view! { <App/> },
        hostname: "localhost".into(),
    };

    // let config_http_client_wasm = cmp_http_client_wasm::Config {
    //     connection_config: cmp_http_client_wasm::ConnectionConfig {
    //         base_url: Url::parse("http://192.168.71.1").unwrap(),
    //     },
    //     requests_input: vec![],
    //     requests_periodic: vec![cmp_http_client_wasm::RequestPeriodic {
    //         period: Duration::from_secs(1),
    //         http_param: cmp_http_client_wasm::HttpParam::Get {
    //             endpoint: "messages".into(),
    //         },
    //         on_success: |data| {
    //             let msgs: Vec<Message<Custom>> = serde_json::from_str(data)?;
    //             Ok(msgs)
    //         },
    //         on_failure: || vec![],
    //     }],
    // };

    let config_http_client_wasm = cmp_http_client_wasm::Config {
        connection_config: cmp_http_client_wasm::ConnectionConfig {
            base_url: Url::parse("http://192.168.71.1").unwrap(),
        },
        requests_input: vec![],
        requests_periodic: vec![cmp_http_client_wasm::RequestPeriodic {
            period: Duration::from_secs(1),
            http_param: cmp_http_client_wasm::HttpParam::Get {
                endpoint: "messages".into(),
            },
            on_success: |data| {
                let msgs: Vec<Message<Custom>> = serde_json::from_str(data)?;
                // let msg = Message::new_custom(Custom::Gpio0Button(data.to_string()));
                Ok(msgs)
            },
            on_failure: || vec![],
        }],
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
            .add_cmp(cmp_http_client_wasm::Cmp::new(config_http_client_wasm))
            .wait_result()
            .await?;
        Ok(()) as anyhow::Result<()>
    });
    spawn_local(context);
    Ok(())
}
