use url::Url;

use messages_lib::IMessage;

use super::read;

pub enum ClientConfig<TRead>
where
    TRead: Fn(&read::ResponseType) -> Vec<Box<dyn IMessage>>,
{
    Tcp(TcpClientConfig<TRead>),
    Rtu,
}

pub struct TcpClientConfig<TRead>
where
    TRead: Fn(&read::ResponseType) -> Vec<Box<dyn IMessage>>,
{
    pub url: Url,
    pub read_config: read::ReadRequest<TRead>,
}

type test = fn() -> ();
