use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};

use super::{
    super::config::Buffer,
    {I2cRequest, TaskOutput},
};

pub struct InputPeriodic {
    pub output: TaskOutput<I2cRequest>,
    pub buffer: Arc<Mutex<Buffer>>,
    pub period: Duration,
}

impl InputPeriodic {
    pub async fn spawn(self) -> super::Result<()> {
        loop {
            let buffer = { self.buffer.lock().await.clone() };
            let request = I2cRequest::SetOutputs(buffer.into());
            self.output
                .send(request)
                .await
                .map_err(|_| super::Error::TokioTaskSend)?;
            sleep(self.period).await;
        }
    }
}
