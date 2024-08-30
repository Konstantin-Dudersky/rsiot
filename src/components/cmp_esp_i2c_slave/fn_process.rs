use std::fmt::Debug;

use esp_idf_hal::{
    delay::{TickType, BLOCK},
    i2c::{I2c, I2cSlaveConfig, I2cSlaveDriver},
    peripheral::Peripheral,
    sys::i2c_reset_tx_fifo,
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::task::JoinSet;
use tracing::{debug, trace, warn};

use crate::{drivers_i2c::postcard_serde, executor::CmpInOut, message::MsgDataBound};

use super::{BufferData, Config, Error, FnI2cComm};

const BUFFER_LEN: usize = 128;

pub async fn fn_process<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>(
    config: Config<TMsg, TI2c, TPeripheral, TI2cRequest, TI2cResponse, TBufferData>,
    _in_out: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound + 'static,
    TI2c: Peripheral<P = TPeripheral> + 'static,
    TPeripheral: I2c,
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
    TBufferData: BufferData,
{
    let i2c_idf_config = I2cSlaveConfig::new()
        .sda_enable_pullup(false)
        .scl_enable_pullup(false)
        .tx_buffer_length(BUFFER_LEN)
        .rx_buffer_length(BUFFER_LEN);

    let mut i2c_slave = I2cSlaveDriver::new(
        config.i2c,
        config.sda,
        config.scl,
        config.slave_address,
        &i2c_idf_config,
    )
    .unwrap();

    debug!("I2c slave drive initialized");

    let mut task_set: JoinSet<()> = JoinSet::new();

    task_set.spawn_blocking(move || loop {
        trace!("Wait for request");

        // Ждем, пока появится в буфере байт
        let mut request_byte: [u8; 1] = [0];
        let res = i2c_slave.read(&mut request_byte, BLOCK);
        if let Err(err) = res {
            warn!("Error I2C slave: {}", err);
            continue;
        }

        let result = process_request(&mut i2c_slave, request_byte[0], config.fn_i2c_comm);

        if let Err(err) = result {
            warn!("Error I2C slave: {}", err);
            continue;
        }
    });

    while let Some(res) = task_set.join_next().await {
        res.unwrap();
    }

    Ok(())
}

fn process_request<TI2cRequest, TI2cResponse>(
    i2c_slave: &mut I2cSlaveDriver,
    first_byte: u8,
    fn_i2c_comm: FnI2cComm<TI2cRequest, TI2cResponse>,
) -> super::Result<()>
where
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
{
    // Чтение буфера приема I2C
    // Копируем данные из входного буфера побайтово. Если скопировать сразу несколько байт,
    // могут появляться смещения
    let mut request_buffer = vec![];
    request_buffer.push(first_byte);
    let mut request_byte: [u8; 1] = [0];
    while i2c_slave.read(&mut request_byte, 0).is_ok() {
        request_buffer.push(request_byte[0]);
    }

    // Сбрасываем буфер отправки
    unsafe { i2c_reset_tx_fifo(i2c_slave.port()) };

    // Десериализация запроса
    let request: TI2cRequest = postcard_serde::deserialize(&mut request_buffer)?;
    trace!("Request: {:?}", request);

    // Определяем ответ по функции fn_i2c_comm
    let response = (fn_i2c_comm)(request).map_err(Error::FnI2cComm)?;
    trace!("Response: {:?}", response);

    // Сериализация ответа
    let response_buffer = postcard_serde::serialize(&response)?;

    // Запись в буфер отправки I2C
    let timeout = TickType::new_millis(5000).ticks();
    i2c_slave
        .write(&response_buffer, timeout)
        .map_err(Error::WritingToI2cBuffer)?;

    Ok(())
}

// TODO - сделать расчет CRC - падает stack protection fault
