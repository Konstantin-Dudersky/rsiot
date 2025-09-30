use futures::TryFutureExt;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
};

use crate::{
    components_config::mqtt_client::{
        ConfigPublish, ConfigSubscribe, MqttMsgGen, MqttMsgRecv, MqttMsgSend,
    },
    executor::{MsgBusInput, MsgBusLinker, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{task_input::Input, task_output::Output};

pub(crate) struct MqttGeneralTasks<'a, TMsg, TError>
where
    TMsg: MsgDataBound,
{
    /// Шина сообщений
    pub msg_bus: MsgBusLinker<TMsg>,

    /// Ёмкость очередей сообщений между задачами
    pub buffer_size: usize,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<Result<(), TError>>,

    /// Настройка публикации данных в брокере
    pub publish: ConfigPublish<TMsg>,

    /// Настройка подписки на данные из брокера
    pub subscribe: ConfigSubscribe<TMsg>,

    /// Генератор сообщений MQTT
    pub mqtt_msg_gen: MqttMsgGen,

    pub error_fn_publish: fn(anyhow::Error) -> TError,
    pub error_fn_subscribe: fn(anyhow::Error) -> TError,

    pub error_task_end_input: fn() -> TError,

    pub error_task_end_output: fn() -> TError,

    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TMsg, TError> MqttGeneralTasks<'_, TMsg, TError>
where
    TMsg: 'static + MsgDataBound,
    TError: 'static + Send,
{
    pub fn spawn(self) -> (mpsc::Receiver<MqttMsgSend>, mpsc::Sender<MqttMsgRecv>) {
        let (ch_tx_send, ch_rx_send) = mpsc::channel::<MqttMsgSend>(self.buffer_size);
        let (ch_tx_recv, ch_rx_recv) = mpsc::channel::<MqttMsgRecv>(self.buffer_size);

        // Получение сообщений из шины
        let task = Input {
            input: self.msg_bus.input(),
            output: ch_tx_send.clone(),
            config_publish: self.publish,
            mqtt_msg_gen: self.mqtt_msg_gen.clone(),
            error_fn_publish: self.error_fn_publish,
            error_task_end: self.error_task_end_input,
            error_tokio_mpsc_send: self.error_tokio_mpsc_send,
        };
        join_set_spawn(self.task_set, "mqtt_general_tasks | input", task.spawn());

        let task = Output {
            input: ch_rx_recv,
            output_send: ch_tx_send,
            output_msg_bus: self.msg_bus.output(),
            config_subscribe: self.subscribe,
            mqtt_msg_gen: self.mqtt_msg_gen.clone(),
            error_fn_subscribe: self.error_fn_subscribe,
            error_tokio_mpsc_send: self.error_tokio_mpsc_send,
            error_task_end: self.error_task_end_output,
        };
        join_set_spawn(self.task_set, "mqtt_general_tasks | output", task.spawn());

        (ch_rx_send, ch_tx_recv)
    }
}
