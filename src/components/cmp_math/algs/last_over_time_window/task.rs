use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, task::JoinSet};

use crate::{
    executor::{MsgBusOutput, join_set_spawn},
    message::{MsgDataBound, ValueTime},
};

use super::{
    AlgFnOutputMsgbus, AlgInput, AlgOutput, Error, buffer::Buffer, task_input::TaskInput,
    task_output::TaskOutput,
};

pub struct Task<TMsg>
where
    TMsg: MsgDataBound,
{
    pub input: AlgInput,
    pub output: AlgOutput,
    pub output_msgbus: MsgBusOutput<TMsg>,
    pub time_window: Duration,
    pub fn_output: AlgFnOutputMsgbus<TMsg, f64>,
}

impl<TMsg> Task<TMsg>
where
    TMsg: 'static + MsgDataBound,
{
    pub async fn spawn(self) -> Result<(), Error> {
        let mut task_set = JoinSet::new();

        let buffer = Arc::new(Mutex::new(Buffer {
            last_value: ValueTime::default(),
            window: self.time_window,
        }));

        let task = TaskInput {
            input: self.input,
            buffer: buffer.clone(),
        };
        join_set_spawn(
            &mut task_set,
            "cmp_math | last_over_time_window | input",
            task.spawn(),
        );

        let task = TaskOutput {
            output: self.output,
            output_msgbus: self.output_msgbus,
            fn_output_msgbus: self.fn_output,
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
