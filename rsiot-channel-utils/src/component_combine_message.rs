//! Сборка нескольких сообщений в одно

use std::collections::HashMap;
use tokio::sync::mpsc;

use rsiot_messages_core::IMessage;
use tracing::info;

pub async fn component_combine_message<TMessage>(
    mut channel_rcv: mpsc::Receiver<TMessage>,
    channel_send: mpsc::Sender<TMessage>,
    keys: Vec<&str>,
    config_func: fn(Vec<TMessage>) -> TMessage,
) where
    TMessage: IMessage,
{
    let keys: Vec<String> = keys.iter().map(|key| key.to_string()).collect();

    let mut hash = HashMap::<String, Option<TMessage>>::new();
    for key in &keys {
        hash.entry(key.clone()).or_insert(None);
    }

    while let Some(msg) = channel_rcv.recv().await {
        channel_send.send(msg.clone()).await.unwrap();
        let key = msg.key();
        if keys.contains(&key) {
            hash.entry(key).and_modify(|m| *m = Some(msg));
        }
        if hash.values().all(|m| m.is_some()) {
            let messages: Vec<TMessage> =
                hash.values().map(|m| m.clone().expect("")).collect();
            let new_msg = config_func(messages);
            channel_send.send(new_msg).await.unwrap();
            for key in &keys {
                hash.entry(key.clone()).and_modify(|m| *m = None);
            }
        }
    }
}
