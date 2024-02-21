use futures::StreamExt;
use redis::{AsyncCommands, Client};
use tokio::{
    task::JoinSet,
    time::{sleep, Duration},
};
use tracing::{error, info, trace, warn};

use rsiot_component_core::{Cache, CmpInput, CmpOutput, ComponentError};
use rsiot_messages_core::{message_v2::MsgDataBound, IMessageChannel};

use crate::{config::Config, error::Error};

type Result = std::result::Result<(), Error>;

pub async fn fn_process<TMessage, TMessageChannel>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage, TMessageChannel>,
    _cache: Cache<TMessage>,
) -> std::result::Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    info!("Initialization. Config: {:?}", config,);

    loop {
        info!("Starting");
        let result = task_main(input.clone(), output.clone(), config.clone()).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn task_main<TMessage, TMessageChannel>(
    input: CmpInput<TMessage>,
    output: CmpOutput<TMessage>,
    config: Config<TMessage, TMessageChannel>,
) -> Result
where
    TMessage: MsgDataBound + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    let mut set = JoinSet::new();
    set.spawn(task_subscription(output.clone(), config.clone()));
    set.spawn(task_read_hash(output.clone(), config.clone()));
    set.spawn(task_publication(input, config));
    while let Some(res) = set.join_next().await {
        res??;
    }
    Ok(())
}

/// Задача публикации в канале Pub/Sub, и сохранение в кеше.
async fn task_publication<TMessage, TMessageChannel>(
    mut input: CmpInput<TMessage>,
    config: Config<TMessage, TMessageChannel>,
) -> Result
where
    TMessage: MsgDataBound,
    TMessageChannel: IMessageChannel,
{
    let client = redis::Client::open(config.url.to_string())?;
    let mut connection = client.get_async_connection().await?;
    while let Ok(msg) = input.recv().await {
        let msg = match msg {
            Some(val) => val,
            None => continue,
        };
        let data = (config.fn_input)(&msg).map_err(Error::FnInput)?;
        let data = match data {
            Some(data) => data,
            None => continue,
        };
        for item in data {
            let channel = item.channel.to_string();
            let key = item.key;
            let value = item.value;
            connection.hset(&channel, key, &value).await?;
            connection.publish(&channel, &value).await?;
        }
    }
    Ok(())
}

/// Подписка на канал Pub/Sub
async fn task_subscription<TMessage, TMessageChannel>(
    output: CmpOutput<TMessage>,
    config: Config<TMessage, TMessageChannel>,
) -> Result
where
    TMessage: MsgDataBound,
    TMessageChannel: IMessageChannel,
{
    info!("Start redis subscription");
    let client = Client::open(config.url.to_string())?;
    let connection = client.get_async_connection().await?;
    let mut pubsub = connection.into_pubsub();
    pubsub
        .subscribe(config.subscription_channel.to_string())
        .await?;
    let mut stream = pubsub.on_message();
    while let Some(redis_msg) = stream.next().await {
        trace!("New message from Redis: {:?}", redis_msg);
        let payload: String = redis_msg.get_payload()?;
        let msgs = (config.fn_output)(&payload).map_err(Error::FnOutput)?;
        let msgs = match msgs {
            Some(msgs) => msgs,
            None => continue,
        };
        for msg in msgs {
            output.send(msg).await.map_err(Error::CmpOutput)?
        }
    }
    Err(Error::EndRedisSubscription)
}

/// Чтение данных из хеша
async fn task_read_hash<TMessage, TMessageChannel>(
    output: CmpOutput<TMessage>,
    config: Config<TMessage, TMessageChannel>,
) -> Result
where
    TMessage: MsgDataBound,
    TMessageChannel: IMessageChannel,
{
    info!("Start reading redis hash");

    let url = config.url.to_string();
    let redis_channel = config.subscription_channel.to_string();

    let client = Client::open(url)?;
    let mut connection = client.get_async_connection().await?;
    let values: Vec<String> = connection.hvals(redis_channel).await?;
    for value in values {
        let msgs = (config.fn_output)(&value).map_err(Error::FnOutput);
        let msgs = match msgs {
            Ok(msgs) => msgs,
            Err(err) => {
                warn!("{}", err);
                continue;
            }
        };
        let msgs = match msgs {
            Some(msgs) => msgs,
            None => continue,
        };
        for msg in msgs {
            output.send(msg).await.map_err(Error::CmpOutput)?
        }
    }
    info!("Finish reading redis hash");
    Ok(())
}
