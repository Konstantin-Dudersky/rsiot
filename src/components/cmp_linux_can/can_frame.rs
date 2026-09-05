use socketcan::EmbeddedFrame;

use super::{CanFrame, Error};

impl TryFrom<CanFrame> for socketcan::CanAnyFrame {
    type Error = Error;

    fn try_from(value: CanFrame) -> Result<Self, Error> {
        match &value {
            CanFrame::Normal { id, data } => {
                let id: socketcan::CanId = (*id).try_into()?;

                let frame = socketcan::CanDataFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanAnyFrame::Normal(frame))
            }

            CanFrame::Error { id, data } => {
                let id: socketcan::CanId = (*id).try_into()?;

                let frame = socketcan::CanErrorFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanAnyFrame::Error(frame))
            }
        }
    }
}

impl TryFrom<CanFrame> for socketcan::CanFrame {
    type Error = Error;

    fn try_from(value: CanFrame) -> Result<Self, Error> {
        match &value {
            CanFrame::Normal { id, data } => {
                let id: socketcan::CanId = (*id).try_into()?;

                let frame = socketcan::CanDataFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanFrame::Data(frame))
            }

            CanFrame::Error { id, data } => {
                let id: socketcan::CanId = (*id).try_into()?;

                let frame = socketcan::CanErrorFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanFrame::Error(frame))
            }
        }
    }
}
