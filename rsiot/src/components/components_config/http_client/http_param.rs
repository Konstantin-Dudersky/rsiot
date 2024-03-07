/// Параметры HTTP
#[derive(Clone, Debug)]
pub enum HttpParam {
    /// (endpoint)
    Get(String),
    Put(String),
    Post(String),
}
