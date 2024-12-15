//! Запуск:
//!
//! ```bash
//! cargo run  --example cmp_surrealdb --features "cmp_surrealdb" --target="x86_64-unknown-linux-gnu"
//! ```

#[cfg(feature = "cmp_surrealdb")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use tracing::info;

    use rsiot::{
        components::{
            cmp_inject_periodic,
            cmp_surrealdb::{self, RequestInputConfig},
        },
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_service::*, Deserialize, Message, MsgDataBound, Serialize},
    };

    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    enum Custom {
        Request(u16),
    }

    impl MsgDataBound for Custom {
        type TService = Service;
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
        request_input: vec![RequestInputConfig {
            fn_input: |msg| match msg.get_custom_data()? {
                Custom::Request(content) => {
                    let value = content;
                    let query = include_str!("./new_value_int.surql");
                    let query = query
                        .replace("$ts", &msg.ts.to_rfc3339())
                        .replace("$value_float", &format!("{:.2}", value));
                    Some(query)
                }
            },
            fn_on_success: |response| {
                info!("Response: {response:?}");
                Ok(vec![])
            },
            fn_on_failure: Vec::new,
        }],
        request_start: vec![],
    };

    let mut counter = 0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::Request(counter));
            counter += 1;
            vec![msg]
        },
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Custom, Service>::new(executor_config)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_surrealdb::Cmp::new(surrealdb_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_surrealdb"))]
fn main() {}
