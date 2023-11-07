use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};
use url::Url;

use rsiot_messages_core::IMessage;
use rsiot_timescaledb_storing::{start_timescaledb_storing, Row};

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

    let (channel_send, channel_rcv) = mpsc::channel::<Message>(128);

    let mut counter = 0;
    let _task_simulation = spawn(async move {
        loop {
            channel_send
                .send(Message::Message0(counter as f64))
                .await
                .unwrap();
            counter += 1;
            sleep(Duration::from_secs(2)).await;
        }
    });

    let task_storing =
        spawn(start_timescaledb_storing(channel_rcv, config, url));

    task_storing.await.unwrap();
}

fn config(msg: Message) -> Option<Row> {
    let entity = msg.key();
    let ts_now = Utc::now().into();
    match msg {
        Message::Message0(value) => Some(Row::new(ts_now, &entity, "", value)),
    }
}
