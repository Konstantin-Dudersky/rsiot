use futures_util::StreamExt as _;
use redis::{AsyncCommands, Client};
use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{sleep, Duration},
};
use tracing::{error, info, trace};
use url::Url;

use rsiot_component_core::{IComponent, Input, Output};
use rsiot_extra_components::cmp_mpsc_to_mpsc;
use rsiot_messages_core::{IMessage, IMessageChannel};

use crate::{
    cmp_redis_subscriber::{self, Config},
    error::Error,
};

type TaskResult = Result<(), Error>;

pub async fn fn_process<TMessage, TMessageChannel>(
    _input: Input<TMessage>,
    output: Output<TMessage>,
    config: Config<TMessageChannel>,
) where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    info!("Initialization. Config: {:?}", config);

    // Создаем канал для пересылки сообщений на выход
    let (stream_to_output_tx, stream_to_output_rx) = mpsc::channel::<TMessage>(100);
    let _task_to_output =
        cmp_mpsc_to_mpsc::create().set_and_spawn(Some(stream_to_output_rx), output);

    loop {
        info!("Starting");
        let result = task_main(stream_to_output_tx.clone(), config.clone()).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn task_main<TMessage, TMessageChannel>(
    output: mpsc::Sender<TMessage>,
    config: cmp_redis_subscriber::Config<TMessageChannel>,
) -> TaskResult
where
    TMessage: IMessage + 'static,
    TMessageChannel: IMessageChannel + 'static,
{
    let mut set = JoinSet::new();
    set.spawn(task_subscription(
        output.clone(),
        config.url.clone(),
        config.redis_channel.clone(),
    ));
    set.spawn(task_read_hash(
        output.clone(),
        config.url.clone(),
        config.redis_channel.clone(),
    ));
    while let Some(res) = set.join_next().await {
        res??;
    }
    Ok(())
}

/// Подписка на канал Pub/Sub
async fn task_subscription<TMessage, TMessageChannel>(
    output: mpsc::Sender<TMessage>,
    url: Url,
    redis_channel: TMessageChannel,
) -> TaskResult
where
    TMessage: IMessage,
    TMessageChannel: IMessageChannel,
{
    info!("Start redis subscription");
    let client = Client::open(url.to_string())?;
    let connection = client.get_async_connection().await?;
    let mut pubsub = connection.into_pubsub();
    pubsub.subscribe(redis_channel.to_string()).await?;
    let mut stream = pubsub.on_message();
    loop {
        let redis_msg = stream.next().await;
        let redis_msg = match redis_msg {
            Some(value) => value,
            None => return Err(Error::GetMessage),
        };
        trace!("New message from Redis: {:?}", redis_msg);
        let payload: String = redis_msg.get_payload()?;
        let msg: Result<TMessage, _> = TMessage::from_json(&payload);
        match msg {
            Ok(msg) => output.send(msg).await?,
            Err(err) => {
                let err = format!("Wrong message: {:?}; error: {:?}", payload, err);
                error!(err);
            }
        }
    }
}

/// Чтение данных из хеша
async fn task_read_hash<TMessage, TMessageChannel>(
    output: mpsc::Sender<TMessage>,
    url: Url,
    redis_channel: TMessageChannel,
) -> TaskResult
where
    TMessage: IMessage,
    TMessageChannel: IMessageChannel,
{
    info!("Start reading redis hash");
    let client = Client::open(url.to_string())?;
    let mut connection = client.get_async_connection().await?;
    let values: Vec<String> = connection.hvals(redis_channel.to_string()).await?;
    for value in values {
        let msg = TMessage::from_json(&value);
        let msg = match msg {
            Ok(val) => val,
            Err(err) => {
                let err = format!("Wrong message: {:?}; error: {:?}", value, err);
                error!(err);
                continue;
            }
        };
        output.send(msg).await?;
    }
    info!("Finish reading redis hash");
    Ok(())
}
