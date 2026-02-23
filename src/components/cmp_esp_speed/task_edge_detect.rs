use std::time::Duration;

use esp_idf_svc::hal::gpio::{AnyIOPin, Level, PinDriver};
use tokio::sync::mpsc::{self, error::TrySendError};
use tracing::warn;

use crate::executor::Instant;

use super::{Error, int_msg::IntMsg};

pub struct TaskEdgeDetect {
    pub output: mpsc::Sender<IntMsg>,
    pub pin_speed: AnyIOPin,
    pub pin_led: Option<AnyIOPin>,
    pub period: Duration,
}

impl TaskEdgeDetect {
    pub async fn spawn(self) -> Result<(), Error> {
        let mut pin_speed = PinDriver::input(self.pin_speed).map_err(Error::CreatePinDriver)?;

        let mut pin_led = match self.pin_led {
            Some(pin_led) => {
                let pin_led = PinDriver::output(pin_led).map_err(Error::CreatePinDriver)?;
                Some(pin_led)
            }
            None => None,
        };

        let mut prev_value = false;
        let mut counter: u32 = 0;
        let mut prev_period = Instant::now();

        loop {
            let level = pin_speed.get_level();
            let level = gpio_level_to_bool(&level);

            // Устанавливаем состояние светодиода состояния
            if let Some(pin_led) = pin_led.as_mut() {
                match level {
                    true => pin_led.set_high().map_err(Error::SetGpioOutput)?,
                    false => pin_led.set_low().map_err(Error::SetGpioOutput)?,
                }
            }

            if level && !prev_value {
                counter += 1;

                if prev_period.elapsed() > self.period {
                    let elapsed = prev_period.elapsed();

                    let micros = elapsed.as_micros() as f64;
                    let frequency = counter as f64 * 1000000.0 / micros;

                    let int_msg = IntMsg::Value {
                        elapsed,
                        count: counter,
                        frequency,
                    };

                    prev_period = Instant::now();
                    counter = 0;

                    let res = self.output.try_send(int_msg);
                    if let Err(e) = res {
                        match e {
                            TrySendError::Full(_) => {
                                warn!("Message queue is full");
                            }
                            TrySendError::Closed(_) => {
                                warn!("Channel closed");
                                return Err(Error::TaskEndEdgeDetect);
                            }
                        }
                    }
                }
            }
            prev_value = level;

            pin_speed
                .wait_for_any_edge()
                .await
                .map_err(Error::WaitForAnyEdge)?;
        }
    }
}

fn gpio_level_to_bool(level: &Level) -> bool {
    match level {
        Level::Low => false,
        Level::High => true,
    }
}
