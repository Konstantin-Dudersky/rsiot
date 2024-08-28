use std::sync::Arc;

use esp_idf_hal::{
    delay::BLOCK,
    i2c::{I2c, I2cSlaveConfig, I2cSlaveDriver},
    peripheral::Peripheral,
    sys::i2c_reset_rx_fifo,
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

    let mut i2c_slave = I2cSlaveDriver::new(
        config.i2c,
        config.sda,
        config.scl,
        config.slave_address,
        &i2c_idf_config,
    )
    .unwrap();
    // let driver = Arc::new(Mutex::new(driver));

    info!("I2c slave drive initialized");

    let mut task_set: JoinSet<()> = JoinSet::new();

    task_set.spawn_blocking(move || loop {
        let mut reg_addr: [u8; 10] = [0; 10];
        let res = i2c_slave.read(&mut reg_addr, BLOCK);
        info!("Read result: {:?}", res);
        info!("Buffer: {:?}", reg_addr);
    });

    // task_set.spawn_blocking(move || {
    //     let mut data: [u8; 256] = [0; 256];
    //     loop {
    //         let mut reg_addr: [u8; 1] = [0];
    //         let res = i2c_slave.read(&mut reg_addr, BLOCK);
    //         if res.is_err() {
    //             println!(
    //                 "SLAVE: failed to read register address from master: Error: {:?}",
    //                 res
    //             );
    //             continue;
    //         }
    //         let mut rx_data: [u8; 1] = [0];
    //         match i2c_slave.read(&mut rx_data, 0) {
    //             Ok(size) => {
    //                 println!(
    //                     "SLAVE: write operation {:#04x} to reg addr {:#04x}; size: {size}",
    //                     rx_data[0], reg_addr[0]
    //                 );
    //                 data[reg_addr[0] as usize] = rx_data[0];
    //             }
    //             Err(_) => {
    //                 let d = data[reg_addr[0] as usize];
    //                 println!(
    //                     "SLAVE: read operation {:#04x} from reg addr {:#04x}",
    //                     d, reg_addr[0]
    //                 );
    //                 i2c_slave.write(&[d], BLOCK).unwrap();
    //             }
    //         }
    //     }
    // });

    // // Обработка входящих сообщений
    // let task = tasks::Input {
    //     msg_bus: in_out.clone(),
    //     driver: i2c_slave.clone(),
    //     fn_input: config.fn_input,
    // };
    // task_set.spawn_local(task.spawn());

    // // Генерирование исходящих сообщений
    // let task = tasks::Output {
    //     msg_bus: in_out.clone(),
    //     driver: i2c_slave.clone(),
    //     fn_output: config.fn_output,
    // };
    // task_set.spawn_local(task.spawn());

    while let Some(res) = task_set.join_next().await {
        res.unwrap();
    }

    Ok(())
}
