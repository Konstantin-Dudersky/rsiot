use std::{fmt::Debug, sync::Arc, thread::sleep, time::Duration};

use esp_idf_hal::{
    delay::{TickType, BLOCK},
    i2c::I2cSlaveDriver,
    sys::{i2c_reset_rx_fifo, i2c_reset_tx_fifo, TickType_t},
};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::Mutex;
use tracing::{trace, warn};

use crate::{components::cmp_esp_i2c_slave::FnI2cComm, serde_utils::postcard_serde};

pub struct I2cComm<TI2cRequest, TI2cResponse, TBufferData> {
    pub i2c_slave: I2cSlaveDriver<'static>,
    pub fn_i2c_comm: FnI2cComm<TI2cRequest, TI2cResponse, TBufferData>,
    pub buffer_data: Arc<Mutex<TBufferData>>,
    pub start_delay: Duration,
}

/// Таймаут ожидания нового символа. Если задать 0, то будут ошибки передачи
const WAIT_SYMBOL_TIMEOUT: TickType_t = TickType::new_millis(5).ticks();

impl<TI2cRequest, TI2cResponse, TBufferData> I2cComm<TI2cRequest, TI2cResponse, TBufferData>
where
    TI2cRequest: Debug + DeserializeOwned + 'static,
    TI2cResponse: Debug + Serialize + 'static,
{
    pub fn spawn(mut self) -> super::Result<()> {
        sleep(self.start_delay);

        loop {
            trace!("Wait for request");

            // Ждем, пока появится в буфере байт
            let mut request_byte: [u8; 1] = [0];
            let res = self.i2c_slave.read(&mut request_byte, BLOCK);
            if let Err(err) = res {
                warn!("Error I2C slave: {}", err);
                continue;
            }

            let result = process_request(
                &mut self.i2c_slave,
                request_byte[0],
                self.fn_i2c_comm,
                self.buffer_data.clone(),
            );

            if let Err(err) = result {
                warn!("Error I2C slave: {}", err);
                unsafe { i2c_reset_rx_fifo(self.i2c_slave.port()) };

                continue;
            }
        }
    }
}

// TODO - сделать вызов fn_output после коммуникации
fn process_request<TI2cRequest, TI2cResponse, TBufferData>(
    i2c_slave: &mut I2cSlaveDriver,
    first_byte: u8,
    fn_i2c_comm: FnI2cComm<TI2cRequest, TI2cResponse, TBufferData>,
    buffer_data: Arc<Mutex<TBufferData>>,
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
    while i2c_slave
        .read(&mut request_byte, WAIT_SYMBOL_TIMEOUT)
        .is_ok()
    {
        request_buffer.push(request_byte[0]);
    }

    // Сбрасываем буфер отправки
    unsafe { i2c_reset_tx_fifo(i2c_slave.port()) };

    // Десериализация запроса
    let request: TI2cRequest = postcard_serde::deserialize(&mut request_buffer)?;
    trace!("Request: {:?}", request);

    // Определяем ответ по функции fn_i2c_comm
    let response = {
        let mut buffer_data = buffer_data.blocking_lock();
        (fn_i2c_comm)(request, &mut buffer_data).map_err(super::Error::FnI2cComm)?
    };
    trace!("Response: {:?}", response);

    // Сериализация ответа
    let response_buffer = postcard_serde::serialize(&response)?;

    // Запись в буфер отправки I2C
    let timeout = TickType::new_millis(100).ticks();
    i2c_slave
        .write(&response_buffer, timeout)
        .map_err(super::Error::WritingToI2cBuffer)?;

    Ok(())
}
