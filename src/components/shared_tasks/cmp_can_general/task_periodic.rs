use std::{sync::Arc, time::Duration};

use futures::TryFutureExt;
use tokio::{
    sync::{Mutex, mpsc},
    time::sleep,
};
use tracing::warn;

use crate::components_config::can_general::{BufferBound, Frame};

pub struct Periodic<TBuffer, TError>
where
    TBuffer: BufferBound,
{
    pub output: mpsc::Sender<Frame>,
    pub buffer: Arc<Mutex<TBuffer>>,
    pub period: Duration,
    pub fn_periodic: fn(&TBuffer) -> anyhow::Result<Option<Vec<Frame>>>,
    pub error_tokio_mpsc_send: fn() -> TError,
}

impl<TBuffer, TError> Periodic<TBuffer, TError>
where
    TBuffer: BufferBound,
{
    pub async fn spawn(self) -> Result<(), TError> {
        loop {
            sleep(self.period).await;

            let frames = {
                let buffer = self.buffer.lock().await;
                (self.fn_periodic)(&buffer)
            };
            let frames = match frames {
                Ok(v) => v,
                Err(e) => {
                    warn!("Error in fn_periodic of CAN Periodic task: {}", e);
                    continue;
                }
            };
            let Some(frames) = frames else { continue };

            for frame in frames {
                self.output
                    .send(frame)
                    .await
                    .map_err(|_| (self.error_tokio_mpsc_send)())?;
            }
        }
    }
}
