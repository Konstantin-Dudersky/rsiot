use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    components_config::{
        websocket_client::{FnClientToServer, FnServerToClient},
        websocket_general::WebsocketMessage,
    },
    executor::{CmpInOut, MsgBusInput, MsgBusOutput, join_set_spawn},
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

    /// Подключение к шине сообщений
    pub msgbus_linker: CmpInOut<TMsg>,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<super::Result<()>>,

    /// Преобразование входящих сообщений в текст для отправки на сервер
    pub fn_client_to_server: FnClientToServer<TMsg, TClientToServer>,

    /// Преобразование полученного от сервера текста в исходящие сообщения
    pub fn_server_to_client: FnServerToClient<TMsg, TServerToClient>,

    pub fn_connection_state: fn(bool) -> Option<TMsg>,
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
    pub fn spawn(
        self,
    ) -> (
        broadcast::Receiver<Vec<u8>>,
        mpsc::Sender<Vec<u8>>,
        mpsc::Sender<bool>,
    ) {
        let buffer_size = self.msgbus_linker.max_capacity();

        let (ch_tx_input_to_send, ch_rx_input_to_send) = broadcast::channel(buffer_size);
        let (ch_tx_receive_to_output, ch_rx_receive_to_output) = mpsc::channel(buffer_size);
        let (ch_tx_connection_state, ch_rx_connection_state) = mpsc::channel(buffer_size);

        // Преобразование входящих сообщений в текст для отправки
        let task = tasks::ClientToServer {
            input: self.msgbus_linker.input(),
            output: ch_tx_input_to_send,
            fn_input: self.fn_client_to_server,
            serde_alg: self.serde_alg,
        };
        join_set_spawn(
            self.task_set,
            "websocket_client | client_to_server",
            task.spawn(),
        );

        // Преобразование полученного текста в сообщение
        let task = tasks::ServerToClient {
            input: ch_rx_receive_to_output,
            output: self.msgbus_linker.output(),
            fn_output: self.fn_server_to_client,
            serde_alg: self.serde_alg,
        };
        join_set_spawn(
            self.task_set,
            "websocket_client | server_to_client",
            task.spawn(),
        );

        let task = tasks::ConnectionState {
            input: ch_rx_connection_state,
            output: self.msgbus_linker.output(),
            fn_connection_state: self.fn_connection_state,
        };
        join_set_spawn(
            self.task_set,
            "websocket_client | connection_state",
            task.spawn(),
        );

        self.msgbus_linker.close();

        (
            ch_rx_input_to_send,
            ch_tx_receive_to_output,
            ch_tx_connection_state,
        )
    }
}
