use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    components_config::http_client::{MsgRequest, MsgResponse, RequestInput, RequestPeriodic},
    executor::{CmpInOut, MsgBusInput, MsgBusOutput, join_set_spawn},
    message::MsgDataBound,
};

use super::{Error, tasks};

/// Запуск общих задач работы HTTP-клиента
pub struct HttpClientGeneral<'a, TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: MsgBusOutput<TMsg>,

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
        let (ch_tx_requests, ch_rx_requests) = mpsc::channel(self.buffer_size);
        let (ch_tx_reponse, ch_rx_response) = mpsc::channel(self.buffer_size);

        // Создание HTTP-запросов на основе входящих сообщений
        let task = tasks::Input {
            input: self.input,
            output: ch_tx_requests.clone(),
            request_input_config: self.requests_input.clone(),
        };
        join_set_spawn(self.task_set, "cmp_http_client | input", task.spawn());

        // Создание периодических HTTP-запросов
        for pr in self.requests_periodic.iter() {
            let task = tasks::Periodic {
                output: ch_tx_requests.clone(),
                request_periodic: pr.clone(),
            };
            join_set_spawn(self.task_set, "cmp_http_client | periodic", task.spawn());
        }

        // Обработка ответов от сервера
        let task = tasks::Response {
            input: ch_rx_response,
            output: self.output,
            requests_input: self.requests_input,
            requests_periodic: self.requests_periodic,
        };
        join_set_spawn(self.task_set, "cmp_http_client | response", task.spawn());

        (ch_rx_requests, ch_tx_reponse)
    }
}
