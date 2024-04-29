pub trait RsiotI2cDriverBase {
    async fn write_read(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> Result<Vec<u8>, String>;

    async fn write(&mut self, address: u8, request: &[u8]) -> Result<(), String>;
}
