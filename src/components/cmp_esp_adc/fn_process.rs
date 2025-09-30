use std::{sync::Arc, time::Duration};

use esp_idf_svc::hal::adc::AdcChannelDriver;
use esp_idf_svc::hal::adc::{self, AdcDriver};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};

use crate::{
    executor::MsgBusLinker,
    message::{Message, MsgDataBound},
};

use super::{Config, ConfigInputType};

pub async fn fn_process<TMsg>(config: Config<TMsg>, in_out: MsgBusLinker<TMsg>) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
{
    // Пока не придумал способа, как можно работать с этими структурами в общем виде
    // https://github.com/esp-rs/esp-idf-hal/issues/209

    let adc1 = AdcDriver::new(config.adc1, &adc::config::Config::new().calibration(true)).unwrap();
    let adc1 = Arc::new(Mutex::new(adc1));

    let adc2 = AdcDriver::new(config.adc2, &adc::config::Config::new().calibration(true)).unwrap();
    let adc2 = Arc::new(Mutex::new(adc2));

    let mut task_set = JoinSet::new();

    for input in config.inputs {
        let adc1_clone = adc1.clone();
        let _adc2_clone = adc2.clone();
        let in_out_clone = in_out.clone();

        match input.peripherals {
            ConfigInputType::Gpio0(_) => todo!(),
            ConfigInputType::Gpio1(_) => todo!(),
            ConfigInputType::Gpio2(pin) => task_set.spawn_local(async move {
                let mut driver =
                    AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pin).unwrap();
                loop {
                    let sample: u16 = adc1_clone.lock().await.read(&mut driver).unwrap();
                    postprocess(
                        in_out_clone.clone(),
                        input.fn_output,
                        input.update_period,
                        sample,
                    )
                    .await;
                }
            }),
            ConfigInputType::Gpio3(pin) => task_set.spawn_local(async move {
                let mut driver =
                    AdcChannelDriver::<{ adc::attenuation::DB_11 }, _>::new(pin).unwrap();
                loop {
                    let sample: u16 = adc1_clone.lock().await.read(&mut driver).unwrap();
                    postprocess(
                        in_out_clone.clone(),
                        input.fn_output,
                        input.update_period,
                        sample,
                    )
                    .await;
                }
            }),
            ConfigInputType::Gpio4(_) => todo!(),
            ConfigInputType::Gpio5(_) => todo!(),
        };
    }

    loop {
        sleep(Duration::from_secs(2)).await;
    }
}

async fn postprocess<TMsg>(
    in_out: MsgBusLinker<TMsg>,
    fn_output: fn(u16) -> Message<TMsg>,
    update_period: Duration,
    sample: u16,
) where
    TMsg: MsgDataBound,
{
    let msg = (fn_output)(sample);
    in_out.send_output(msg).await.unwrap();
    sleep(update_period).await;
}
