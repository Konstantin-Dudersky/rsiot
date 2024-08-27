use std::sync::Arc;

use esp_idf_hal::{
    i2c::{I2c, I2cSlaveConfig, I2cSlaveDriver},
    peripheral::Peripheral,
};
use tokio::{sync::Mutex, task::JoinSet};
use tracing::info;

use crate::{executor::CmpInOut, message::MsgDataBound};

use super::{tasks, Config};

pub async fn fn_process<TMsg, TI2c, TPeripheral>(
    config: Config<TMsg, TI2c, TPeripheral>,
    in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
{
    let i2c_idf_config = I2cSlaveConfig::new()
        .sda_enable_pullup(false)
        .scl_enable_pullup(false)
        .tx_buffer_length(config.tx_buf_len)
        .rx_buffer_length(config.rx_buf_len);

    let driver = I2cSlaveDriver::new(
        config.i2c,
        config.sda,
        config.scl,
        config.slave_address,
        &i2c_idf_config,
    )
    .unwrap();
    let driver = Arc::new(Mutex::new(driver));

    info!("I2c slave drive initialized");

    let mut task_set = JoinSet::new();

    // Обработка входящих сообщений
    let task = tasks::Input {
        msg_bus: in_out.clone(),
        driver: driver.clone(),
        fn_input: config.fn_input,
    };
    task_set.spawn_local(task.spawn());

    // Генерирование исходящих сообщений
    let task = tasks::Output {
        msg_bus: in_out.clone(),
        driver: driver.clone(),
        fn_output: config.fn_output,
    };
    task_set.spawn_local(task.spawn());

    while let Some(res) = task_set.join_next().await {
        res.unwrap().unwrap();
    }

    Ok(())
}
