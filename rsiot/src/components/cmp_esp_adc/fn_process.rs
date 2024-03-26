use std::{sync::Arc, time::Duration};

use esp_idf_svc::hal::{
    adc::{self, AdcChannelDriver, AdcDriver},
    gpio::ADCPin,
};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{Config, ConfigInput, ConfigInputAttenuation, ConfigInputType};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: CmpInOut<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    let adc1 = AdcDriver::new(config.adc1, &adc::config::Config::new().calibration(true)).unwrap();
    let adc1 = Arc::new(Mutex::new(adc1));

    let adc2 = AdcDriver::new(config.adc2, &adc::config::Config::new().calibration(true)).unwrap();
    let adc2 = Arc::new(Mutex::new(adc2));

    let mut task_set = JoinSet::new();

    for input in config.inputs {
        let mut adc_pin = match input.peripherals {
            ConfigInputType::Gpio0(_) => todo!(),
            ConfigInputType::Gpio1(_) => todo!(),
            ConfigInputType::Gpio2(pin) => {
                AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pin).unwrap()
            }
            ConfigInputType::Gpio3(pin) => match input.attenuation {
                ConfigInputAttenuation::Db11 => {
                    AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pin).unwrap()
                }
            },
            ConfigInputType::Gpio4(_) => todo!(),
            ConfigInputType::Gpio5(_) => todo!(),
        };

        let adc_clone = adc1.clone();
        let in_out_clone = in_out.clone();
        task_set.spawn_local(async move {
            let mut lock = adc_clone.lock().await;
            loop {
                let sample: u16 = lock.read(&mut adc_pin).unwrap();
                let msg = (input.fn_output)(sample);
                in_out_clone.send_output(msg).await.unwrap();
                sleep(input.update_period).await;
            }
        });
    }

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}

async fn input(pin: impl ADCPin, attenuation: ConfigInputAttenuation) {
    match attenuation {
        ConfigInputAttenuation::DB6 => {
            AdcChannelDriver::<{ adc::attenuation::DB_6 }, _>::new(pin).unwrap()
        }
        ConfigInputAttenuation::Db11 => {
            AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pin).unwrap()
        }
    };
}
