//! Запуск:
//!
//! ```bash
//! cargo run -p rsiot-timescaledb-storing --example timescaledb-storing
//! ```

use tokio::{main, time::Duration};
use url::Url;

use rsiot_component_core::ComponentExecutor;
use rsiot_extra_components::cmp_inject_periodic;
use rsiot_messages_core::{msg_types, ExampleMessage};
use rsiot_timescaledb_storing::cmp_timescaledb_storing;

#[main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    let url = Url::parse("postgres://postgres:postgres@localhost:5432/db_data_test")?;
    let db_config = cmp_timescaledb_storing::Config {
        connection_string: url,
    };

    ComponentExecutor::new(100)
        .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
        .add_cmp(cmp_timescaledb_storing::Cmp::new(db_config))
        .wait_result()
        .await?;

    Ok(())
}