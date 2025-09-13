use crate::{
    components_config::{i2c_master, master_device::DeviceTrait},
    message::MsgDataBound,
};

// ANCHOR: Config
/// Конфигурация компонента cmp_linux_i2c_master.
pub struct Config<TMsg>
where
    TMsg: MsgDataBound,
{
    /// Устройство I2C, например "/dev/i2c-0"
    pub dev_i2c: String,

    /// Драйвера устройств
    pub devices:
        Vec<Box<dyn DeviceTrait<TMsg, i2c_master::FieldbusRequest, i2c_master::FieldbusResponse>>>,
}
// ANCHOR: Config
