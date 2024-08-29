use std::fmt::Debug;

use esp_idf_hal::{
    delay::BLOCK,
    i2c::{I2c, I2cSlaveConfig, I2cSlaveDriver},
    peripheral::Peripheral,
    sys::{i2c_reset_rx_fifo, i2c_reset_tx_fifo},
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::task::JoinSet;
use tracing::{info, warn};

use crate::{drivers_i2c::postcard_serde, executor::CmpInOut, message::MsgDataBound};

use super::Config;

pub async fn fn_process<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>(
    config: Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse>,
    _in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
{
    let i2c_idf_config = I2cSlaveConfig::new()
        .sda_enable_pullup(false)
        .scl_enable_pullup(false)
        .tx_buffer_length(config.buffer_len)
        .rx_buffer_length(config.buffer_len);

    let mut i2c_slave = I2cSlaveDriver::new(
        config.i2c,
        config.sda,
        config.scl,
        config.slave_address,
        &i2c_idf_config,
    )
    .unwrap();

    info!("I2c slave drive initialized");

    let mut task_set: JoinSet<()> = JoinSet::new();

    task_set.spawn_blocking(move || loop {
        // info!("Wait for request");
        // let mut request_buffer: [u8; postcard_serde::MESSAGE_LEN] =
        //     [0; postcard_serde::MESSAGE_LEN];

        let mut request_buffer = vec![];
        let mut request_buffer_1: [u8; 1] = [0];
        let _ = i2c_slave.read(&mut request_buffer_1, BLOCK);
        request_buffer.push(request_buffer_1[0]);
        while i2c_slave.read(&mut request_buffer_1, 0).is_ok() {
            request_buffer.push(request_buffer_1[0]);
        }

        // unsafe { i2c_reset_rx_fifo(0) };
        unsafe { i2c_reset_tx_fifo(0) };

        // if let Err(err) = res {
        //     warn!("Error reading buffer: {}", err);
        //     continue;
        // }

        // info!("Request buffer: {:?}", request_buffer);
        let request: Result<TI2cRequest, _> = postcard_serde::deserialize(&mut request_buffer);
        let request = match request {
            Ok(val) => val,
            Err(err) => {
                let err = format!("Deserialization error: {}", err);
                warn!("{}", err);
                continue;
            }
        };
        // info!("Request: {:?}", request);

        let response = (config.fn_master_comm)(request);
        let response = match response {
            Ok(val) => val,
            Err(err) => {
                let err = format!("{}", err);
                warn!("{}", err);
                continue;
            }
        };
        // info!("Response: {:?}", response);

        let response_buffer = postcard_serde::serialize(&response);
        let response_buffer = match response_buffer {
            Ok(val) => val,
            Err(err) => {
                let err = format!("Serialization error: {}", err);
                warn!("{}", err);
                continue;
            }
        };

        let res = i2c_slave.write(&response_buffer, BLOCK);
        if let Err(err) = res {
            warn!("Error writing to buffer: {}", err);
            continue;
        }
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

// TODO - сделать расчет CRC - падает stack protection fault
