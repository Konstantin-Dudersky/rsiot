use std::time::Duration;

use tokio::sync::mpsc;
use tracing::warn;

use crate::{
    executor::{Instant, MsgBusOutput},
    message::{Message, MsgDataBound},
};

use super::Error;

pub struct TaskCalculate<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<()>,
    pub output: MsgBusOutput<TMsg>,
    pub period: Duration,
    pub fn_output: fn(f64) -> TMsg,
}

impl<TMsg> TaskCalculate<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut counter: u32 = 0;
        let mut prev_period = Instant::now();

        while (self.input.recv().await).is_some() {
            counter += 1;

            if prev_period.elapsed() > self.period {
                let micros = prev_period.elapsed().as_micros() as f64;
                prev_period = Instant::now();
                let frequency = counter as f64 * 1000000.0 / micros;
                counter = 0;

                let msg = (self.fn_output)(frequency);
                let msg = Message::new_custom(msg);
                let res = self.output.try_send(msg);

                if let Err(e) = res {
                    warn!("Failed to send message: {}", e);
                    return Err(Error::TaskEndCalculate);
                }
            }
        }

        Err(Error::TaskEndCalculate)
    }
}
