use crate::{
    drivers_i2c::I2cSlaveAddress,
    message::{Message, MsgDataBound},
};

pub type FnInput<TMsg> = fn(&Message<TMsg>, &mut Buffer) -> ();

/// Настройка модулуля коммуникации с модулем PM-RQ8
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Адрес модуля
    pub address: I2cSlaveAddress,
    /// Функция преобразования входящих сообщений в команды для модуля
    pub fn_input: FnInput<TMsg>,
}

/// Буфер данных коммуникации с модулем PM-RQ8
#[derive(Clone, Debug, Default)]
pub struct Buffer {
    /// Состояние входа 0
    pub output_0: bool,
    /// Состояние входа 1
    pub output_1: bool,
    /// Состояние входа 2
    pub output_2: bool,
    /// Состояние входа 3
    pub output_3: bool,
    /// Состояние входа 4
    pub output_4: bool,
    /// Состояние входа 5
    pub output_5: bool,
    /// Состояние входа 6
    pub output_6: bool,
    /// Состояние входа 7
    pub output_7: bool,
}

impl From<Buffer> for u8 {
    fn from(value: Buffer) -> Self {
        let mut sum = 0;
        if value.output_0 {
            sum += 2_u8.pow(0);
        }
        if value.output_1 {
            sum += 2_u8.pow(1);
        }
        if value.output_2 {
            sum += 2_u8.pow(2);
        }
        if value.output_3 {
            sum += 2_u8.pow(3);
        }
        if value.output_4 {
            sum += 2_u8.pow(4);
        }
        if value.output_5 {
            sum += 2_u8.pow(5);
        }
        if value.output_6 {
            sum += 2_u8.pow(6);
        }
        if value.output_7 {
            sum += 2_u8.pow(7);
        }
        sum
    }
}
