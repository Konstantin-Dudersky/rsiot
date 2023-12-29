use redis::AsyncCommands;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use rsiot_component_core::{Input, Output};
use rsiot_messages_core::{IMessage, IMessageChannel};

use crate::{config::Config, error::Error};

pub async fn fn_process<TMessage, TMessageChannel>(
    input: Input<TMessage>,
    _output: Output<TMessage>,
    config: Config<TMessage, TMessageChannel>,
) where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    info!("Initialization. Config: {:?}", config);

    loop {
        info!("Starting");

        let result =
            task_main::<TMessage, TMessageChannel>(input.resubscribe(), config.clone()).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        info!("Restarting...");
        sleep(Duration::from_secs(2)).await;
    }
}

async fn task_main<TMessage, TMessageChannel>(
    mut input: Input<TMessage>,
    config: Config<TMessage, TMessageChannel>,
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
