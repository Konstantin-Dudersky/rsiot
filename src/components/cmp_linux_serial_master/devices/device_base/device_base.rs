#![allow(clippy::module_inception)]

use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{components_config::uart_general::RequestResponseBound, message::Message};
use crate::{executor::join_set_spawn, message::MsgDataBound};

use super::super::super::UartMessageRaw;
use super::{config::*, tasks};

pub struct DeviceBase<TMsg, TRequest, TResponse> {
    pub address: u8,
    pub periodic_requests: Vec<ConfigPeriodicRequest<TRequest>>,
    pub input_request: Vec<ConfigInputRequest>,
    pub fn_output: fn(TResponse) -> Vec<Message<TMsg>>,
}

impl<TMsg, TRequest, TResponse> DeviceBase<TMsg, TRequest, TResponse>
where
    TRequest: 'static + RequestResponseBound,
    TResponse: 'static + RequestResponseBound,
    TMsg: MsgDataBound + 'static,
{
    pub async fn spawn(
        self,
        ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw>,
        ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw>,
        ch_tx_device_to_filter: mpsc::Sender<Message<TMsg>>,
    ) {
        let mut task_set: JoinSet<()> = JoinSet::new();

        for periodic_request in self.periodic_requests {
            let task = tasks::PeriodicRequest {
                address: self.address,
                period: periodic_request.period,
                request: periodic_request.request,
                ch_tx_device_to_uart: ch_tx_device_to_uart.clone(),
            };
            join_set_spawn(&mut task_set, task.spawn());
        }

        let task = tasks::Output {
            address: self.address,
            ch_tx_uart_to_device: ch_rx_uart_to_device,
            ch_cmp_output: ch_tx_device_to_filter,
            fn_output: self.fn_output,
        };
        join_set_spawn(&mut task_set, task.spawn());

        while let Some(res) = task_set.join_next().await {
            res.unwrap();
        }
    }
}
