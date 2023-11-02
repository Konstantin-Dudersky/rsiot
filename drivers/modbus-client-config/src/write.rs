// pub type RequestCallback = fn(&Box<dyn IMessage>) -> RequestParams;
pub type RequestCallback<T> = fn(&T) -> RequestParams;

pub enum RequestParams {
    NoRequest,
    WriteSingleRegister(u16, u16),
}

pub struct Request<T> {
    pub params: RequestCallback<T>,
}
