use crate::drivers_i2c::I2cDevices;
use crate::message::MsgDataBound;

/// Конфигурация cmp_raspberrypi_i2c_master
#[derive(Clone)]
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Конфигурация устройств по шине
    pub devices: Vec<I2cDevices<TMsg>>,
}
