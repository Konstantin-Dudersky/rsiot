use super::RsiotI2cDriverBase;

pub struct BMP180 {
    address: u8,
}

impl BMP180 {
    pub fn new(address: u8) -> Self {
        Self { address }
    }

    pub async fn process(&self, driver: impl RsiotI2cDriverBase) {}
}
