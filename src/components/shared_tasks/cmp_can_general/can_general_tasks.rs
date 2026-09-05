use std::{sync::Arc, time::Duration};

use futures::TryFutureExt;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};

use crate::{
    components_config::can_general::CanFrame,
    executor::{MsgBusInput, MsgBusLinker, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{task_input::Input, task_output::Output};

pub(crate) struct CanGeneralTasks<'a, TMsg, TError, TFnInput>
where
    TMsg: MsgDataBound,
    TFnInput: Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>>,
{
    /// Подключение к шине сообщений
    pub msgbus_linker: MsgBusLinker<TMsg>,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<Result<(), TError>>,

    pub fn_input: TFnInput,

    pub fn_output: fn(CanFrame) -> anyhow::Result<Option<Vec<TMsg>>>,

    pub error_task_end_input: fn() -> TError,

    pub error_task_end_output: fn() -> TError,

    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TError, TFnInput> CanGeneralTasks<'_, TMsg, TError, TFnInput>
where
    TMsg: 'static + MsgDataBound,
    TError: 'static + Send,
    TFnInput: 'static + Fn(&TMsg) -> anyhow::Result<Option<Vec<CanFrame>>> + Send,
{
    pub fn spawn(self) -> (mpsc::Receiver<CanFrame>, mpsc::Sender<CanFrame>) {
        let buffer_size = self.msgbus_linker.max_capacity();

        let (ch_tx_send_to_can, ch_rx_send_to_can) = mpsc::channel::<CanFrame>(buffer_size);
        let (ch_tx_recv_from_can, ch_rx_recv_from_can) = mpsc::channel::<CanFrame>(buffer_size);

        // Получение сообщений из шины
        let task = Input {
            input: self.msgbus_linker.input(),
            output: ch_tx_send_to_can.clone(),
            fn_input: self.fn_input,
            error_task_end: self.error_task_end_input,
            error_tokio_mpsc_send: self.error_tokio_mpsc_send,
        };
        join_set_spawn(self.task_set, "can_general_tasks | input", task.spawn());

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
