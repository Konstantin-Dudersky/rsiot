use std::collections::VecDeque;

use tokio::sync::{broadcast, mpsc};

use super::{Error, IntMsgBound, Result};

pub struct Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub input: broadcast::Receiver<TIntMsg>,
    pub output: mpsc::Sender<TIntMsg>,
    pub fn_input_value: fn(TIntMsg) -> Option<f64>,
    pub fn_input_count: fn(TIntMsg) -> Option<usize>,
    pub fn_output: fn(f64) -> TIntMsg,
}

impl<TIntMsg> Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        let mut count = 0;
        let mut buffer: VecDeque<f64> = VecDeque::new();

        while let Ok(input_int_msg) = self.input.recv().await {
            if let Some(new_count) = (self.fn_input_count)(input_int_msg) {
                count = new_count;
            }
            if let Some(value) = (self.fn_input_value)(input_int_msg) {
                buffer.push_back(value);
                while buffer.len() > count {
                    buffer.pop_front();
                }

                let avg = buffer.iter().sum::<f64>() / buffer.len() as f64;
                let output_int_msg = (self.fn_output)(avg);
                self.output.send(output_int_msg).await.map_err(|_| {
                    Error::AlgTaskUnexpectedEnd(String::from("AlgLastOverTimeWindow"))
                })?;
            };
        }

        let err = String::from("AlgLastOverTimeWindow");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}
