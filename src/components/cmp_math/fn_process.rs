use futures::TryFutureExt;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinSet,
};

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::{Message, MsgDataBound},
};

use super::{algs, Algs, Config, Error, IntMsgBound, Result};

const BUFFER_SIZE: usize = 1000;

pub async fn fn_process<TMsg, TIntMsg>(
    config: Config<TMsg, TIntMsg>,
    msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TIntMsg: 'static + IntMsgBound,
{
    let mut task_set = JoinSet::new();

    let (ch_tx_msgbus_to_input, ch_rx_msgbus_to_input) =
        mpsc::channel::<Message<TMsg>>(BUFFER_SIZE);
    let (ch_tx_input_to_alg, ch_rx_input_to_alg) = mpsc::channel::<TIntMsg>(BUFFER_SIZE);
    let (ch_tx_into_alg, ch_rx_into_alg) = broadcast::channel::<TIntMsg>(BUFFER_SIZE);
    let (ch_tx_from_alg, ch_rx_from_alg) = mpsc::channel::<TIntMsg>(BUFFER_SIZE);
    let (ch_tx_output_to_msgbus, ch_rx_output_to_msgbus) =
        mpsc::channel::<Message<TMsg>>(BUFFER_SIZE);

    let task = shared_tasks::msgbus_to_mpsc::MsgBusToMpsc {
        msg_bus: msg_bus.clone(),
        output: ch_tx_msgbus_to_input,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_math",
        task.spawn().map_err(Error::TaskMsgBusToMpsc),
    );

    let task = TaskInput {
        input: ch_rx_msgbus_to_input,
        output: ch_tx_input_to_alg.clone(),
        fn_input: config.fn_input,
    };
    join_set_spawn(&mut task_set, "cmp_math", task.spawn());

    let task = shared_tasks::mpsc_to_broadcast::Task {
        input: ch_rx_input_to_alg,
        output: ch_tx_into_alg,
    };
    join_set_spawn(
        &mut task_set,
        "cmp_math",
        task.spawn().map_err(Error::TaskMpscToBroadcast),
    );

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
                join_set_spawn(&mut task_set, "cmp_math", task.spawn());
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
                join_set_spawn(&mut task_set, "cmp_math", task.spawn());
            }
        };
    }

    let task = TaskOutput {
        input: ch_rx_from_alg,
        output_to_algs: ch_tx_input_to_alg.clone(),
        output_to_msgbus: ch_tx_output_to_msgbus,
        fn_output: config.fn_output,
    };
    join_set_spawn(&mut task_set, "cmp_math", task.spawn());

    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_rx_output_to_msgbus,
        msg_bus: msg_bus.clone(),
    };
    join_set_spawn(
        &mut task_set,
        "cmp_math",
        task.spawn().map_err(Error::TaskMpscToMsgbus),
    );

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
    pub input: mpsc::Receiver<Message<TMsg>>,
    pub output: mpsc::Sender<TIntMsg>,
    pub fn_input: fn(TMsg) -> Option<TIntMsg>,
}
impl<TMsg, TIntMsg> TaskInput<TMsg, TIntMsg>
where
    TMsg: MsgDataBound,
    TIntMsg: IntMsgBound,
{
    pub async fn spawn(mut self) -> Result<()> {
        while let Some(msg) = self.input.recv().await {
            let Some(msg) = msg.get_custom_data() else {
                continue;
            };
            if let Some(int_msg) = (self.fn_input)(msg) {
                self.output
                    .send(int_msg)
                    .await
                    .map_err(|_| Error::TaskInputEnd)?;
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
    pub output_to_algs: mpsc::Sender<TIntMsg>,
    pub output_to_msgbus: mpsc::Sender<Message<TMsg>>,
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
                .await
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
