//! Компонент для фильтрации сообщений
//!
//! Вход - поток сообщений `tokio::sync::mpsc::Receiver`
//! Выход - поток отфильтрованных сообщений `tokio::sync::mpsc::Sender`

pub type FilterFn<TMessage> = fn(TMessage) -> Option<TMessage>;

use tokio::{
    sync::mpsc::{error::SendError, Receiver, Sender},
    time::{sleep, Duration},
};
use tracing::{error, info};

/// Компонент для фильтрации сообщений
/// - `input` - поток сообщений `tokio::sync::mpsc::Receiver`
/// - `output` - поток отфильтрованных сообщений `tokio::sync::mpsc::Sender`
/// - `filter_fn` - функция типа `fn(TMessage) -> Option<TMessage>`
pub async fn component_filter_message<TMessage>(
    mut input: Receiver<TMessage>,
    output: Sender<TMessage>,
    filter_fn: FilterFn<TMessage>,
) -> () {
    info!("Component component_filter_message started");
    loop {
        let result = loop_(&mut input, &output, filter_fn).await;
        match result {
            Ok(_) => (),
            Err(err) => error!("{:?}", err),
        }
        sleep(Duration::from_secs(2)).await;
        info!("Restarting...");
    }
}

async fn loop_<TMessage>(
    input: &mut Receiver<TMessage>,
    output: &Sender<TMessage>,
    filter_fn: FilterFn<TMessage>,
) -> Result<(), SendError<TMessage>> {
    while let Some(msg) = input.recv().await {
        let msg = filter_fn(msg);
        match msg {
            Some(msg) => output.send(msg).await?,
            None => (),
        }
    }
    Ok(())
}
