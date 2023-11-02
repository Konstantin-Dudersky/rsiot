use messages_lib::IMessage;

pub enum RequestParams {
    /// (start address, count)
    ReadHoldingRegisters(u16, u16),
    /// (start address, count)
    ReadCoils(u16, u16),
}

pub enum ResponseType {
    U16(Vec<u16>),
    Bool(Vec<bool>),
}

pub struct ReadRequest<TResponseFunc>
where
    TResponseFunc: Fn(&ResponseType) -> Vec<Box<dyn IMessage>>,
{
    pub request_params: RequestParams,
    pub response_func: TResponseFunc,
}

pub trait Test: Fn(&ResponseType) -> Vec<Box<dyn IMessage>> {}

pub struct ReadRequest1<T>
where
    T: Test,
{
    pub request_params: RequestParams,
    pub response_func: T,
}
