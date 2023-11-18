use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_channel_utils::{component_inject_periodic, component_logger};
use rsiot_messages_core::IMessage;
use tracing::Level;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main1() {
    tracing_subscriber::fmt().init();

    let (stream_origin, stream_into_logger) = mpsc::channel::<Message>(10);

    let mut counter = 0.0;
    let task_inject_periodic = spawn(component_inject_periodic(
        stream_origin,
        Duration::from_secs(2),
        move || {
            let msg = Message::Message0(counter);
            counter += 1.0;
            vec![msg]
        },
    ));

    let _task_logger =
        spawn(component_logger(stream_into_logger, None, Level::INFO));

    task_inject_periodic.await.unwrap();
}

use rsiot_channel_utils::{
    create_inject_periodic, create_logger, CompInjectPeriodicConfig,
};
use rsiot_component_core::ComponentPipeBuilder;

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut counter = 0.0;
    let pipe = ComponentPipeBuilder::<Message>::new(100)
        .begin(create_inject_periodic(CompInjectPeriodicConfig {
            period: Duration::from_secs(2),
            fn_periodic: move || {
                let msg = Message::Message0(counter);
                counter += 1.0;
                vec![msg]
            },
        }))
        .end(Box::new(create_logger(2)));

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}
