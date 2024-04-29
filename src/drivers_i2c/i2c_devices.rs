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
        /// Режим работы пина P00
        pin_00: super::PCF8575PinMode<TMsg>,
    },
}
