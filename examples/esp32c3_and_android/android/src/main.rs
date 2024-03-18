mod app;

use std::time::Duration;

use rsiot::{
    components::{cmp_http_client_wasm, cmp_leptos, cmp_webstorage},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    logging::configure_logging,
    message::*,
};
use tokio::task::LocalSet;
use tracing::info;
use url::Url;

use app::*;
use leptos::*;
use message::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    configure_logging("info");
    console_error_panic_hook::set_once();

    let http_client_config = cmp_http_client_wasm::http_client_config::Config {
        connection_config: cmp_http_client_wasm::http_client_config::ConnectionConfig {
            base_url: Url::parse("http://192.168.71.1:80").unwrap(),
        },
        requests_input: vec![cmp_http_client_wasm::http_client_config::RequestInput {
            fn_input: |msg| match msg.data {
                MsgData::Custom(Custom::SetRelayState(_)) => {
                    let param = cmp_http_client_wasm::http_client_config::HttpParam::Put {
                        endpoint: "messages1".into(),
                        body: msg.serialize().unwrap(),
                    };
                    Some(param)
                }
                _ => None,
            },
            on_success: |_| Ok(vec![]),
            on_failure: || vec![],
        }],
        requests_periodic: vec![cmp_http_client_wasm::http_client_config::RequestPeriodic {
            period: Duration::from_secs(1),
            http_param: cmp_http_client_wasm::http_client_config::HttpParam::Get {
                endpoint: "messages".into(),
            },
            on_success: |data| {
                let msgs: Vec<Message<Custom>> = serde_json::from_str(data)?;
                info!("{:?}", data);
                Ok(msgs)
            },
            on_failure: || vec![],
        }],
    };

    let leptos_config = cmp_leptos::Config {
        body_component: || view! { <App/> },
        hostname: "127.0.0.1:1420".into(),
    };

    let webstorage_config = cmp_webstorage::Config {
        kind: cmp_webstorage::ConfigKind::SessionStorage,
        fn_input: |msg| Some(msg),
        fn_output: |_| None,
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "android_app".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let context = LocalSet::new();
    context.spawn_local(async move {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_leptos::Cmp::new(leptos_config))
            .add_cmp(cmp_webstorage::Cmp::new(webstorage_config))
            .add_cmp(cmp_http_client_wasm::Cmp::new(http_client_config))
            .wait_result()
            .await
            .unwrap()
    });
    spawn_local(context);
}
