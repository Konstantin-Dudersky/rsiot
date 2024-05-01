use std::{io::Cursor, sync::Arc, time::Duration};

use byteorder::{BigEndian, ReadBytesExt};
use tokio::{sync::Mutex, time::sleep};
use tracing::info;

use super::{super::RsiotI2cDriverBase, config};

pub struct ADS1115<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
{
    /// Адрес
    pub address: u8,

    /// Настройка
    pub inputs: Vec<config::InputConfig<TMsg>>,

    /// Ссылка на драйвер
    pub driver: Arc<Mutex<Driver>>,
}

impl<TMsg, Driver> ADS1115<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
{
    pub async fn spawn(&self) {
        loop {
            info!("Call ADS1115");
            {
                let mut driver = self.driver.lock().await;

                // Посылаем конфигурацию
                let mut request = vec![0x01];
                request.extend(config::config_to_bytes(
                    config::MuxConfig::Diff_0_1,
                    config::Amplifier::V_2_048,
                ));

                let response = driver.write(self.address, &request).await;
                info!("Write config: {:?}", response);

                sleep(Duration::from_millis(10)).await;

                // Читаем ответ
                let request = [0x00];
                let response = driver.write_read(self.address, &request, 2).await;
                if let Ok(response) = response {
                    let volt =
                        convert_response_to_voltage(&response, config::Amplifier::V_2_048).unwrap();
                    info!("Conversion: {}", volt);
                }
            }
            sleep(Duration::from_secs(2)).await;
        }
    }
}

fn convert_response_to_voltage(
    response: &[u8],
    amplfier: config::Amplifier,
) -> Result<f64, String> {
    let mut rdr = Cursor::new(response);
    let response = rdr.read_i16::<BigEndian>().unwrap();
    let max_scale = amplfier.max_value();
    let volt = response as f64 * max_scale / 32768.0;

    Ok(volt)
}

struct TaskInput<TMsg, Driver> {
    pub address: u8,
    pub inputs: config::InputConfig<TMsg>,
    pub driver: Arc<Mutex<Driver>>,
    pub period: Duration,
}

impl<TMsg, Driver> TaskInput<TMsg, Driver>
where
    Driver: RsiotI2cDriverBase,
{
    pub async fn spawn(&self) -> Result<(), String> {
        loop {
            info!("Call ADS1115");
            {
                let mut driver = self.driver.lock().await;

                // Посылаем конфигурацию
                let mut request = vec![0x01];
                request.extend(config::config_to_bytes(
                    &self.inputs.mux_config,
                    &self.inputs.amplifier,
                ));

                let response = driver.write(self.address, &request).await;
                info!("Write config: {:?}", response);

                sleep(Duration::from_millis(10)).await;

                // Читаем ответ
                let request = [0x00];
                let response = driver.write_read(self.address, &request, 2).await;
                if let Ok(response) = response {
                    let volt =
                        convert_response_to_voltage(&response, config::Amplifier::V_2_048).unwrap();
                    info!("Conversion: {}", volt);
                }
            }

            sleep(self.period).await
        }
    }
}
