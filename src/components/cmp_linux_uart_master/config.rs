use std::time::Duration;

use crate::{
    components::shared_tasks::fieldbus_execution::FieldbusDiag,
    components_config::{
        master_device::DeviceTrait,
        uart_general::{Baudrate, DataBits, FieldbusRequest, FieldbusResponse, Parity, StopBits},
    },
    message::MsgDataBound,
};

// ANCHOR: Config
/// Конфигурация cmp_linux_uart_master
#[derive(Debug)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Название порта
    ///
    /// Примеры:
    ///
    /// ```
    /// port: "/dev/ttyUSB0"
    /// port: "/dev/ttyAMA0"
    /// ```
    pub port: &'static str,

    /// Скорость сетевого обмена
    pub baudrate: Baudrate,

    /// Кол-во бит данных
    pub data_bits: DataBits,

    /// Бит четности
    pub parity: Parity,

    /// Кол-во стоповых бит
    pub stop_bits: StopBits,

    /// Время ожидания ответа
    pub timeout: Duration,

    /// Название чипа gpio в системе.
    ///
    /// Примеры:
    ///
    /// ```rust
    /// gpio_chip: "/dev/gpiochip0"
    /// ```
    pub gpio_chip: &'static str,

    /// Номер пина для сигнала RTS (ready to send).
    ///
    /// Примеры:
    ///
    /// ```rust
    /// // На raspberry pi 17 пин - 11 физ. вывод на гребенке
    /// pin_rts: Some(17),
    ///
    /// // Если пин RTS не нужен
    /// pin_rts: None
    /// ```
    pub pin_rts: Option<u32>,

    /// Массив устройств
    pub devices: Vec<Box<dyn DeviceTrait<TMsg, FieldbusRequest, FieldbusResponse>>>,

    /// Функция для формирования сообщения диагностики
    pub fn_diag: fn(&FieldbusDiag) -> TMsg,

    /// Период отправки сообщений диагностики
    pub fn_diag_period: Duration,
}
// ANCHOR: Config

// impl<TMsg> Default for Config<TMsg>
// where
//     TMsg: MsgDataBound,
// {
//     fn default() -> Self {
//         Self {
//             port: "/dev/ttyAMA0",
//             baudrate: Baudrate::default(),
//             data_bits: DataBits::default(),
//             parity: Parity::default(),
//             stop_bits: StopBits::default(),
//             timeout: Duration::from_millis(100),
//             gpio_chip: "/dev/gpiochip0",
//             pin_rts: Some(17),
//             devices: vec![],
//             fn_diag: ,
//             fn_diag_period: Duration::from_secs(60),
//         }
//     }
// }
