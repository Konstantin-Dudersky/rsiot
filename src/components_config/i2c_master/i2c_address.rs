/// Адрес подчиненного устройства
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2cAddress {
    /// Прямое подключение
    Direct {
        /// Адрес подчиненного устройства
        address: u8,
    },
    /// Через мультиплексор
    Mux {
        /// Адрес мультиплексора
        mux_address: u8,

        /// Канал на мультиплексоре. 0..7
        channel: u8,

        /// Адрес подчиненного устройства
        address: u8,
    },
}

impl Default for I2cAddress {
    fn default() -> Self {
        Self::Direct { address: 0x00 }
    }
}
