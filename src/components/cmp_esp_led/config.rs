use esp_idf_svc::hal::{gpio::AnyIOPin, peripheral::Peripheral, rmt::RmtChannel};
use serde::{Deserialize, Serialize};
use ws2812_esp32_rmt_driver::RGB8;

use crate::message::{Message, MsgDataBound};

/// Конфигурация cmp_esp_led
pub struct Config<TMsg, TPeripheral, TRmt>
where
    TMsg: MsgDataBound,
    TPeripheral: RmtChannel,
    TRmt: Peripheral<P = TPeripheral> + 'static,
{
    /// Пин для управляющего сигнала
    pub pin: AnyIOPin,

    /// Канал для управления сигналом RMT
    pub rmt_channel: TRmt,

    /// Функция преобразования входящих сообщений в значение цвета
    pub fn_input: fn(&Message<TMsg>) -> Option<Vec<ConfigRgb>>,
}

/// Задание цвета
#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Serialize)]
pub struct ConfigRgb
where
    Self: Sized,
{
    /// R
    pub r: u8,
    /// G
    pub g: u8,
    /// B
    pub b: u8,
}

impl From<ConfigRgb> for RGB8 {
    fn from(value: ConfigRgb) -> Self {
        RGB8 {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}
