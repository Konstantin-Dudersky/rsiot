use crate::message::{Message, MsgDataBound};

/// Конфигурации устройств по шине I2C
#[derive(Clone)]
pub enum I2cDevices<TMsg>
where
    TMsg: MsgDataBound,
{
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
        pin_00: super::PCF8575PinMode<TMsg>,
        pin_01: super::PCF8575PinMode<TMsg>,
        pin_02: super::PCF8575PinMode<TMsg>,
        pin_03: super::PCF8575PinMode<TMsg>,
        pin_04: super::PCF8575PinMode<TMsg>,
        pin_05: super::PCF8575PinMode<TMsg>,
        pin_06: super::PCF8575PinMode<TMsg>,
        pin_07: super::PCF8575PinMode<TMsg>,
        pin_10: super::PCF8575PinMode<TMsg>,
        pin_11: super::PCF8575PinMode<TMsg>,
        pin_12: super::PCF8575PinMode<TMsg>,
        pin_13: super::PCF8575PinMode<TMsg>,
        pin_14: super::PCF8575PinMode<TMsg>,
        pin_15: super::PCF8575PinMode<TMsg>,
        pin_16: super::PCF8575PinMode<TMsg>,
        pin_17: super::PCF8575PinMode<TMsg>,
    },
}
