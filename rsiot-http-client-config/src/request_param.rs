#[derive(Debug)]
pub enum RequestParam {
    /// (endpoint)
    Get(String),
    Put(String),
    Post(String),
}
