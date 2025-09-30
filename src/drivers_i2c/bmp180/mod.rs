//! Датчик давления BMP180

mod calculate_values;

use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::{sync::Mutex, time::sleep};
use tracing::{info, trace, warn};

use crate::{
    executor::MsgBusLinker,
    message::{Message, MsgDataBound, PhyQuantity},
};

use super::{I2cSlaveAddress, RsiotI2cDriverBase};

use calculate_values::calculate_values;

pub struct BMP180<TMsg>
where
    TMsg: MsgDataBound,
{
    pub address: I2cSlaveAddress,
    pub fn_output: fn(BMP180Data) -> Vec<Message<TMsg>>,
    pub oversampling: BMP180Oversampling,
    pub cmp_in_out: MsgBusLinker<TMsg>,
}

impl<TMsg> BMP180<TMsg>
where
    TMsg: MsgDataBound,
{
    pub async fn fn_process(&self, driver: Arc<Mutex<impl RsiotI2cDriverBase>>) {
        let calibration_data: CalibrationData = loop {
            let calibration_data = request_calibration_data(driver.clone(), self.address).await;
            match calibration_data {
                Ok(calibration_data) => break calibration_data.into(),
                Err(err) => {
                    warn!("Error getting calibration data for sensor BMP180: {}", err);
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
            }
        };

        loop {
            let temperature = request_temperature(driver.clone(), self.address).await;
            let Ok(temperature) = temperature else {
                warn!("Error reading temperature from BMP180: {temperature:?}");
                sleep(Duration::from_secs(2)).await;
                continue;
            };

            let pressure = request_pressure(driver.clone(), self.address, self.oversampling).await;
            let Ok(pressure) = pressure else {
                warn!("Error reading pressure from BMP180: {pressure:?}");
                sleep(Duration::from_secs(2)).await;
                continue;
            };

            let values = calculate_values(
                &calibration_data,
                &temperature,
                &pressure,
                self.oversampling,
            );
            let msgs = (self.fn_output)(values);
            for msg in msgs {
                self.cmp_in_out.send_output(msg).await.unwrap();
            }
            sleep(Duration::from_secs(2)).await;
        }
    }
}

/// Запрос данных калибровки. Необходимо выполнить один раз при запуске
async fn request_calibration_data(
    driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    address: I2cSlaveAddress,
) -> Result<ResponseCalibrationData, String> {
    let mut response;
    {
        let mut driver = driver.lock().await;
        response = driver
            .write_read(address, &[0xAA], 22, Duration::from_secs(2))
            .await?;
    }
    swap_msb_lsb(&mut response);
    let response =
        bincode::deserialize::<ResponseCalibrationData>(&response).map_err(|e| e.to_string())?;
    info!("Calibration data BMP180: {response:?}");
    Ok(response)
}

/// Запрос на измерение температуры
async fn request_temperature(
    driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    address: I2cSlaveAddress,
) -> Result<ResponseUncompensatedTemperature, String> {
    let mut response;
    {
        let mut driver = driver.lock().await;
        driver
            .write(address, &[0xF4, 0x2E], Duration::from_secs(2))
            .await?;
    }
    sleep(Duration::from_millis(5)).await;
    {
        let mut driver = driver.lock().await;
        response = driver
            .write_read(address, &[0xF6], 2, Duration::from_secs(2))
            .await?;
    }
    swap_msb_lsb(&mut response);
    let response = bincode::deserialize::<ResponseUncompensatedTemperature>(&response)
        .map_err(|e| e.to_string())?;
    trace!("BMP180 temperature: {response:?}");
    Ok(response)
}

/// Запрос на измерение давления
#[allow(non_snake_case)]
async fn request_pressure(
    driver: Arc<Mutex<impl RsiotI2cDriverBase>>,
    address: I2cSlaveAddress,
    oversampling: BMP180Oversampling,
) -> Result<ResponseUncompensatedPressure, String> {
    let response;
    let oversampling_command = match oversampling {
        BMP180Oversampling::UltraLowPower => 0x34,
        BMP180Oversampling::Standard => 0x74,
        BMP180Oversampling::HighResolution => 0xB4,
        BMP180Oversampling::UltraHighResolution => 0xF4,
    };
    {
        let mut driver = driver.lock().await;
        driver
            .write(
                address,
                &[0xF4, oversampling_command],
                Duration::from_secs(2),
            )
            .await?;
    }
    sleep(Duration::from_millis(26)).await;
    {
        let mut driver = driver.lock().await;
        response = driver
            .write_read(address, &[0xF6], 3, Duration::from_secs(2))
            .await?;
    }
    let UP = (((response[0] as u32) << 16) + ((response[1] as u32) << 8) + response[2] as u32)
        >> (8 - oversampling as u8);
    let response = ResponseUncompensatedPressure { UP };
    trace!("BMP180 pressure: {response:?}");
    Ok(response)
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct ResponseCalibrationData {
    AC1: i16,
    AC2: i16,
    AC3: i16,
    AC4: u16,
    AC5: u16,
    AC6: u16,
    B1: i16,
    B2: i16,
    MB: i16,
    MC: i16,
    MD: i16,
}

#[allow(non_snake_case)]
struct CalibrationData {
    AC1: f64,
    AC2: f64,
    AC3: f64,
    AC4: f64,
    AC5: f64,
    AC6: f64,
    B1: f64,
    B2: f64,
    #[allow(dead_code)]
    MB: f64,
    MC: f64,
    MD: f64,
}

impl From<ResponseCalibrationData> for CalibrationData {
    fn from(value: ResponseCalibrationData) -> Self {
        CalibrationData {
            AC1: value.AC1 as f64,
            AC2: value.AC2 as f64,
            AC3: value.AC3 as f64,
            AC4: value.AC4 as f64,
            AC5: value.AC5 as f64,
            AC6: value.AC6 as f64,
            B1: value.B1 as f64,
            B2: value.B2 as f64,
            MB: value.MB as f64,
            MC: value.MC as f64,
            MD: value.MD as f64,
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct ResponseUncompensatedTemperature {
    UT: u16,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct ResponseUncompensatedPressure {
    UP: u32,
}

fn swap_msb_lsb(ve: &mut [u8]) {
    let mut index = 0;
    while index < ve.len() {
        ve.swap(index, index + 1);
        index += 2;
    }
}

#[derive(Debug)]
pub struct BMP180Data {
    pub temperature: PhyQuantity,
    pub pressure: PhyQuantity,
    pub altitude: PhyQuantity,
}

/// Кол-во измерений для формирования значен
#[derive(Clone, Copy, Debug)]
pub enum BMP180Oversampling {
    /// number of samples = 1
    UltraLowPower = 0,
    /// number of samples = 2
    Standard = 1,
    /// number of samples = 4
    HighResolution = 2,
    /// number of samples = 8
    UltraHighResolution = 3,
}
