use super::CanId;

/// CAN-кадр
#[derive(Clone, Debug)]
pub enum CanFrame {
    /// Стандартный CAN-кадр в классическом CAN 2.0
    Normal {
        /// Идентификатор
        id: CanId,
        /// Данные
        data: Vec<u8>,
    },
    // Remote(CanRemoteFrame),
    /// CAN-кадр ошибки
    Error {
        /// Идентификатор
        id: CanId,
        /// Данные
        data: Vec<u8>,
    },
    // Fd(CanFdFrame),
}

impl CanFrame {
    /// Возвращает идентификатор кадра
    pub fn id(&self) -> CanId {
        match self {
            CanFrame::Normal { id, .. } => *id,
            CanFrame::Error { id, .. } => *id,
        }
    }

    /// Возвращает данные кадра
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
            Self::Normal {
                id,
                data: value.data().to_vec(),
            }
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
