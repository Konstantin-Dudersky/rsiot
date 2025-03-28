use futures::TryFutureExt;
use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    components::shared_tasks,
    components_config::{
        websocket_client::{FnClientToServer, FnServerToClient},
        websocket_general::WebsocketMessage,
    },
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
    serde_utils::SerdeAlg,
};

use super::tasks;

/// Запуск задач, общих для всех websocket клиентов
pub struct WebsocketClientGeneralTasks<'a, TMsg, TServerToClient, TClientToServer>
where
    TMsg: MsgDataBound,
{
    /// Алгоритм сериализации / десериализации
    pub serde_alg: SerdeAlg,

    /// Шина сообщений
    pub msg_bus: CmpInOut<TMsg>,

    /// Ёмкость очередей сообщений между задачами
    pub buffer_size: usize,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<super::Result<()>>,

    /// Преобразование входящих сообщений в текст для отправки на сервер
    pub fn_client_to_server: FnClientToServer<TMsg, TClientToServer>,

    /// Преобразование полученного от сервера текста в исходящие сообщения
    pub fn_server_to_client: FnServerToClient<TMsg, TServerToClient>,

    /// Канал для передачи состояния соединения
    pub ch_tx_connection_state: mpsc::Sender<bool>,
}

impl<TMsg, TServerToClient, TClientToServer>
    WebsocketClientGeneralTasks<'_, TMsg, TServerToClient, TClientToServer>
where
    TMsg: 'static + MsgDataBound,
    TServerToClient: 'static + WebsocketMessage,
    TClientToServer: 'static + WebsocketMessage,
{
    /// Запуск задач.
    ///
    /// Возвращает кортеж с каналами передачи запросов / ответов
    pub fn spawn(self) -> (mpsc::Receiver<Vec<u8>>, mpsc::Sender<Vec<u8>>) {
        let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = mpsc::channel(self.buffer_size);
        let (ch_tx_input_to_send, ch_rx_input_to_send) = mpsc::channel(self.buffer_size);
        let (ch_tx_receive_to_output, ch_rx_receive_to_output) = mpsc::channel(self.buffer_size);
        let (ch_tx_output_to_msgbus, ch_rx_output_to_msgbus) = mpsc::channel(self.buffer_size);

        // Получение сообщений из шины
        let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
            msg_bus: self.msg_bus.clone(),
            output: ch_tx_msgbus_to_input,
        };
        join_set_spawn(
            self.task_set,
            task.spawn().map_err(super::Error::TaskMsgbusToMpsc),
        );

        // Преобразование входящих сообщений в текст для отправки
        let task = tasks::ClientToServer {
            input: ch_rx_msgbus_to_input,
            output: ch_tx_input_to_send,
            fn_input: self.fn_client_to_server,
            serde_alg: self.serde_alg,
        };
        join_set_spawn(self.task_set, task.spawn());

        // Преобразование полученного текста в сообщение
        let task = tasks::ServerToClient {
            input: ch_rx_receive_to_output,
            output: ch_tx_output_to_msgbus,
            output_connection_state: self.ch_tx_connection_state,
            fn_output: self.fn_server_to_client,
            serde_alg: self.serde_alg,
        };
        join_set_spawn(self.task_set, task.spawn());

        // Пересылка сообщений на шину
        let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
            input: ch_rx_output_to_msgbus,
            msg_bus: self.msg_bus,
        };
        join_set_spawn(
            self.task_set,
            task.spawn().map_err(super::Error::TaskMpscToMsgBus),
        );

        (ch_rx_input_to_send, ch_tx_receive_to_output)
    }
}
