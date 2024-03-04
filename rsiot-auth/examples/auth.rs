//! cargo run --package rsiot-auth --example auth

#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::time::Duration;

    use rsiot_auth as cmp_auth;
    use rsiot_component_core::ComponentExecutor;
    use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
    use rsiot_messages_core::{example_message::*, system_messages::*, *};
    use tracing::Level;

    tracing_subscriber::fmt().init();

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let inject_periodic_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(4),
        fn_periodic: move || {
            let value = AuthLoginRequest {
                login: "admin".to_string(),
                password: "admin".to_string(),
            };
            let msg = message_new!("System-AuthLoginRequest::value");
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

    let executor_config = rsiot_component_core::ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "example_auth".into(),
        fn_auth: |_| None,
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_auth::Cmp::new(auth_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
        .wait_result()
        .await?;

    Ok(())
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn main() {}
