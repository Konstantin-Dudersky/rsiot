use redis::AsyncCommands;
use tokio::{
    spawn,
    sync::broadcast,
    time::{sleep, Duration},
};
use tracing::{error, info};

use rsiot_component_core::{StreamInput, StreamOutput};
use rsiot_extra_components::cmpbase_mpsc_to_broadcast;
use rsiot_messages_core::{IMessage, IMessageChannel};

use crate::{cmp_redis_publisher, error::Error};

pub async fn fn_process<TMessage, TMessageChannel>(
    input: StreamInput<TMessage>,
    _output: StreamOutput<TMessage>,
    config: cmp_redis_publisher::Config<TMessage, TMessageChannel>,
) where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    info!("Initialization. Config: {:?}", config);

    // Создаем канал для пересылки сообщений со входа потокам
    let (input_broadcast_tx, _input_broadcast_rx) = broadcast::channel::<TMessage>(100);
    let future = cmpbase_mpsc_to_broadcast::new(input, input_broadcast_tx.clone());
    let _task_to_output = spawn(future);

    loop {
        info!("Starting");

        let result =
            task_main::<TMessage, TMessageChannel>(input_broadcast_tx.subscribe(), config.clone())
                .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage, TMessageChannel>(
    mut input: broadcast::Receiver<TMessage>,
    config: cmp_redis_publisher::Config<TMessage, TMessageChannel>,
) -> Result<(), Error>
where
    TMessage: IMessage,
    TMessageChannel: IMessageChannel,
{
    let client = redis::Client::open(config.url.to_string())?;
    let mut connection = client.get_async_connection().await?;
    while let Ok(msg) = input.recv().await {
        let channels = (config.fn_input)(&msg);
        for channel in channels {
            let json = msg.to_json()?;
            let channel = channel.to_string();
            connection.hset(&channel, msg.key(), &json).await?;
            connection.publish(&channel, &json).await?;
        }
    }
    Ok(())
}
