pub trait RsiotI2cDriverBase {
    async fn write_read(&mut self, address: u8);
}
