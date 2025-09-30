use std::{sync::Arc, time::Duration};

use futures::TryFutureExt;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};

use crate::{
    components_config::can_general::{BufferBound, CanFrame},
    executor::{MsgBusInput, MsgBusLinker, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{task_input::Input, task_output::Output, task_periodic::Periodic};

pub(crate) struct CanGeneralTasks<'a, TMsg, TBuffer, TError>
where
    TMsg: MsgDataBound,
    TBuffer: BufferBound,
{
    /// Подключение к шине сообщений
    pub msgbus_linker: MsgBusLinker<TMsg>,

    /// Значение в буфере по умолчанию.
    ///
    /// Буфер используется для отправки периодических сообщений.
    ///
    /// Если буфер не используется, можно задать значение `()`.
    pub buffer_default: TBuffer,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<Result<(), TError>>,

    pub fn_input: fn(&TMsg, &mut TBuffer) -> anyhow::Result<Option<Vec<CanFrame>>>,

    pub period: Duration,

    pub fn_periodic: fn(&TBuffer) -> anyhow::Result<Option<Vec<CanFrame>>>,

    pub fn_output: fn(CanFrame) -> Option<Vec<TMsg>>,

    pub error_task_end_input: fn() -> TError,

    pub error_task_end_output: fn() -> TError,

    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TBuffer, TError> CanGeneralTasks<'_, TMsg, TBuffer, TError>
where
    TMsg: 'static + MsgDataBound,
    TBuffer: 'static + BufferBound,
    TError: 'static + Send,
{
    pub fn spawn(self) -> (mpsc::Receiver<CanFrame>, mpsc::Sender<CanFrame>) {
        let buffer = Arc::new(Mutex::new(self.buffer_default));

        let buffer_size = self.msgbus_linker.max_capacity();

        let (ch_tx_send_to_can, ch_rx_send_to_can) = mpsc::channel::<CanFrame>(buffer_size);
        let (ch_tx_recv_from_can, ch_rx_recv_from_can) = mpsc::channel::<CanFrame>(buffer_size);

        // Получение сообщений из шины
        let task = Input {
            input: self.msgbus_linker.input(),
            output: ch_tx_send_to_can.clone(),
            buffer: buffer.clone(),
            fn_input: self.fn_input,
            error_task_end: self.error_task_end_input,
            error_tokio_mpsc_send: self.error_tokio_mpsc_send,
        };
        join_set_spawn(self.task_set, "can_general_tasks | input", task.spawn());

        let task = Periodic {
            output: ch_tx_send_to_can,
            buffer,
            period: self.period,
            fn_periodic: self.fn_periodic,
            error_tokio_mpsc_send: self.error_tokio_mpsc_send,
        };
        join_set_spawn(self.task_set, "can_general_tasks | periodic", task.spawn());

        let task = Output {
            input: ch_rx_recv_from_can,
            output: self.msgbus_linker.output(),
            fn_output: self.fn_output,
            error_task_end: self.error_task_end_output,
            error_tokio_mpsc_send: self.error_tokio_mpsc_send,
        };
        join_set_spawn(self.task_set, "can_general_tasks | output", task.spawn());

        self.msgbus_linker.close();

        (ch_rx_send_to_can, ch_tx_recv_from_can)
    }
}
