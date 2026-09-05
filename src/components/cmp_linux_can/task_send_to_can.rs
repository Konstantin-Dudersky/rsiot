use tokio::sync::broadcast;
use tracing::warn;

use crate::components_config::can_general::{CanFrame, CanSettings};

use super::{
    Error,
    can_socket::{CanSocketAsync, CanSocketSync},
};

pub struct SendToCanAsync {
    pub input: broadcast::Receiver<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
}
impl SendToCanAsync {
    pub async fn spawn(mut self) -> Result<(), Error> {
        let socket = CanSocketAsync::open(&self.ifname, self.can_settings.clone())?;
        // let delay_after_send = DelayAfterSend::new(&self.can_settings);
        while let Ok(frame) = self.input.recv().await {
            // let delay = delay_after_send.calc(&frame);
            let res = socket.write_frame(frame).await;
            // tokio_time_sleep(delay).await;
            if let Err(err) = res {
                warn!("Error sending frame: {}", err);
                break;
            }
        }
        Err(Error::TaskEndSendToCan)
    }
}

pub struct SendToCanSync {
    pub input: broadcast::Receiver<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
}
impl SendToCanSync {
    pub fn spawn(mut self) -> Result<(), Error> {
        let ifname = self.ifname;
        let can_settings = self.can_settings;
        let mut socket = CanSocketSync::open(&ifname, can_settings)?;
        while let Ok(frame) = self.input.blocking_recv() {
            let res = socket.transmit(frame);
            if let Err(err) = res {
                warn!("Error sending frame: {}", err);
                continue;
            }
        }
        Err(Error::TaskEndSendToCan)
    }
}
