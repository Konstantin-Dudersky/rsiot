use std::time::Duration;

use tokio::{
    sync::mpsc::{self, error::TrySendError},
    time::sleep,
};
use tracing::warn;

use super::{Error, int_msg::IntMsg};

pub struct TaskTick {
    pub output: mpsc::Sender<IntMsg>,
    pub period: Duration,
}

impl TaskTick {
    pub async fn spawn(self) -> Result<(), Error> {
        loop {
            sleep(self.period).await;
            let int_msg = IntMsg::Tick();

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
}
