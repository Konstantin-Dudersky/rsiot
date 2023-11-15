pub enum RequestKind {
    Get,
    Put,
    Post,
}

pub struct Request {
    pub endpoint: String,
    pub kind: RequestKind,
}
