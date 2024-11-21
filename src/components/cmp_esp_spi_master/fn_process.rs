use esp_idf_svc::hal::{
    peripheral::Peripheral,
    spi::{Spi, SpiAnyPins},
};
use futures::TryFutureExt;
use tokio::{sync::mpsc::channel, task::JoinSet};

use crate::{
    components::shared_tasks,
    executor::{join_set_spawn, CmpInOut},
    message::{Message, MsgDataBound},
};

use super::{tasks, Config, InnerMessage};

pub async fn fn_process<TMsg, TSpi, TPeripheral>(
    config: Config<TMsg, TSpi, TPeripheral>,
    msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TSpi: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: Spi + SpiAnyPins + 'static,
{
    let mut task_set = JoinSet::new();

    const BUFFER_SIZE: usize = 100;
    let (ch_input_to_spi_comm_send, ch_input_to_spi_comm_recv) =
        channel::<InnerMessage<TMsg>>(BUFFER_SIZE);
    let (ch_spi_comm_to_filter_send, ch_spi_comm_to_filter_recv) =
        channel::<Message<TMsg>>(BUFFER_SIZE);
    let (ch_filter_to_msgbus_send, ch_filter_to_msgbus_recv) =
        channel::<Message<TMsg>>(BUFFER_SIZE);

    // Оборачиваем входящие сообщения в InnerMessage
    let task = tasks::Input {
        input: msg_bus.clone(),
        output: ch_input_to_spi_comm_send.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Оборачиваем периодический вызов в InnerMessage
    let task = tasks::Period {
        output: ch_input_to_spi_comm_send.clone(),
        period: config.fn_output_period,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Коммуникация SPI
    let task = tasks::SpiComm {
        input: ch_input_to_spi_comm_recv,
        output: ch_spi_comm_to_filter_send,
        config,
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Фильтрация одинаковых сообщений
    let task = shared_tasks::filter_identical_data::FilterIdenticalData {
        input: ch_spi_comm_to_filter_recv,
        output: ch_filter_to_msgbus_send,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskFilter),
    );

    // Создаем исходящие сообщения
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: ch_filter_to_msgbus_recv,
        cmp_in_out: msg_bus,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(super::Error::TaskMpscToMsgBus),
    );

    while let Some(res) = task_set.join_next().await {
        res??
    }

    Ok(())
}
