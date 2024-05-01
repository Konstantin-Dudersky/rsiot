pub trait RsiotI2cDriverBase
where
    Self: Send,
{
    fn write_read(
        &mut self,
        address: u8,
        request: &[u8],
        response_size: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + std::marker::Send;

    fn write(
        &mut self,
        address: u8,
        request: &[u8],
    ) -> impl std::future::Future<Output = Result<(), String>> + std::marker::Send;

    fn read(
        &mut self,
        address: u8,
        response_size: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, String>> + std::marker::Send;
}
