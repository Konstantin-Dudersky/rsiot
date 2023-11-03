use std::net::IpAddr;

use super::{read, write};

#[derive(Clone, Debug)]
pub enum ClientConfig<TMsg> {
    Tcp(TcpClientConfig<TMsg>),
    Rtu,
}

#[derive(Clone, Debug)]
pub struct TcpClientConfig<TMsg> {
    pub host: IpAddr,
    pub port: u16,
    pub read_config: Vec<read::Request<TMsg>>,
    pub write_config: write::Request<TMsg>,
}
