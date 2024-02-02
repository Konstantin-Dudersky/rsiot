/// Маршрут для получения сообщений
pub async fn root() -> String {
    let text = include_str!("../../doc/api_description.md");
    text.to_string()
}
