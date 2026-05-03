use tokio::sync::mpsc;
use tracing::{info, trace};

use super::{
    CanFilter, CanFrame, CanSettings, Error,
    can_socket::{CanSocketAsync, CanSocketSync},
};

pub struct RecvFromCanAsync {
    pub output: mpsc::Sender<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
    pub filters: Vec<CanFilter>,
}
impl RecvFromCanAsync {
    pub async fn spawn(self) -> Result<(), Error> {
        let mut socket = CanSocketAsync::open(&self.ifname, self.can_settings)?;
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

pub struct RecvFromCanSync {
    pub output: mpsc::Sender<CanFrame>,
    pub ifname: String,
    pub can_settings: CanSettings,
    pub filters: Vec<CanFilter>,
}
impl RecvFromCanSync {
    pub fn spawn(self) -> Result<(), Error> {
        let mut socket = CanSocketSync::open(&self.ifname, self.can_settings)?;
        socket.set_filters(&self.filters)?;

        loop {
            let frame = socket.receive();
            info!("Frame: {:?}", frame);

            // self.output
            //     .try_send(frame)
            //     .map_err(|_| Error::TokioSyncMpscSend)?;
        }

        Err(Error::TaskEndRecvFromCan)
    }
}
