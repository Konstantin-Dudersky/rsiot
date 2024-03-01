//! cargo run --package rsiot-auth --example auth

use std::time::Duration;

use rsiot_auth as cmp_auth;
use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::{cmp_inject_periodic, cmp_logger};
use rsiot_messages_core::{example_message::*, system_messages::*, *};
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
            password: "admin".into(),
            role: AuthPermissions::Admin,
        }]),
    };

    ComponentExecutor::<Custom>::new(100, "example_auth")
        .add_cmp(cmp_logger::Cmp::new(logger_config))
        .add_cmp(cmp_auth::Cmp::new(auth_config))
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_periodic_config))
        .wait_result()
        .await?;

    Ok(())
}
