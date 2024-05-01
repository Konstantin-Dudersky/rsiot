use crate::message::{Message, MsgDataBound};

/// Конфигурации устройств по шине I2C
#[derive(Clone)]
pub enum I2cDevices<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Аналого-цифровой преобразователь
    ADS1115 {
        /// Адрес. Зависит от подключения входа ADDR:
        /// - GND - 0x48
        /// - VDD - 0x49
        /// - SDA - 0x4A
        /// - SCL - 0x4B
        address: u8,

        /// Настройка входов
        inputs: Vec<super::ads1115::config::InputConfig<TMsg>>,
    },

    /// Датчик давления BMP180
    BMP180 {
        /// Адрес. По-умолчанию 0x77
        address: u8,
        /// Функция преобразования данных в исходящие сообщения
        fn_output: fn(super::bmp180::BMP180Data) -> Vec<Message<TMsg>>,
        /// Кол-во измерений для определения значения
        oversampling: super::bmp180::BMP180Oversampling,
    },

    /// Расширение GPIO PCF8575
    PCF8575 {
        /// Адрес
        address: u8,
        /// Настройка пина P00
        pin_00: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P01
        pin_01: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P02
        pin_02: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P03
        pin_03: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P04
        pin_04: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P05
        pin_05: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P06
        pin_06: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P07
        pin_07: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P10
        pin_10: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P11
        pin_11: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P12
        pin_12: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P13
        pin_13: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P14
        pin_14: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P15
        pin_15: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P16
        pin_16: super::PCF8575PinMode<TMsg>,
        /// Настройка пина P17
        pin_17: super::PCF8575PinMode<TMsg>,
    },
}