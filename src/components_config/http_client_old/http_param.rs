/// Параметры HTTP
#[derive(Clone, Debug)]
pub enum HttpParam {
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
        body: String,
    },

    /// POST method
    Post {
        /// Endpoint for connection
        endpoint: String,
        /// Body of request
        body: String,
    },
}
