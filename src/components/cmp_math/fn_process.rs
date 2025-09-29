use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    executor::{CmpInOut, MsgBusInput, MsgBusOutput, join_set_spawn},
    message::{Message, MsgDataBound},
};

use super::{Algs, Config, Error, IntMsgBound, Result, algs};

pub async fn fn_process<TMsg, TIntMsg>(
    config: Config<TMsg, TIntMsg>,
    msgbus_linker: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TIntMsg: 'static + IntMsgBound,
{
    let mut task_set = JoinSet::new();

    let buffer_size = msgbus_linker.max_capacity();

    let (ch_tx_into_alg, ch_rx_into_alg) = broadcast::channel::<TIntMsg>(buffer_size);
    let (ch_tx_from_alg, ch_rx_from_alg) = mpsc::channel::<TIntMsg>(buffer_size);

    let task = TaskInput {
        input: msgbus_linker.input(),
        output: ch_tx_into_alg.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_math | input", task.spawn());

    for alg in config.algs {
        match alg {
            Algs::LastOverTimeWindow {
                fn_input_value,
                fn_input_window,
                fn_output,
            } => {
                let task = algs::last_over_time_window::Task {
                    input: ch_rx_into_alg.resubscribe(),
                    output: ch_tx_from_alg.clone(),
                    fn_input_value,
                    fn_output,
                    fn_input_window,
                };
                join_set_spawn(
                    &mut task_set,
                    "cmp_math | last_over_time_window",
                    task.spawn(),
                );
            }
            Algs::SimpleMovingAverage {
                fn_input_value,
                fn_input_count,
                fn_output,
            } => {
                let task = algs::simple_moving_average::Task {
                    input: ch_rx_into_alg.resubscribe(),
                    output: ch_tx_from_alg.clone(),
                    fn_input_value,
                    fn_input_count,
                    fn_output,
                };
                join_set_spawn(
                    &mut task_set,
                    "cmp_math | simple_moving_average",
                    task.spawn(),
                );
            }

            Algs::SMA {
                fn_input_value,
                fn_input_time_window,
                fn_output,
            } => {
                let task = algs::sma::Task {
                    input: ch_rx_into_alg.resubscribe(),
                    output: ch_tx_from_alg.clone(),
                    fn_input_value,
                    fn_input_time_window,
                    fn_output,
                };
                join_set_spawn(&mut task_set, "cmp_math | sma", task.spawn());
            }

            Algs::EMA {
                kind,
                fn_input_value,
                fn_input_time_window,
                fn_output,
            } => {
                let task = algs::ema::Task {
                    input: ch_rx_into_alg.resubscribe(),
                    output: ch_tx_from_alg.clone(),
                    fn_input_value,
                    fn_input_time_window,
                    fn_output,
                    kind,
                };
                join_set_spawn(&mut task_set, "cmp_math | sma", task.spawn());
            }

            Algs::Derivative {
                fn_input_value,
                fn_input_time_window,
                normalization_time,
                gamma,
                fn_output,
            } => {
                let task = algs::derivative::Task {
                    input: ch_rx_into_alg.resubscribe(),
                    output: ch_tx_from_alg.clone(),
                    fn_input_value,
                    fn_input_time_window,
                    normalization_time,
                    gamma,
                    fn_output,
                };
                join_set_spawn(&mut task_set, "cmp_math | derivative", task.spawn());
            }
        };
    }

    let task = TaskOutput {
        input: ch_rx_from_alg,
        output_to_algs: ch_tx_into_alg,
        output_to_msgbus: msgbus_linker.output(),
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, "cmp_math | output", task.spawn());

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::FnProcessEnd)
}

struct TaskInput<TMsg, TIntMsg>
where
    TMsg: MsgDataBound,
    TIntMsg: IntMsgBound,
{
    pub input: MsgBusInput<TMsg>,
    pub output: broadcast::Sender<TIntMsg>,
    pub fn_input: fn(TMsg) -> Option<TIntMsg>,
}
impl<TMsg, TIntMsg> TaskInput<TMsg, TIntMsg>
where
    TMsg: MsgDataBound,
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Ok(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            if let Some(int_msg) = (self.fn_input)(msg) {
                self.output.send(int_msg).map_err(|_| Error::TaskInputEnd)?;
            }
        }
        Err(Error::TaskInputEnd)
    }
}

struct TaskOutput<TMsg, TIntMsg>
where
    TMsg: MsgDataBound,
    TIntMsg: IntMsgBound,
{
    pub input: mpsc::Receiver<TIntMsg>,
    pub output_to_algs: broadcast::Sender<TIntMsg>,
    pub output_to_msgbus: MsgBusOutput<TMsg>,
    pub fn_output: fn(TIntMsg) -> Option<Vec<TMsg>>,
}
impl<TMsg, TIntMsg> TaskOutput<TMsg, TIntMsg>
where
    TMsg: MsgDataBound,
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(int_msg) = self.input.recv().await {
            self.output_to_algs
                .send(int_msg)
                .map_err(|_| Error::TaskOutputEnd)?;
            let msgs = (self.fn_output)(int_msg);
            let Some(msgs) = msgs else { continue };
            for msg in msgs {
                self.output_to_msgbus
                    .send(Message::new_custom(msg))
                    .await
                    .map_err(|_| Error::TaskOutputEnd)?;
            }
        }
        Err(Error::TaskOutputEnd)
    }
}
