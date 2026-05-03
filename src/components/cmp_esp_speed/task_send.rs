use std::time::Duration;

use tokio::sync::mpsc;
use tracing::warn;

use crate::{
    executor::{Instant, MsgBusOutput},
    message::{Message, MsgDataBound},
};

use super::{Error, int_msg::IntMsg};

pub struct TaskSend<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: mpsc::Receiver<IntMsg>,
    pub output: MsgBusOutput<TMsg>,
    pub period: Duration,
    pub fn_output: fn(f64) -> TMsg,
}

impl<TMsg> TaskSend<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn spawn(mut self) -> Result<(), Error> {
        let mut prev_period = Instant::now();

        while let Some(int_msg) = self.input.recv().await {
            match int_msg {
                IntMsg::Value {
                    elapsed,
                    count,
                    frequency,
                } => {
                    let msg = (self.fn_output)(frequency);
                    let msg = Message::new_custom(msg);

                    let res = self.output.try_send(msg);
                    if let Err(e) = res {
                        warn!("Failed to send message: {}", e);
                        return Err(Error::TaskEndCalculate);
                    }

                    prev_period = Instant::now();
                }
                IntMsg::Tick() => {
                    if prev_period.elapsed() > 2 * self.period {
                        let msg = (self.fn_output)(0.0);
                        let msg = Message::new_custom(msg);

                        let res = self.output.try_send(msg);
                        if let Err(e) = res {
                            warn!("Failed to send message: {}", e);
                            return Err(Error::TaskEndCalculate);
                        }

                        prev_period = Instant::now();
                    }
                }
            }
        }

        Err(Error::TaskEndCalculate)
    }
}
