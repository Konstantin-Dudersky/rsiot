//! Замена сообщения

use tokio::sync::mpsc::{Receiver, Sender};

/// Замена сообщения
pub struct ChangeMpscMsg<TInput, TOutput> {
    /// Входящий поток сообщений
    pub input: Receiver<TInput>,
    /// Исходящий поток сообщений
    pub output: Sender<TOutput>,
    /// Функция изменения сообщения
    pub fn_change: fn(TInput) -> TOutput,
}

impl<TInput, TOutput> ChangeMpscMsg<TInput, TOutput> {
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(input_msg) = self.input.recv().await {
            let output_msg = (self.fn_change)(input_msg);
            self.output
                .send(output_msg)
                .await
                .map_err(|e| Error::TokioSyncMpsc(e.to_string()))?;
        }
        Ok(())
    }
}

#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    TokioSyncMpsc(String),
}
