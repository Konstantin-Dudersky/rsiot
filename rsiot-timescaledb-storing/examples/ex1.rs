use tokio::{main, time::Duration};
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_inject_periodic;
use rsiot_messages_core::{msg_types, ExampleMessage};
use rsiot_timescaledb_storing::cmp_timescaledb_storing;

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let url = Url::parse("postgres://postgres:postgres@localhost:5432/db_data_test").unwrap();

    let mut counter = 0.0;

    let mut chain = ComponentChain::new(100)
        .add_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = ExampleMessage::ValueInstantF64(msg_types::Value::new(counter));
                counter += 1.0;
                vec![msg]
            },
        }))
        .add_cmp(cmp_timescaledb_storing::new(
            cmp_timescaledb_storing::Config {
                connection_string: url,
            },
        ));

    chain.spawn().await;
}
