use super::RequestKind;

/// Параметры HTTP
#[derive(Clone, Debug)]
pub enum MsgRequest {
    /// GET method
    Get {
        /// Endpoint for connections
        endpoint: String,
    },

    /// PUT method
    Put {
        /// Endpoint for connection
        endpoint: String,
        /// Body of request
        body: Vec<u8>,
    },

    /// POST method
    Post {
        /// Endpoint for connection
        endpoint: String,
        /// Body of request
        body: Vec<u8>,
    },
}

impl MsgRequest {
    pub fn new(request_kind: RequestKind, endpoint: String, body: Vec<u8>) -> MsgRequest {
        match request_kind {
            RequestKind::Get => Self::Get {
                endpoint: endpoint.to_string(),
            },
            RequestKind::Post => Self::Post {
                endpoint: endpoint.to_string(),
                body,
            },
            RequestKind::Put => Self::Put {
                endpoint: endpoint.to_string(),
                body,
            },
            RequestKind::Delete => todo!(),
        }
    }

    pub fn get_endpoint(&self) -> &str {
        match self {
            MsgRequest::Get { endpoint } => endpoint,
            MsgRequest::Put { endpoint, .. } => endpoint,
            MsgRequest::Post { endpoint, .. } => endpoint,
        }
    }
}
