use url::Url;

use super::{read, write};

pub enum ClientConfig<T> {
    Tcp(TcpClientConfig<T>),
    Rtu,
}

pub struct TcpClientConfig<T> {
    pub url: Url,
    pub read_config: Vec<read::Request<T>>,
    pub write_config: write::Request<T>,
}
