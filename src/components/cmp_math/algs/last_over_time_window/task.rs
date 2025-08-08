use std::{sync::Arc, time::Duration};

use tokio::{
    sync::{broadcast, mpsc, Mutex},
    task::JoinSet,
};

use crate::executor::join_set_spawn;

use super::{
    buffer::Buffer, task_input::TaskInput, task_output::TaskOutput, Error, IntMsgBound, Result,
};

pub struct Task<TIntMsg>
where
    TIntMsg: IntMsgBound,
{
    pub input: broadcast::Receiver<TIntMsg>,
    pub output: mpsc::Sender<TIntMsg>,
    pub fn_input_value: fn(TIntMsg) -> Option<f64>,
    pub fn_input_window: fn(TIntMsg) -> Option<Duration>,
    pub fn_output: fn(f64) -> TIntMsg,
}

impl<TIntMsg> Task<TIntMsg>
where
    TIntMsg: 'static + IntMsgBound,
{
    pub async fn spawn(self) -> Result<()> {
        let mut task_set = JoinSet::new();

        let buffer = Arc::new(Mutex::new(Buffer {
            last_value: 0.0,
            window: Duration::from_millis(100),
        }));

        let task = TaskInput {
            input: self.input,
            fn_input_value: self.fn_input_value,
            fn_input_window: self.fn_input_window,
            buffer: buffer.clone(),
        };
        join_set_spawn(
            &mut task_set,
            "cmp_math | last_over_time_window | input",
            task.spawn(),
        );

        let task = TaskOutput {
            output: self.output,
            fn_output: self.fn_output,
            buffer: buffer.clone(),
        };
        join_set_spawn(
            &mut task_set,
            "cmp_math | last_over_time_window | output",
            task.spawn(),
        );

        while let Some(res) = task_set.join_next().await {
            res??;
        }

        let err = String::from("AlgLastOverTimeWindow");
        Err(Error::AlgTaskUnexpectedEnd(err))
    }
}
