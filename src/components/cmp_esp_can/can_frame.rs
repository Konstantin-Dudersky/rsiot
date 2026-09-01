use enumset::EnumSet;
use esp_idf_svc::hal::can::Flags;
// use esp_idf_svc::hal::can::Frame;

use crate::components_config::can_general::{CanFrame, CanId};

use super::Error;

// impl TryFrom<esp_idf_svc::hal::can::Frame> for CanFrame {
//     type Error = Error;

//     fn try_from(value: esp_idf_svc::hal::can::Frame) -> Result<Self, Self::Error> {
//         let id: CanId = value.id().into();
//         let frame = if value.is_data_frame() {
//             let mut data = [0_u8; 8];
//             for (i, b) in value.data().iter().enumerate() {
//                 data[i] = *b;
//             }
//             Self::Normal { id, data }
//         } else {
//             todo!()
//         };

//         Ok(frame)
//     }
// }

impl TryFrom<CanFrame> for esp_idf_svc::hal::can::Frame {
    type Error = Error;

    fn try_from(value: CanFrame) -> Result<Self, Self::Error> {
        let frame = match value {
            CanFrame::Normal { id, data } => {
                let mut flags = EnumSet::new();
                if matches!(id, CanId::Extended(_)) {
                    flags.insert(Flags::Extended);
                }

                let id = id.as_raw();

                let frame = esp_idf_svc::hal::can::Frame::new(id, flags, &data);
                let Some(frame) = frame else {
                    return Err(Error::FrameConversionIntoField(value));
                };
                frame
            }
            CanFrame::Error { id, data } => todo!(),
        };
        Ok(frame)
    }
}
