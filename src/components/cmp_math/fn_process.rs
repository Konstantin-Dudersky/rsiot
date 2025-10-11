use tokio::{sync::mpsc, task::JoinSet};

use crate::{
    executor::{MsgBusLinker, join_set_spawn},
    message::MsgDataBound,
};

use super::{
    Algs, Config, ConfigBranch, Error, algs, task_input::TaskInput, task_output::TaskOutput,
};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> Result<(), Error>
where
    TMsg: 'static + MsgDataBound,
{
    let mut task_set = JoinSet::new();

    for (index, branch) in config.branches.into_iter().enumerate() {
        setup_branch(index, &msgbus_linker, &mut task_set, branch);
    }

    msgbus_linker.close();

    while let Some(res) = task_set.join_next().await {
        res??;
    }

    Err(Error::FnProcessEnd)
}

pub fn setup_branch<TMsg>(
    index: usize,
    msgbus_linker: &MsgBusLinker<TMsg>,
    task_set: &mut JoinSet<Result<(), Error>>,
    branch: ConfigBranch<TMsg>,
) where
    TMsg: 'static + MsgDataBound,
{
    let (ch_tx, ch_rx) = mpsc::channel(msgbus_linker.max_capacity());

    let task = TaskInput {
        input: msgbus_linker.input(),
        output: ch_tx,
        fn_input: branch.fn_input,
    };
    join_set_spawn(
        task_set,
        format!("cmp_math | branch {index} | input"),
        task.spawn(),
    );

    let mut ch_rx_prev = ch_rx;

    for alg in branch.algs.into_iter() {
        let (ch_tx, ch_rx) = mpsc::channel(msgbus_linker.max_capacity());

        match alg {
            Algs::Derivative {
                time_window,
                normalization_time,
                gamma,
                fn_output_msgbus,
            } => {
                let task = algs::derivative::Task {
                    input: ch_rx_prev,
                    output: ch_tx,
                    output_msgbus: msgbus_linker.output(),
                    time_window,
                    normalization_time,
                    gamma,
                    fn_output_msgbus,
                };
                join_set_spawn(
                    task_set,
                    format!("cmp_math | branch {index} | derivative"),
                    task.spawn(),
                );
            }

            Algs::Downsampling {
                time_window,
                fn_output_msgbus,
            } => {
                let task = algs::downsampling::Task {
                    input: ch_rx_prev,
                    output: ch_tx,
                    output_msgbus: msgbus_linker.output(),
                    time_window,
                    fn_output_msgbus,
                };
                join_set_spawn(
                    task_set,
                    format!("cmp_math | branch {index} | downsampling"),
                    task.spawn(),
                );
            }

            Algs::EMA {
                kind,
                time_window,
                fn_output_msgbus,
            } => {
                let task = algs::ema::Task {
                    input: ch_rx_prev,
                    output: ch_tx,
                    output_msgbus: msgbus_linker.output(),
                    kind,
                    time_window,
                    fn_output_msgbus,
                };
                join_set_spawn(
                    task_set,
                    format!("cmp_math | branch {index} | EMA"),
                    task.spawn(),
                );
            }

            Algs::LastOverTimeWindow {
                time_window,
                fn_output_msgbus: fn_output,
            } => {
                let task = algs::last_over_time_window::Task {
                    input: ch_rx_prev,
                    output: ch_tx,
                    output_msgbus: msgbus_linker.output(),
                    time_window,
                    fn_output,
                };
                join_set_spawn(
                    task_set,
                    format!("cmp_math | branch {index} | last_over_time_window"),
                    task.spawn(),
                );
            }

            Algs::SMA {
                time_window,
                fn_output_msgbus,
            } => {
                let task = algs::sma::Task {
                    input: ch_rx_prev,
                    output: ch_tx,
                    output_msgbus: msgbus_linker.output(),
                    time_window,
                    fn_output_msgbus,
                };
                join_set_spawn(
                    task_set,
                    format!("cmp_math | branch {index} | SMA"),
                    task.spawn(),
                );
            }

            Algs::Statistic {
                time_window,
                indicators,
                fn_output_msgbus,
            } => {
                let task = algs::statistic::Task {
                    input: ch_rx_prev,
                    output: ch_tx,
                    output_msgbus: msgbus_linker.output(),
                    time_window,
                    indicators,
                    fn_output_msgbus,
                };
                join_set_spawn(
                    task_set,
                    format!("cmp_math | branch {index} | statistic"),
                    task.spawn(),
                );
            }
        };
        ch_rx_prev = ch_rx;
    }

    let task = TaskOutput {
        input: ch_rx_prev,
        output: msgbus_linker.output(),
        fn_output: branch.fn_output,
    };
    join_set_spawn(
        task_set,
        format!("cmp_math | branch {index} | output"),
        task.spawn(),
    );
}
