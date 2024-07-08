use std::sync::Arc;
use std::time::Duration;

use tokio::{sync::Mutex, time::sleep};

use super::super::{I2cSlaveAddress, RsiotI2cDriverBase};

/// Опрос модуля PCA9555
pub struct PCA9555 {
    /// Адрес микросхемы
    pub address: I2cSlaveAddress,
}

impl PCA9555 {
    /// Запустить опрос устройства
    pub async fn spawn(&self, driver: Arc<Mutex<impl RsiotI2cDriverBase + 'static>>) {
        loop {
            let mut driver = driver.lock().await;

            // read ------------------
            let request = [0];
            let result = driver.write_read(self.address, &request, 2).await;
            println!("Result read: {:?}", result);

            // write -----------------

            // let request = [6, 0];
            // let result = driver.write(self.address, &request).await;
            // println!("Result 1: {:?}", result);

            // let request = [2, value];
            // value = if value == 0b11111111 { 0 } else { 0b11111111 };
            // let result = driver.write(self.address, &request).await;
            // println!("Result 2: {:?}", result);

            // let request = [2];
            // let result = driver.write_read(self.address, &request, 2).await;
            // println!("Result 3: {:?}", result);

            // let request = [6];
            // let result = driver.write_read(self.address, &request, 2).await;
            // println!("Result 4: {:?}", result);

            sleep(Duration::from_secs(2)).await
        }
    }
}
