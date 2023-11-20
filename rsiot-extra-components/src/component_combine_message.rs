//! Компонент для преобразования нескольких сообщений в новое

use std::collections::HashMap;
use tokio::{
    sync::mpsc::{self, error::SendError},
    time::{sleep, Duration},
};

use rsiot_messages_core::IMessage;
use tracing::{error, info};

type FilterFn<TMessage> = fn(TMessage) -> Option<TMessage>;
type CombineFn<TMessage> = fn(Vec<TMessage>) -> Option<TMessage>;

/// Компонент для преобразования нескольких сообщений в новое.
/// На выход передаются все исходные сообщения, плюс новые
///
/// - `input` - исходный поток сообщений
/// - `output` - исходный поток сообщений, плюс новые сообщения
/// - `filter_fn` - функция для фильтрации необходимых исходных сообщений.
/// Сообщения сохраняются в хеше. Сигнатура `fn(TMessage) -> Option<TMessage>`
/// - `transform_fn` - функция для преобразования сохраненных сообщений в новое.
/// Сигнатура `fn(Vec<TMessage>) -> Option<TMessage>`
pub async fn component_combine_message<TMessage>(
    mut input: mpsc::Receiver<TMessage>,
    output: mpsc::Sender<TMessage>,
    filter_fn: FilterFn<TMessage>,
    combine_fn: CombineFn<TMessage>,
) where
    TMessage: IMessage,
{
    info!("Component component_combine_message started");
    loop {
        let result = loop_(&mut input, &output, filter_fn, combine_fn).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...")
    }
}

async fn loop_<TMessage>(
    input: &mut mpsc::Receiver<TMessage>,
    output: &mpsc::Sender<TMessage>,
    filter_fn: FilterFn<TMessage>,
    combine_fn: CombineFn<TMessage>,
) -> Result<(), SendError<TMessage>>
where
    TMessage: IMessage,
{
    let mut msg_hash = HashMap::<String, TMessage>::new();
    while let Some(msg) = input.recv().await {
        output.send(msg.clone()).await?;
        let new_msg =
            filter_and_transform(&mut msg_hash, msg, filter_fn, combine_fn);
        if let Some(new_msg) = new_msg {
            output.send(new_msg).await?;
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
    let msg_vec: Vec<TMessage> = msg_hash.values().cloned().collect();
    transform_fn(msg_vec)
}
