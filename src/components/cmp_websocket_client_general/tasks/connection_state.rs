use tokio::sync::mpsc;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound, ServiceBound},
};

/// Задача контроля соединения
///
/// Должна вызываться отдельно от остальных задач
pub struct ConnectionState<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Канал получения состояния соединения
    pub input: mpsc::Receiver<bool>,
    /// Шина сообщений
    pub output: CmpInOut<TMsg, TService>,
    /// Преобразование состояния в исходящее сообщение
    pub fn_connection_state: fn(bool) -> Option<Message<TMsg>>,
}

impl<TMsg, TService> ConnectionState<TMsg, TService>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    /// Запуск на выполнение
    pub async fn spawn(mut self) -> super::Result<()> {
        create_msg_and_send(false, self.fn_connection_state, &self.output).await?;
        while let Some(state) = self.input.recv().await {
            create_msg_and_send(state, self.fn_connection_state, &self.output).await?;
        }
        Err(super::Error::TaskConnectionState)
    }
}

async fn create_msg_and_send<TMsg, TService>(
    state: bool,
    fn_connection_state: fn(bool) -> Option<Message<TMsg>>,
    output: &CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
{
    let msg = (fn_connection_state)(state);
    let Some(msg) = msg else { return Ok(()) };
    output
        .send_output(msg)
        .await
        .map_err(|_| super::Error::TokioSyncMpsc)
}
