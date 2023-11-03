// pub type RequestCallback = fn(&Box<dyn IMessage>) -> RequestParams;
pub type Callback<TMsg> = fn(&TMsg) -> RequestParams;

#[derive(Clone, Debug)]
pub enum RequestParams {
    NoRequest,
    /// (address, value)
    WriteSingleRegister(u16, u16),
}

#[derive(Clone, Debug)]
pub struct Request<TMsg> {
    pub params: Callback<TMsg>,
}
