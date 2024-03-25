use std::time::Duration;

use esp_idf_svc::hal::adc::{self, AdcChannelDriver, AdcDriver};
use tokio::{task::JoinSet, time::sleep};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, ConfigInput};

pub async fn fn_process<TMsg>(config: Config<TMsg>, _in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut adc1 =
        AdcDriver::new(config.adc1, &adc::config::Config::new().calibration(true)).unwrap();

    let mut task_set = JoinSet::new();

    for input in config.inputs {
        let mut adc_pin = match input.peripherals {
            ConfigInput::Gpio0(pin) => todo!(),
            ConfigInput::Gpio1(pin) => todo!(),
            ConfigInput::Gpio2(pin) => todo!(),
            ConfigInput::Gpio3(pin) => {
                AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pin).unwrap()
            }
            ConfigInput::Gpio4(pin) => todo!(),
            ConfigInput::Gpio5(pin) => todo!(),
        };
        task_set.spawn_local(async move {
            let sample: u16 = adc1.read(&mut adc_pin).unwrap();
            println!("{sample}");
            sleep(Duration::from_secs(2)).await;
        });
    }

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}
