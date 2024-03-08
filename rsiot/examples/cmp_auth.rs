//! cargo run --package rsiot --example cmp_auth --features cmp_auth

#[cfg(feature = "cmp_auth")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use rsiot::{
        components::{cmp_auth, cmp_inject_periodic, cmp_logger},
        executor::{ComponentExecutor, ComponentExecutorConfig},
        message::{example_message::*, system_messages::*, *},
    };
    use tracing::Level;

    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
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
        store: cmp_auth::ConfigStore::Local(vec![cmp_auth::ConfigStoreItem {
            login: "admin".into(),
            password: "admin1".into(),
            role: AuthPermissions::Admin,
        }]),
    };

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "example_auth".into(),
        fn_auth: |msg, _| Some(msg),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_auth::Cmp::new(auth_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(feature = "cmp_auth"))]
fn main() {}
