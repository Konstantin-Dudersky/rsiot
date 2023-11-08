//! Компонент для преобразования нескольких сообщений в новое

use std::collections::HashMap;
use tokio::{
    sync::mpsc::{self, error::SendError},
    time::{sleep, Duration},
};

use rsiot_messages_core::IMessage;
use tracing::{error, info};

/// Компонент для преобразования нескольких сообщений в новое
///
/// # Arguments
/// - `filter_fn` - функция для фильтрации необходимых исходных сообщений.
/// Сообщения сохраняются в хеше.
/// - `transform_fn` - функция для преобразования сохраненных сообщений в новое.
pub async fn component_combine_message<TMessage>(
    mut channel_rcv: mpsc::Receiver<TMessage>,
    channel_send: mpsc::Sender<TMessage>,
    filter_fn: fn(TMessage) -> Option<TMessage>,
    transform_fn: fn(Vec<TMessage>) -> Option<TMessage>,
) where
    TMessage: IMessage,
{
    info!("Component component_combine_message started");
    loop {
        let result =
            loop_(&mut channel_rcv, &channel_send, filter_fn, transform_fn)
                .await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn loop_<TMessage>(
    channel_rcv: &mut mpsc::Receiver<TMessage>,
    channel_send: &mpsc::Sender<TMessage>,
    filter_fn: fn(TMessage) -> Option<TMessage>,
    transform_fn: fn(Vec<TMessage>) -> Option<TMessage>,
) -> Result<(), SendError<TMessage>>
where
    TMessage: IMessage,
{
    let mut msg_hash = HashMap::<String, TMessage>::new();
    while let Some(msg) = channel_rcv.recv().await {
        channel_send.send(msg.clone()).await?;
        let new_msg =
            filter_and_transform(&mut msg_hash, msg, filter_fn, transform_fn);
        if let Some(new_msg) = new_msg {
            channel_send.send(new_msg).await?;
        }
    }
    Ok(())
}

/// Обработка сообщения.
/// Проводим сообщение через функцию фильтрации. Сохраняем в хеше. Проводим все
/// сообщения через функцию трасформации.
fn filter_and_transform<TMessage>(
    msg_hash: &mut HashMap<String, TMessage>,
    msg: TMessage,
    filter_fn: fn(TMessage) -> Option<TMessage>,
    transform_fn: fn(Vec<TMessage>) -> Option<TMessage>,
) -> Option<TMessage>
where
    TMessage: IMessage,
{
    let msg = filter_fn(msg);
    let msg = match msg {
        Some(val) => val,
        None => return None,
    };
    msg_hash.insert(msg.key(), msg.clone());
    let msg_vec: Vec<TMessage> = msg_hash.values().map(|m| m.clone()).collect();
    transform_fn(msg_vec)
}
