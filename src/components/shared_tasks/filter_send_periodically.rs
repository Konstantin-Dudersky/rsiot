//! Задача фильтрации потока сообщений. Сообщения кешируются и отправляются периодически.

use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::{
    sync::{mpsc, Mutex},
    task::JoinSet,
};

use crate::{
    executor::{join_set_spawn, sleep},
    message::{Message, MsgDataBound},
};

/// Задача фильтрации потока сообщений. Сообщения кешируются и отправляются периодически.
pub struct FilterSendPeriodically<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Поток входящих сообщений
    pub input: mpsc::Receiver<Message<TMsg>>,
    /// Поток исходящих сообщений
    pub output: mpsc::Sender<Message<TMsg>>,
    /// Периодичность отправки сообщений
    pub period: Duration,
}

impl<TMsg> FilterSendPeriodically<TMsg>
where
    TMsg: MsgDataBound + 'static,
{
    /// Запуск на выполнение
    pub async fn spawn(self) -> Result<(), Error> {
        let cache = Arc::new(Mutex::new(HashMap::new()));

        let mut task_set = JoinSet::new();

        let task = TaskInput {
            input: self.input,
            cache: cache.clone(),
        };
        join_set_spawn(&mut task_set, "filter_send_periodically", task.spawn());

        let task = TaskOutput {
            output: self.output,
            cache: cache.clone(),
            period: self.period,
        };
        join_set_spawn(&mut task_set, "filter_send_periodically", task.spawn());

        while let Some(res) = task_set.join_next().await {
            res??;
        }

        Err(Error::TaskEnd)
    }
}

type Cache<TMsg> = Arc<Mutex<HashMap<String, Message<TMsg>>>>;

struct TaskInput<TMsg> {
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub cache: Cache<TMsg>,
}
impl<TMsg> TaskInput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(msg) = self.input.recv().await {
            let mut cache = self.cache.lock().await;
            cache.insert(msg.key.clone(), msg);
        }
        Err(Error::TaskEnd)
    }
}

struct TaskOutput<TMsg> {
    pub output: mpsc::Sender<Message<TMsg>>,
    pub cache: Cache<TMsg>,
    pub period: Duration,
}
impl<TMsg> TaskOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        loop {
            {
                let mut cache = self.cache.lock().await;
                for msg in cache.values() {
                    self.output
                        .send(msg.clone())
                        .await
                        .map_err(|_| Error::TokioMpscSend)?;
                }
                cache.clear();
            }
            sleep(self.period).await;
        }
    }
}

#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Task ended")]
    TaskEnd,

    #[error(transparent)]
    TokioJoin(#[from] tokio::task::JoinError),

    #[error("TokioMpscSend")]
    TokioMpscSend,
}
