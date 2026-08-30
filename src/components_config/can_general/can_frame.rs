use super::CanId;

// ANCHOR: CanFrame
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
    Error1 {
        /// Идентификатор
        id: CanId,
        /// Данные
        data: [u8; 8],
    },
    // Fd(CanFdFrame),
}
// ANCHOR: CanFrame

impl CanFrame {
    /// Размер CAN-кадра в битах
    pub fn frame_size(&self) -> f32 {
        match self {
            CanFrame::Normal { id, data: _ } => match id {
                CanId::Standard(_) => 111.0,
                CanId::Extended(_) => 131.0,
            },
            CanFrame::Error1 { id, data: _ } => match id {
                CanId::Standard(_) => 111.0,
                CanId::Extended(_) => 131.0,
            },
        }
    }
}

impl embedded_can::Frame for CanFrame {
    fn new(id: impl Into<socketcan::Id>, data: &[u8]) -> Option<Self> {
        todo!()
    }

    fn new_remote(id: impl Into<socketcan::Id>, dlc: usize) -> Option<Self> {
        todo!()
    }

    fn is_extended(&self) -> bool {
        todo!()
    }

    fn is_remote_frame(&self) -> bool {
        todo!()
    }

    fn id(&self) -> socketcan::Id {
        todo!()
    }

    fn dlc(&self) -> usize {
        todo!()
    }

    fn data(&self) -> &[u8] {
        todo!()
    }
}
