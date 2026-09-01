use tracing::warn;

use super::CanId;

/// CAN-кадр
#[derive(Clone, Copy, Debug)]
pub enum CanFrame {
    /// Стандартный CAN-кадр в классическом CAN 2.0
    Normal {
        /// Идентификатор
        id: CanId,
        /// Данные
        data: [u8; 8],
    },
    // Remote(CanRemoteFrame),
    /// CAN-кадр ошибки
    Error {
        /// Идентификатор
        id: CanId,
        /// Данные
        data: [u8; 8],
    },
    // Fd(CanFdFrame),
}

impl CanFrame {
    pub fn id(&self) -> CanId {
        match self {
            CanFrame::Normal { id, .. } => *id,
            CanFrame::Error { id, .. } => *id,
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            CanFrame::Normal { data, .. } => data,
            CanFrame::Error { data, .. } => data,
        }
    }
}

impl<TEmbedFrame> From<TEmbedFrame> for CanFrame
where
    TEmbedFrame: embedded_can::Frame,
{
    fn from(value: TEmbedFrame) -> Self {
        let id: CanId = value.id().into();
        if value.is_data_frame() {
            let mut data = [0_u8; 8];
            let len = value.data().len().min(8);
            if len > 8 {
                warn!("Data length exceeds 8 bytes: {}", len);
            }
            data[..len].copy_from_slice(&value.data()[..len]);

            Self::Normal { id, data }
        } else {
            todo!()
        }
    }
}

// impl<TEmbedFrame> From<CanFrame> for TEmbedFrame
// where
//     TEmbedFrame: embedded_can::Frame,
// {
//     fn from(value: CanFrame) -> Self {}
// }
