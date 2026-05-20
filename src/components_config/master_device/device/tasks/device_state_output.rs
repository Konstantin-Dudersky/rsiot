use tokio::sync::mpsc;

use crate::{
    executor::{CheckCapacity, sleep},
    message::{Message, MsgDataBound},
};

use super::{ConfigDeviceStateOutput, DeviceStateType, Error};

pub struct DeviceStateOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub device_state: DeviceStateType,
    pub ch_tx_output_to_filter: mpsc::Sender<Message<TMsg>>,
    pub config: ConfigDeviceStateOutput<TMsg>,
}

impl<TMsg> DeviceStateOutput<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            let msg = {
                let state = self.device_state.lock().await;
                (self.config.fn_device_state)(*state)
            };

            let msg = msg.to_message();
            self.ch_tx_output_to_filter
                .check_capacity(
                    0.2,
                    "master_device | DeviceStateOutput | ch_tx_output_to_filter",
                )
                .send(msg)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;

            sleep(self.config.period).await
        }
    }
}
