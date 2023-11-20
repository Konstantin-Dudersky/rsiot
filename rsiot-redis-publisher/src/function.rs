use redis::AsyncCommands;
use tokio::{
    spawn,
    sync::broadcast,
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_channel_utils::cmpbase_mpsc_to_broadcast;
use rsiot_component_core::{StreamInput, StreamOutput};
use rsiot_messages_core::IMessage;

use crate::{cmp_redis_publisher, Error};

pub async fn function<TMessage>(
    input: StreamInput<TMessage>,
    _output: StreamOutput<TMessage>,
    config: cmp_redis_publisher::Config,
) where
    TMessage: IMessage + 'static,
{
    info!("Initialization. Config: {:?}", config);

    // Создаем канал для пересылки сообщений со входа потокам
    let (input_broadcast_tx, _input_broadcast_rx) =
        broadcast::channel::<TMessage>(100);
    let future =
        cmpbase_mpsc_to_broadcast::create(input, input_broadcast_tx.clone());
    let _task_to_output = spawn(future);

    loop {
        info!("Starting");

        let result = task_main::<TMessage>(
            input_broadcast_tx.subscribe(),
            config.clone(),
        )
        .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage>(
    mut input: broadcast::Receiver<TMessage>,
    config: cmp_redis_publisher::Config,
) -> Result<(), Error>
where
    TMessage: IMessage,
{
    let client = redis::Client::open(config.url.to_string())?;
    let mut connection = client.get_async_connection().await?;
    while let Ok(msg) = input.recv().await {
        let json = msg.to_json()?;
        connection
            .hset(&config.redis_channel, msg.key(), &json)
            .await?;
        connection.publish(&config.redis_channel, &json).await?;
    }
    Ok(())
}
