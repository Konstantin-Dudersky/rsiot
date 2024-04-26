use esp_idf_svc::hal::i2c::I2cDriver;

use crate::drivers_i2c::RsiotI2cDriverBase;

pub struct RsiotI2cDriver {
    i2c: I2cDriver<'static>,
}

impl RsiotI2cDriver {
    pub fn new(i2c: I2cDriver<'static>) -> Self {
        Self { i2c }
    }
}

impl RsiotI2cDriverBase for RsiotI2cDriver {
    async fn write_read(&mut self, address: u8) {
        let send_bytes = vec![0xD0];
        let size = 1;
        let mut answer = vec![0; size];
        self.i2c.write_read(address, &send_bytes, &mut answer, 1000);
    }
}
