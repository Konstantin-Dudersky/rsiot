use std::sync::Arc;

use tokio::sync::Mutex;

use super::{AlgInput, Error, buffer::Buffer};

pub struct TaskInput {
    pub input: AlgInput,
    pub buffer: Arc<Mutex<Buffer>>,
}
impl TaskInput {
    pub async fn spawn(mut self) -> Result<(), Error> {
        while let Some(input_value) = self.input.recv().await {
            let mut buffer = self.buffer.lock().await;
            buffer.last_value = input_value;
        }

        let err = String::from("AlgLastOverTimeWindow - TaskInput");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}
