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
    // Error(CanErrorFrame),
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
        }
    }
}
