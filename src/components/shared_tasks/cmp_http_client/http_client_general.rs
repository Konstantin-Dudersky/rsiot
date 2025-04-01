use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    components_config::http_client::{MsgRequest, MsgResponse, RequestInput, RequestPeriodic},
    executor::{join_set_spawn, CmpInOut},
    message::MsgDataBound,
};

use super::{tasks, Error};

/// Запуск общих задач работы HTTP-клиента
pub struct HttpClientGeneral<'a, TMsg>
where
    TMsg: MsgDataBound,
{
    /// Шина сообщений
    pub msg_bus: CmpInOut<TMsg>,

    /// Ёмкость очередей сообщений между задачами
    pub buffer_size: usize,

    /// Ссылка на коллекцию задач tokio
    pub task_set: &'a mut JoinSet<super::Result<()>>,

    /// Запросы, которые формируются на основе входящих сообщений
    pub requests_input: Vec<Box<dyn RequestInput<TMsg>>>,

    /// Периодические запросы
    pub requests_periodic: Vec<Box<dyn RequestPeriodic<TMsg>>>,
}

impl<TMsg> HttpClientGeneral<'_, TMsg>
where
    TMsg: 'static + MsgDataBound,
{
    /// Запуск
    pub fn spawn(self) -> (mpsc::Receiver<MsgRequest>, mpsc::Sender<MsgResponse>) {
        let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) = mpsc::channel(self.buffer_size);
        let (ch_tx_requests, ch_rx_requests) = mpsc::channel(self.buffer_size);
        let (ch_tx_reponse, ch_rx_response) = mpsc::channel(self.buffer_size);
        let (ch_tx_output_to_msgbus, ch_rx_output_to_msgbus) = mpsc::channel(self.buffer_size);

        // Получение сообщений из шины
        let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
            msg_bus: self.msg_bus.clone(),
            output: ch_tx_msgbus_to_input,
        };
        join_set_spawn(self.task_set, task.spawn().map_err(Error::TaskMsgBusToMpsc));

        // Создание HTTP-запросов на основе входящих сообщений
        let task = tasks::InputRequest {
            input: ch_rx_msgbus_to_input,
            output: ch_tx_requests.clone(),
            request_input_config: self.requests_input.clone(),
        };
        join_set_spawn(self.task_set, task.spawn());

        // Создание периодических HTTP-запросов
        for pr in self.requests_periodic.iter() {
            let task = tasks::PeriodicRequest {
                output: ch_tx_requests.clone(),
                request_periodic: pr.clone(),
            };
            join_set_spawn(self.task_set, task.spawn());
        }

        // Обработка ответов от сервера
        let task = tasks::ProcessResponse {
            input: ch_rx_response,
            output: ch_tx_output_to_msgbus,
            requests_input: self.requests_input,
            requests_periodic: self.requests_periodic,
        };
        join_set_spawn(self.task_set, task.spawn());

        // Отправка исходящих сообщений
        let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
            input: ch_rx_output_to_msgbus,
            msg_bus: self.msg_bus,
        };
        join_set_spawn(self.task_set, task.spawn().map_err(Error::TaskMpscToMsgBus));

        (ch_rx_requests, ch_tx_reponse)
    }
}
