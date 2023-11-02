use url::Url;

use super::read;

pub enum ClientConfig {
    Tcp(TcpClientConfig),
    Rtu,
}

pub struct TcpClientConfig {
    pub url: Url,
    pub read_config: Vec<read::ReadRequest>,
}

type test = fn() -> ();
