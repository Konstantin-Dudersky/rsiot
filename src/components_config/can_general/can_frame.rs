use super::CanId;

/// CAN-кадр
#[derive(Clone, Debug)]
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
