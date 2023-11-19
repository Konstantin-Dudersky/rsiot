use redis::AsyncCommands;
use tokio::{
    sync::mpsc::Receiver,
    time::{sleep, Duration},
};
use tracing::{error, info};
use url::Url;

use rsiot_messages_core::IMessage;

use crate::Error;

pub async fn start_redis_publisher<TMessage>(
    url: Url,
    redis_channel: String,
    mut channel_to_redis_rx: Receiver<TMessage>,
) where
    TMessage: IMessage,
{
    loop {
        info!(
            "Start Redis publisher. Url: {}, channel: {}",
            url, redis_channel
        );
        let result = start_redis_publisher_loop::<TMessage>(
            url.clone(),
            redis_channel.clone(),
            &mut channel_to_redis_rx,
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

async fn start_redis_publisher_loop<TMessage>(
    url: Url,
    redis_channel: String,
    channel_to_redis_rx: &mut Receiver<TMessage>,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    let client = redis::Client::open(url.to_string())?;
    let mut connection = client.get_async_connection().await?;
    while let Some(msg) = channel_to_redis_rx.recv().await {
        let json = msg.to_json()?;
        connection.hset(&redis_channel, msg.key(), &json).await?;
        connection.publish(&redis_channel, &json).await?;
    }
    Ok(())
}
