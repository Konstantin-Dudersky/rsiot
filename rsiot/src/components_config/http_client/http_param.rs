/// Параметры HTTP
#[derive(Clone, Debug)]
pub enum HttpParam {
    /// (endpoint)
    Get {
        endpoint: String,
    },
    Put {
        endpoint: String,
        body: String,
    },
    Post(String),
}
