// pub type RequestCallback = fn(&Box<dyn IMessage>) -> RequestParams;
pub type Callback<TMsg> = fn(&TMsg) -> RequestParams;

pub enum RequestParams {
    NoRequest,
    WriteSingleRegister(u16, u16),
}

pub struct Request<TMsg> {
    pub params: Callback<TMsg>,
}
