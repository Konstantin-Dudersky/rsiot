/// Тип HTTP-запросов
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum RequestKind {
    Get,
    Post,
    Put,
    Delete,
}
