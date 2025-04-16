#[allow(missing_docs)]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SVG element not found: {0}")]
    ElementNotFound(String),
}
