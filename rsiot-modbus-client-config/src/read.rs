pub type Callback<TMsg> = fn(&ResponseType) -> Vec<TMsg>;

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

pub struct Request<TMsg> {
    pub params: RequestParams,
    pub callback: Callback<TMsg>,
}
