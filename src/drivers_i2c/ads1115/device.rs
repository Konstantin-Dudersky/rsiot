use std::{io::Cursor, sync::Arc, time::Duration};

use byteorder::{BigEndian, ReadBytesExt};
use tokio::{sync::Mutex, task::JoinSet, time::sleep};
use tracing::warn;

use crate::{
    executor::CmpInOut,
    message::{MsgDataBound, ServiceBound},
};

use super::{
    super::{I2cSlaveAddress, RsiotI2cDriverBase},
    config,
};

/// АЦП ADS1115
pub struct ADS1115<TMsg, TService, Driver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    Driver: RsiotI2cDriverBase,
{
    /// Адрес
    pub address: I2cSlaveAddress,

    /// Настройка
    pub inputs: Vec<config::InputConfig<TMsg>>,

    /// Ссылка на драйвер
    pub driver: Arc<Mutex<Driver>>,

    pub cmp_in_out: CmpInOut<TMsg, TService>,
}

impl<TMsg, TService, Driver> ADS1115<TMsg, TService, Driver>
where
    TMsg: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    Driver: RsiotI2cDriverBase + 'static,
{
    pub async fn spawn(&self) {
        loop {
            let mut task_set: JoinSet<Result<(), String>> = JoinSet::new();

            for input in &self.inputs {
                let driver = self.driver.clone();
                let input = input.clone();
                let cmp_in_out = self.cmp_in_out.clone();
                let task = TaskInput {
                    address: self.address,
                    input,
                    driver,
                    cmp_in_out,
                };
                task_set.spawn(async move { task.spawn().await });
            }

            while let Some(res) = task_set.join_next().await {
                warn!("ADS1150 stop execution: {:?}", res);
                task_set.abort_all();
            }

            sleep(Duration::from_secs(2)).await;
        }
    }
}

fn convert_response_to_voltage(
    response: &[u8],
    amplfier: &config::Amplifier,
) -> Result<f64, String> {
    let mut rdr = Cursor::new(response);
    let response = rdr.read_i16::<BigEndian>().unwrap();
    let max_scale = amplfier.max_value();
    let volt = response as f64 * max_scale / 32768.0;

    Ok(volt)
}

struct TaskInput<TMsg, TService, Driver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    Driver: RsiotI2cDriverBase,
{
    pub address: I2cSlaveAddress,
    pub input: config::InputConfig<TMsg>,
    pub driver: Arc<Mutex<Driver>>,
    pub cmp_in_out: CmpInOut<TMsg, TService>,
}

impl<TMsg, TService, Driver> TaskInput<TMsg, TService, Driver>
where
    TMsg: MsgDataBound,
    TService: ServiceBound,
    Driver: RsiotI2cDriverBase,
{
    pub async fn spawn(&self) -> Result<(), String> {
        loop {
            {
                let mut driver = self.driver.lock().await;

                // Посылаем конфигурацию
                let mut request = vec![0x01];
                request.extend(config::config_to_bytes(
                    &self.input.mux_config,
                    &self.input.amplifier,
                ));

                let _ = driver
                    .write(self.address, &request, Duration::from_secs(2))
                    .await;

                sleep(Duration::from_millis(10)).await;

                // Читаем ответ
                let request = [0x00];
                let response = driver
                    .write_read(self.address, &request, 2, Duration::from_secs(2))
                    .await
                    .map_err(String::from)?;
                let volt = convert_response_to_voltage(&response, &self.input.amplifier).unwrap();

                // Обрабатываем исходящие сообщения
                let msg = (self.input.fn_output)(volt);
                let Some(msg) = msg else { continue };
                self.cmp_in_out
                    .send_output(msg)
                    .await
                    .map_err(|e| e.to_string())?;
            }

            sleep(self.input.period).await
        }
    }
}
