use crate::{
    drivers_i2c::I2cSlaveAddress,
    message::{Message, MsgDataBound},
};

pub type FnInput<TMsg> = fn(&Message<TMsg>, &mut Buffer) -> ();

#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: I2cSlaveAddress,
    pub fn_input: FnInput<TMsg>,
}

#[derive(Clone, Debug, Default)]
pub struct Buffer {
    pub output_0: bool,
    pub output_1: bool,
    pub output_2: bool,
    pub output_3: bool,
    pub output_4: bool,
    pub output_5: bool,
    pub output_6: bool,
    pub output_7: bool,
}

impl Into<u8> for Buffer {
    fn into(self) -> u8 {
        let mut value = 0;
        if self.output_0 {
            value += 2_u8.pow(0);
        }
        if self.output_1 {
            value += 2_u8.pow(1);
        }
        if self.output_2 {
            value += 2_u8.pow(2);
        }
        if self.output_3 {
            value += 2_u8.pow(3);
        }
        if self.output_4 {
            value += 2_u8.pow(4);
        }
        if self.output_5 {
            value += 2_u8.pow(5);
        }
        if self.output_6 {
            value += 2_u8.pow(6);
        }
        if self.output_7 {
            value += 2_u8.pow(7);
        }
        value
    }
}
