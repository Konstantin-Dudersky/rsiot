use super::define_window_url;

/// Определить имя хоста
pub fn define_hostname() -> Result<String, String> {
    let window_url = define_window_url()?;
    window_url
        .host_str()
        .map(String::from)
        .ok_or(format!("Cannot defin hostname from url: {}", window_url))
}
