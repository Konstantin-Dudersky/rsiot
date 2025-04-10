use std::fmt::Debug;

use async_trait::async_trait;
use tokio::sync::{broadcast, mpsc};

use crate::message::{Message, MsgDataBound};

use super::RequestResponseBound;

/// Трейт для реализации на структурах обмена данными с подчиненными устройствами
#[async_trait]
pub trait DeviceTrait<TMsg, TRequest, TResponse>
where
    Self: Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
    TRequest: 'static + RequestResponseBound,
    TResponse: 'static + RequestResponseBound,
{
    /// Запустить опрос
    async fn spawn(
        self: Box<Self>,
        ch_rx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_tx_device_to_addindex: mpsc::Sender<TRequest>,
        ch_rx_fieldbus_to_split: mpsc::Receiver<TResponse>,
        ch_tx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()>;
}
