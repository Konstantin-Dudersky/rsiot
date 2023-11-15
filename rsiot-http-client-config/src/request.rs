pub enum RequestKind {
    Get,
    Put,
    Post,
}

pub struct Request {
    endpoint: String,
    kind: RequestKind,
}
