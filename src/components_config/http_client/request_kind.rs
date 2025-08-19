// ANCHOR: RequestKind
/// Тип HTTP-запросов
#[derive(Clone, Copy, Debug)]
pub enum RequestKind {
    /// GET-запрос
    Get,
    /// POST-запрос
    Post,
    /// PUT-запрос
    Put,
    /// DELETE-запрос
    Delete,
}
// ANCHOR: RequestKind
