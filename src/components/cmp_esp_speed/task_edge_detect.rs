use esp_idf_svc::hal::gpio::{AnyIOPin, Level, PinDriver};
use tokio::sync::mpsc::{self, error::TrySendError};
use tracing::warn;

use super::Error;

pub struct TaskEdgeDetect {
    pub output: mpsc::Sender<()>,
    pub pin_speed: AnyIOPin,
    pub pin_led: Option<AnyIOPin>,
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

        loop {
            let level = pin_speed.get_level();
            let level = gpio_level_to_bool(&level);

            if let Some(pin_led) = pin_led.as_mut() {
                match level {
                    true => pin_led.set_high().map_err(Error::SetGpioOutput)?,
                    false => pin_led.set_low().map_err(Error::SetGpioOutput)?,
                }
            }

            if level && !prev_value {
                let res = self.output.try_send(());

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
