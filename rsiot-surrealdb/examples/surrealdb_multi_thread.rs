//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-surrealdb --example surrealdb_multi_thread
//! ```

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use cmp_surrealdb::InputConfig;
    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::cmp_inject_periodic;
    use rsiot_messages_core::{
        msg_meta, Deserialize, IMessage, IMsgContentValue, MsgContent, MsgMeta, Serialize,
    };
    use rsiot_surrealdb as cmp_surrealdb;
    use tracing::info;

    #[derive(Clone, Debug, Deserialize, MsgMeta, PartialEq, Serialize)]
    enum Message {
        Request(MsgContent<u16>),
    }

    impl IMessage for Message {
        fn into_eav(self) -> Vec<rsiot_messages_core::eav::EavModel> {
            vec![]
        }
    }

    tracing_subscriber::fmt().init();

    let surrealdb_config = cmp_surrealdb::Config {
        host: "127.0.0.1".into(),
        port: 8003,
        user: "root".into(),
        password: "root".into(),
        namespace: "rsiot".into(),
        database: "rsiot".into(),
        init_script: include_str!("./init.surql").into(),
        input_config: vec![InputConfig {
            fn_input: |msg| match msg {
                Message::Request(content) => {
                    let value = content.value;
                    let query = include_str!("./new_value_int.surql");
                    let query = query
                        .replace("$ts", &content.ts.to_rfc3339())
                        .replace("$value_float", &format!("{:.2}", value));
                    Some(query)
                }
            },
            fn_on_success: |response| {
                info!("Response: {response}");
                vec![]
            },
            fn_on_failure: Vec::new,
        }],
    };

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::Request(MsgContent::new(counter));
            counter += 1;
            vec![msg]
        },
    };

    ComponentExecutor::<Message>::new(100, "surrealdb_multi_thread")
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_surrealdb::Cmp::new(surrealdb_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
