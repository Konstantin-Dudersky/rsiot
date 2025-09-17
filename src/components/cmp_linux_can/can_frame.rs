use socketcan::EmbeddedFrame;

use super::{CanFrame, CanId, Error};

impl TryFrom<CanFrame> for socketcan::CanAnyFrame {
    type Error = Error;

    fn try_from(value: CanFrame) -> Result<Self, Error> {
        match &value {
            CanFrame::Normal { id, data } => {
                let id: socketcan::CanId = id.clone().try_into()?;

                let frame = socketcan::CanDataFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanAnyFrame::Normal(frame))
            }
        }
    }
}

impl TryFrom<CanFrame> for socketcan::CanFrame {
    type Error = Error;

    fn try_from(value: CanFrame) -> Result<Self, Error> {
        match &value {
            CanFrame::Normal { id, data } => {
                let id: socketcan::CanId = id.clone().try_into()?;

                let frame = socketcan::CanDataFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanFrame::Data(frame))
            }
        }
    }
}

impl TryFrom<socketcan::CanFrame> for CanFrame {
    type Error = Error;

    fn try_from(value: socketcan::CanFrame) -> Result<Self, Self::Error> {
        match value {
            socketcan::CanFrame::Data(frame) => {
                let id: socketcan::CanId = frame.id().into();
                let id: CanId = id.try_into()?;

                let mut data = [0_u8; 8];
                for (i, b) in frame.data().iter().enumerate() {
                    data[i] = *b;
                }

                let frame = CanFrame::Normal { id, data };
                Ok(frame)
            }
            socketcan::CanFrame::Remote(_frame) => todo!(),
            socketcan::CanFrame::Error(_frame) => todo!(),
        }
    }
}
