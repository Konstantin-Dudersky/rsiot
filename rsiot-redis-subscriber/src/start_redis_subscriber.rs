use futures_util::StreamExt as _;
use tokio::{
    sync::mpsc::Sender,
    time::{sleep, Duration},
};
use tracing::{error, info, trace};
use url::Url;

use rsiot_messages_core::IMessage;

use crate::Error;

pub async fn start_redis_subscriber<TMessage>(
    url: Url,
    redis_channel: String,
    tx: Sender<TMessage>,
) -> ()
where
    TMessage: IMessage,
{
    loop {
        info!(
            "Start redis subscriber. Url: {}, channel: {}",
            url, redis_channel
        );
        let result = start_redis_subscriber_loop::<TMessage>(
            url.clone(),
            redis_channel.clone(),
            &tx,
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

pub async fn start_redis_subscriber_loop<TMessage>(
    url: Url,
    redis_channel: String,
    tx: &Sender<TMessage>,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    let client = redis::Client::open(url.to_string())?;
    let connection = client.get_async_connection().await?;
    let mut pubsub = connection.into_pubsub();
    pubsub.subscribe(redis_channel).await?;
    let mut stream = pubsub.on_message();
    loop {
        let msg = stream.next().await;
        let msg = match msg {
            Some(value) => value,
            None => return Err(Error::GetMessageError),
        };
        trace!("New message from Redis: {:?}", msg);
        let payload: String = msg.get_payload()?;
        let payload: TMessage = TMessage::from_str(&payload)?;
        tx.send(payload).await?
    }
}
