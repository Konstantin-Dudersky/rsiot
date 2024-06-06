//! Коммуникация с LED-экраном

use std::{sync::Arc, time::Duration};

use tokio::{sync::Mutex, time::sleep};

use super::RsiotI2cDriverBase;

pub struct SSD1306 {}

impl SSD1306 {
    pub async fn fn_process(self, driver: Arc<Mutex<impl RsiotI2cDriverBase + Send + 'static>>) {
        loop {
            let mut driver = driver.lock().await;

            let res = driver
                .read(
                    super::I2cSlaveAddress::Direct {
                        slave_address: 0x77,
                    },
                    0,
                )
                .await;

            println!("{:?}", res);

            sleep(Duration::from_secs(2)).await;
        }
    }
}
