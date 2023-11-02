use url::Url;

use super::{read, write};

pub enum ClientConfig<TMsg> {
    Tcp(TcpClientConfig<TMsg>),
    Rtu,
}

pub struct TcpClientConfig<TMsg> {
    pub url: Url,
    pub read_config: Vec<read::Request<TMsg>>,
    pub write_config: write::Request<TMsg>,
}
