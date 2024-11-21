use crate::serde_utils::postcard_serde;

/// Сообщение по UART в байтах
pub type UartMessageRaw = [u8; postcard_serde::MESSAGE_LEN];
