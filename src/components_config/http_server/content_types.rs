/// Тип содержимого HTTP-ответа
pub enum ContentType {
    /// `application/json` content-type
    ApplicationJson,
    /// `text/plain; charset=utf-8` content-type
    TextPlain,
    /// `text/html` content-type
    TextHtml,
}
