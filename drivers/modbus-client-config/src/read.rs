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

pub struct ReadRequest {
    pub params: RequestParams,
    pub callback: Callback,
}

pub type Callback = fn(&ResponseType) -> Vec<Box<dyn IMessage>>;
