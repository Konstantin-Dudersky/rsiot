use serde::{Deserialize, Serialize};
use tokio::{
    main, spawn,
    sync::mpsc,
    time::{sleep, Duration},
};

use rsiot_channel_utils::{component_cache, create_cache};
use rsiot_messages_core::IMessage;
use tracing::info;

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Message {
    Message0(f64),
    Message1(f64),
    CombineMessage(f64, f64),
}

impl IMessage for Message {}

#[main]
async fn main() {
    tracing_subscriber::fmt().init();

    let cache = create_cache::<Message>();

    let (msgs_origin, msgs_cache_input) = mpsc::channel::<Message>(1000);
    let (msgs_cache_output, _msgs_end) = mpsc::channel::<Message>(1000);

    let mut counter = 0.0;
    let _task_origin = spawn(async move {
        loop {
            let msg = Message::Message0(counter);
            counter += 1.0;
            msgs_origin.send(msg).await.unwrap();
            sleep(Duration::from_secs(2)).await;
        }
    });

    let cache_task = spawn(component_cache(
        msgs_cache_input,
        msgs_cache_output,
        cache.clone(),
    ));

    let _end_task = spawn(async move {
        loop {
            {
                let lock = cache.lock().unwrap();
                info!("cache: {:?}", lock);
            }
            sleep(Duration::from_secs(5)).await;
        }
    });

    cache_task.await.unwrap();
}
