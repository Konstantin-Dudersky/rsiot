use socketcan::{CanDataFrame, EmbeddedFrame, socket, tokio::CanSocket};
use tokio::sync::mpsc;

use crate::components_config::can_general::{Frame, Id, IdExtended, IdStandard};

use super::Error;

pub struct SendToCan {
    pub input: mpsc::Receiver<Frame>,
    pub dev_can: String,
}

impl SendToCan {
    pub async fn spawn(mut self) -> Result<(), Error> {
        let socket = CanSocket::open(&self.dev_can).map_err(Error::SocketOpen)?;

        while let Some(frame) = self.input.recv().await {
            let frame: socketcan::CanFrame = frame.try_into()?;

            socket.write_frame(frame).await.unwrap();
        }

        Err(Error::TaskEndSendToCan)
    }
}

impl TryFrom<Frame> for socketcan::CanAnyFrame {
    type Error = Error;

    fn try_from(value: Frame) -> Result<Self, Error> {
        match &value {
            Frame::Normal { id, data } => {
                let id: socketcan::CanId = id.clone().try_into()?;

                let frame = socketcan::CanDataFrame::new(id, &data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanAnyFrame::Normal(frame))
            }
        }
    }
}

impl TryFrom<Frame> for socketcan::CanFrame {
    type Error = Error;

    fn try_from(value: Frame) -> Result<Self, Error> {
        match &value {
            Frame::Normal { id, data } => {
                let id: socketcan::CanId = id.clone().try_into()?;

                let frame = socketcan::CanDataFrame::new(id, &data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanFrame::Data(frame))
            }
        }
    }
}

impl TryFrom<Id> for socketcan::CanId {
    type Error = Error;

    fn try_from(value: Id) -> Result<Self, Self::Error> {
        match value {
            Id::Standard(id) => {
                let id = id.as_raw();
                let id =
                    socketcan::StandardId::new(id).ok_or_else(|| Error::InvalidId(id as u64))?;
                Ok(socketcan::CanId::Standard(id))
            }
            Id::Extended(id) => {
                let id = id.as_raw();
                let id =
                    socketcan::ExtendedId::new(id).ok_or_else(|| Error::InvalidId(id as u64))?;
                Ok(socketcan::CanId::Extended(id))
            }
        }
    }
}
