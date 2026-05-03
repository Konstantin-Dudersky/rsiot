use esp_idf_svc::hal::{gpio::AnyIOPin, rmt::RmtChannel};
use serde::{Deserialize, Serialize};
use ws2812_esp32_rmt_driver::RGB8;

use crate::message::{Message, MsgDataBound};

pub type FnInput<TMsg> = fn(&Message<TMsg>) -> Option<Vec<(u8, ConfigRgb)>>;

/// Конфигурация cmp_esp_led
pub struct Config<TMsg, TRmt>
where
    TMsg: MsgDataBound,
    TRmt: RmtChannel + 'static,
{
    /// Пин для управляющего сигнала
    pub pin: AnyIOPin<'static>,

    /// Канал для управления сигналом RMT
    pub rmt_channel: TRmt,

    /// Функция преобразования входящих сообщений в значение цвета
    pub fn_input: FnInput<TMsg>,
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
