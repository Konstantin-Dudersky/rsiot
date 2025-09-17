use tokio::sync::mpsc;
use tracing::trace;

use super::{CanFilter, CanFrame, CanSettings, Error, can_socket::CanSocket};

pub struct RecvFromCan {
    pub output: mpsc::Sender<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
    pub filters: Vec<CanFilter>,
}

impl RecvFromCan {
    pub async fn spawn(self) -> Result<(), Error> {
        let mut socket = CanSocket::open(&self.ifname, self.can_settings)?;
        socket.set_filters(&self.filters)?;

        while let Some(frame) = socket.next().await {
            let frame = frame?;
            trace!("Frame: {:?}", frame);

            self.output
                .send(frame)
                .await
                .map_err(|_| Error::TokioSyncMpscSend)?;
        }

        Err(Error::TaskEndRecvFromCan)
    }
}
