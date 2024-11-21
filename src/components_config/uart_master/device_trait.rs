use async_trait::async_trait;
use tokio::sync::{broadcast, mpsc};

use crate::message::{Message, MsgDataBound};

use super::UartMessageRaw;

/// Трейт для реализации на структурах обмена данными с подчиненными устройствами
#[cfg_attr(not(feature = "single-thread"), async_trait)]
#[cfg_attr(feature = "single-thread", async_trait(?Send))]
pub trait DeviceTrait<TMsg>
where
    Self: std::fmt::Debug + Send + Sync,
    TMsg: MsgDataBound + 'static,
{
    /// Запустить опрос
    async fn spawn(
        self: Box<Self>,
        ch_tx_device_to_uart: mpsc::Sender<UartMessageRaw>,
        ch_rx_uart_to_device: broadcast::Receiver<UartMessageRaw>,
        ch_tx_msgbus_to_device: broadcast::Receiver<Message<TMsg>>,
        ch_rx_device_to_msgbus: mpsc::Sender<Message<TMsg>>,
    ) -> super::Result<()>;
}
