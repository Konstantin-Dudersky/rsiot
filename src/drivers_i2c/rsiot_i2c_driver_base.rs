pub trait RsiotI2cDriverBase {
    fn write_read(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + std::marker::Send;

    async fn write(&mut self, address: u8, request: &[u8]) -> Result<(), String>;

    fn read(
        &mut self,
        address: u8,
        response_size: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + std::marker::Send;
}
