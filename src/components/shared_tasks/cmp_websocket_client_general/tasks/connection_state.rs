use tokio::sync::mpsc;

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

/// Задача контроля соединения
///
/// Должна вызываться отдельно от остальных задач
pub struct ConnectionState<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Канал получения состояния соединения
    pub input: mpsc::Receiver<bool>,
    /// Шина сообщений
    pub output: CmpInOut<TMsg>,
    /// Преобразование состояния в исходящее сообщение
    pub fn_connection_state: fn(bool) -> Option<Message<TMsg>>,
}

impl<TMsg> ConnectionState<TMsg>
where
    TMsg: MsgDataBound,
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

async fn create_msg_and_send<TMsg>(
    state: bool,
    fn_connection_state: fn(bool) -> Option<Message<TMsg>>,
    output: &CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let msg = (fn_connection_state)(state);
    let Some(msg) = msg else { return Ok(()) };
    output
        .send_output(msg)
        .await
        .map_err(|_| super::Error::TokioSyncMpscSend)
}
