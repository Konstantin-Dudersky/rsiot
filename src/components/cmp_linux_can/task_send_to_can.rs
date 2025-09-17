use tokio::sync::mpsc;

use crate::components_config::can_general::{CanFrame, CanSettings};

use super::{Error, can_socket::CanSocket};

pub struct SendToCan {
    pub input: mpsc::Receiver<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
}

impl SendToCan {
    pub async fn spawn(mut self) -> Result<(), Error> {
        let socket = CanSocket::open(&self.ifname, self.can_settings)?;
        while let Some(frame) = self.input.recv().await {
            socket.write_frame(frame).await?;
        }
        Err(Error::TaskEndSendToCan)
    }
}
