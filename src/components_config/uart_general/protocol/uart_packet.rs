use serde::{Deserialize, Serialize};

/// Пакет для передачи данных по UART
#[derive(Debug, Deserialize, Serialize)]
pub struct UartPacket<TData> {
    /// Адрес устройства
    pub address: u8,
    /// Данные для передачи
    pub data: TData,
}
