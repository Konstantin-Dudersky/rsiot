#[derive(Clone, Debug)]
pub enum MsgResponse {
    Success { endpoint: String, body: Vec<u8> },
    Error { endpoint: String, error: String },
}

impl MsgResponse {
    pub fn get_endpoint(&self) -> &String {
        match self {
            MsgResponse::Success { endpoint, .. } => endpoint,
            MsgResponse::Error { endpoint, .. } => endpoint,
        }
    }
}
