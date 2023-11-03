use std::net::IpAddr;

use super::{read, write};

pub enum ClientConfig<TMsg> {
    Tcp(TcpClientConfig<TMsg>),
    Rtu,
}

pub struct TcpClientConfig<TMsg> {
    pub host: IpAddr,
    pub port: u16,
    pub read_config: Vec<read::Request<TMsg>>,
    pub write_config: write::Request<TMsg>,
}
