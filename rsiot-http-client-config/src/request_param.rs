#[derive(Debug)]
pub enum RequestParamKind {
    Get,
    Put,
    Post,
}

#[derive(Debug)]
pub struct RequestParam {
    pub endpoint: String,
    pub kind: RequestParamKind,
}
