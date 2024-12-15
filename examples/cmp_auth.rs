//! cargo run --package rsiot --example cmp_auth --features cmp_auth

#[cfg(feature = "cmp_auth")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_auth, cmp_inject_periodic, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, example_service::*, system_messages::*, *},
    };
    use tracing::Level;

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        fn_input: |msg| Ok(Some(msg.serialize()?)),
    };

    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(4),
        fn_periodic: move || {
            let value = AuthRequestByLogin {
                login: "admin".to_string(),
                password: "admin".to_string(),
            };
            let msg = message_new!("System-AuthRequestByLogin::value");
            vec![msg]
        },
    };

    let auth_config = cmp_auth::Config {
        secret_key: "secret_key".into(),
        store: cmp_auth::ConfigStore::Local(vec![cmp_auth::ConfigStoreLocalItem {
            login: "admin".into(),
            password: "admin1".into(),
            role: AuthPermissions::Admin,
        }]),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        service: Service::example_service,
        fn_auth: |msg, _| Some(msg),
        delay_publish: Duration::from_millis(100),
    };

    ComponentExecutor::<Custom, Service>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_auth::Cmp::new(auth_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_auth"))]
fn main() {}
