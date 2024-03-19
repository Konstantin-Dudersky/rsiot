use slint::android::{init, AndroidApp};
use std::env;
use std::{sync::Arc, time::Duration};

use message::*;
use rsiot::{
    components::{cmp_http_client, cmp_inject_periodic, cmp_logger, cmp_slint},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::*,
};
use slint::{ComponentHandle, SharedString, Weak};
use tokio::{sync::Mutex, task::LocalSet};
use tracing::Level;
use url::Url;

mod message;

slint::include_modules!();

#[no_mangle]
fn android_main(app: AndroidApp) {
    init(app).unwrap();

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let main_window = MainWindow::new().unwrap();

    let main_window_link = main_window.as_weak();

    std::thread::spawn(move || main_executor(main_window_link));
    main_window.run().unwrap();
}

#[tokio::main]
async fn main_executor(slint_inst: Weak<MainWindow>) {
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "test_slint".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            // let msg = Message::new_custom(Custom::ValueInstantF64(counter));
            // counter += 1.0;
            // vec![msg]
            vec![]
        },
    };

    let slint_config = cmp_slint::Config {
        instance: Arc::new(Mutex::new(slint_inst)),
        fn_input: |msg, window| match msg.data {
            MsgData::Custom(Custom::BootButton(value)) => window
                .upgrade_in_event_loop(move |h| {
                    h.global::<GlobalData>()
                        .set_primary(SharedString::from(value.to_string()));
                })
                .unwrap(),
            _ => (),
        },
        fn_output: |window, tx| {
            // window
            //     .upgrade_in_event_loop(move |handle| {
            //         let global = handle.global::<GlobalData>();
            //         global.on_button(move |value| {
            //             let msg =
            //                 Message::new_custom(Custom::ValueInstantString(value.to_string()));
            //             tx.blocking_send(msg).unwrap();
            //         });
            //     })
            //     .unwrap();
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let http_client_config = cmp_http_client::http_client_config::Config {
        connection_config: cmp_http_client::http_client_config::ConnectionConfig {
            base_url: Url::parse("http://192.168.71.1:80").unwrap(),
        },
        requests_input: vec![cmp_http_client::http_client_config::RequestInput {
            fn_input: |msg| match msg.data {
                MsgData::Custom(Custom::SetRelayState(_)) => {
                    let param = cmp_http_client::http_client_config::HttpParam::Put {
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
        requests_periodic: vec![cmp_http_client::http_client_config::RequestPeriodic {
            period: Duration::from_secs(1),
            http_param: cmp_http_client::http_client_config::HttpParam::Get {
                endpoint: "messages".into(),
            },
            on_success: |data| {
                let msgs: Vec<Message<Custom>> = serde_json::from_str(data)?;
                Ok(msgs)
            },
            on_failure: || vec![],
        }],
    };

    let set = LocalSet::new();
    set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .add_cmp(cmp_slint::Cmp::new(slint_config))
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .add_cmp(cmp_http_client::Cmp::new(http_client_config))
            .wait_result()
            .await
            .unwrap();
    });
    set.await;
}
