use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::{main, time::Duration};
use url::Url;

use rsiot_component_core::ComponentChain;
use rsiot_extra_components::cmp_inject_periodic;
use rsiot_messages_core::IMessage;
use rsiot_timescaledb_storing::cmp_timescaledb_storing::{self, Row};

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let url =
        Url::parse("postgres://postgres:postgres@localhost:5432/db_data_test")
            .unwrap();

    let mut counter = 0.0;

    let mut chain = ComponentChain::init(100)
        .start_cmp(cmp_inject_periodic::new(cmp_inject_periodic::Config {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .end_cmp(cmp_timescaledb_storing::new(
            cmp_timescaledb_storing::Config {
                fn_process,
                connection_string: url,
            },
        ));

    chain.spawn().await;
}

fn fn_process(msg: Message) -> Option<Row> {
    let entity = msg.key();
    let ts_now = Utc::now().into();
    match msg {
        Message::Message0(value) => Some(Row::new(ts_now, &entity, "", value)),
    }
}
