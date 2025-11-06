use std::time::Duration;

use tokio::sync::mpsc;
use tracing::warn;

use crate::executor::sleep;

use super::{BufferBound, COMPONENT_NAME, Error, internal_message::InternalMessage};

pub struct Period<TBuffer>
where
    TBuffer: BufferBound,
{
    pub output: mpsc::Sender<InternalMessage<TBuffer>>,
    pub period: Duration,
}

impl<TBuffer> Period<TBuffer>
where
    TBuffer: BufferBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        loop {
            let int_msg = InternalMessage::Period(());

            let res = self.output.try_send(int_msg);

            if res.is_err() {
                warn!("Bus in component {COMPONENT_NAME} is full");
            }

            sleep(self.period).await;
        }
    }
}
