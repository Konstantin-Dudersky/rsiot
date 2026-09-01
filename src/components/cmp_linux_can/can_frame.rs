use socketcan::EmbeddedFrame;
use tracing::warn;

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

            CanFrame::Error { id, data } => {
                let id: socketcan::CanId = id.clone().try_into()?;

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
                let id: socketcan::CanId = id.clone().try_into()?;

                let frame = socketcan::CanDataFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanFrame::Data(frame))
            }

            CanFrame::Error { id, data } => {
                let id: socketcan::CanId = id.clone().try_into()?;

                let frame = socketcan::CanErrorFrame::new(id, data)
                    .ok_or_else(|| Error::FrameConversion(value.clone()))?;

                Ok(socketcan::CanFrame::Error(frame))
            }
        }
    }
}

// impl TryFrom<socketcan::CanFrame> for CanFrame {
//     type Error = Error;

//     fn try_from(value: socketcan::CanFrame) -> Result<Self, Self::Error> {
//         match value {
//             socketcan::CanFrame::Data(frame) => {
//                 let id: socketcan::CanId = frame.id().into();
//                 let id: CanId = id.try_into()?;

//                 let mut data = [0_u8; 8];
//                 for (i, b) in frame.data().iter().enumerate() {
//                     data[i] = *b;
//                 }

//                 let frame = CanFrame::Normal { id, data };
//                 Ok(frame)
//             }
//             socketcan::CanFrame::Remote(_frame) => todo!(),
//             socketcan::CanFrame::Error(frame) => {
//                 warn!("CAN error frame: {frame:?}");
//                 let id: socketcan::CanId = frame.id().into();
//                 let id: CanId = id.try_into()?;

//                 let mut data = [0_u8; 8];
//                 for (i, b) in frame.data().iter().enumerate() {
//                     data[i] = *b;
//                 }
//                 let frame = CanFrame::Error { id, data };
//                 Ok(frame)
//             }
//         }
//     }
// }

// impl embedded_can::Frame for CanFrame {
//     fn new(id: impl Into<socketcan::Id>, data: &[u8]) -> Option<Self> {
//         todo!()
//     }

//     fn new_remote(id: impl Into<socketcan::Id>, dlc: usize) -> Option<Self> {
//         todo!()
//     }

//     fn is_extended(&self) -> bool {
//         todo!()
//     }

//     fn is_remote_frame(&self) -> bool {
//         todo!()
//     }

//     fn id(&self) -> embedded_can::Id {
//         match self {
//             CanFrame::Normal { id, .. } => {
//                 let id: embedded_can::Id = (*id).try_into().unwrap();
//                 id
//             }
//             CanFrame::Error1 { id, .. } => {
//                 let id: embedded_can::Id = (*id).try_into().unwrap();
//                 id
//             }
//         }
//     }

//     fn dlc(&self) -> usize {
//         todo!()
//     }

//     fn data(&self) -> &[u8] {
//         match self {
//             CanFrame::Normal { data, .. } => data,
//             CanFrame::Error1 { data, .. } => data,
//         }
//     }
// }
