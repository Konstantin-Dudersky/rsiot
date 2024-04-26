use std::time::Duration;

use esp_idf_svc::hal::{i2c::I2cDriver, peripherals::Peripherals, units::FromValueType};
use tokio::time::sleep;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg>(_config: Config<TMsg>, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let peripherals = Peripherals::take().unwrap();

    let config = esp_idf_svc::hal::i2c::config::Config::new().baudrate(100_u32.kHz().into());

    let mut i2c = I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio9,
        peripherals.pins.gpio10,
        &config,
    );

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}
