use std::{fmt::Debug, sync::Arc};

use esp_idf_svc::hal::{
    i2c::{I2c, I2cSlaveConfig, I2cSlaveDriver},
    peripheral::Peripheral,
};
use futures::TryFutureExt;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinSet,
};
use tracing::debug;

use crate::{
    components::{cmp_esp_i2c_slave::tasks, shared_tasks},
    executor::{join_set_spawn, CmpInOut},
    message::{MsgDataBound, ServiceBound},
};

use super::{BufferData, Config, Error};

/// Размер буферов приема и отправки
const BUFFER_LEN: usize = 128;

pub async fn fn_process<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData, TService>(
    config: Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>,
    msg_bus: CmpInOut<TMsg, TService>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
    TBufferData: BufferData + 'static,
    TService: ServiceBound + 'static,
{
    let i2c_idf_config = I2cSlaveConfig::new()
        .sda_enable_pullup(false)
        .scl_enable_pullup(false)
        .tx_buffer_length(BUFFER_LEN)
        .rx_buffer_length(BUFFER_LEN);
    let i2c_slave = I2cSlaveDriver::new(
        config.i2c,
        config.sda,
        config.scl,
        config.slave_address,
        &i2c_idf_config,
    )
    .unwrap();

    let buffer_data = Arc::new(Mutex::new(config.buffer_data_default.clone()));

    debug!("I2c slave drive initialized");

    let buffer_size = msg_bus.max_capacity();
    let (channel_buffer_to_filter_send, channel_buffer_to_filter_recv) = mpsc::channel(buffer_size);
    let (channel_filter_to_output_send, channel_filter_to_output_recv) = mpsc::channel(buffer_size);

    let mut task_set: JoinSet<super::Result<()>> = JoinSet::new();

    // Задача коммуникации I2C
    let task = tasks::I2cComm {
        i2c_slave,
        fn_i2c_comm: config.fn_i2c_comm,
        buffer_data: buffer_data.clone(),
        start_delay: config.start_i2ccomm_delay,
    };
    task_set.spawn_blocking(move || task.spawn());

    // Задача обработки входящих сообщений
    let task = tasks::Input {
        msg_bus: msg_bus.clone(),
        fn_input: config.fn_input,
        buffer_data: buffer_data.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Задача создания исходящих сообщений
    let task = tasks::Output {
        output: channel_buffer_to_filter_send,
        fn_output: config.fn_output,
        fn_output_period: config.fn_output_period,
        buffer_data: buffer_data.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn());

    // Фильтрация исходящих сообщений
    let task = shared_tasks::filter_identical_data::FilterIdenticalData {
        input: channel_buffer_to_filter_recv,
        output: channel_filter_to_output_send,
    };
    join_set_spawn(
        &mut task_set,
        task.spawn().map_err(Error::TaskFilterIdenticalData),
    );

    // Пересылка сообщений на выход компонента
    let task = shared_tasks::mpsc_to_msgbus::MpscToMsgBus {
        input: channel_filter_to_output_recv,
        cmp_in_out: msg_bus.clone(),
    };
    join_set_spawn(&mut task_set, task.spawn().map_err(Error::TaskToMsgBus));

    while let Some(res) = task_set.join_next().await {
        res.unwrap()?;
    }

    Ok(())
}
