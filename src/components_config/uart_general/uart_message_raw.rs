/// Сообщение по UART в байтах
pub type UartMessageRaw<const MESSAGE_LEN: usize> = [u8; MESSAGE_LEN];
