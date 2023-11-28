use reqwest::Error as ReqwestError;
use tokio::{sync::mpsc::error::SendError, task::JoinError};

#[derive(Debug)]
pub enum Error<TMessage> {
    /// Ошибка конфигурации пользователя
    ConfigurationError(String),
    ReqwestError(ReqwestError),
    SendChannel(SendError<TMessage>),
    TokioJoin(JoinError),
}

impl<TMessage> From<ReqwestError> for Error<TMessage> {
    fn from(value: ReqwestError) -> Self {
        Self::ReqwestError(value)
    }
}

impl<TMessage> From<SendError<TMessage>> for Error<TMessage> {
    fn from(value: SendError<TMessage>) -> Self {
        Self::SendChannel(value)
    }
}

impl<TMessage> From<JoinError> for Error<TMessage> {
    fn from(value: JoinError) -> Self {
        Self::TokioJoin(value)
    }
}
