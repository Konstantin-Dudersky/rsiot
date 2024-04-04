//! Определить URL-адрес, введенный в браузере
//!
//! В `Cargo.toml` прописать:
//!
//! ```toml
//! web-sys = {version = "*", features = ["Location", "Window"]}
//! ```

use tracing::info;
use url::Url;
use web_sys::window;

/// Определить URL-адрес, введенный в браузере
///
/// В `Cargo.toml` прописать:
///
/// ```toml
/// web-sys = {version = "*", features = ["Location", "Window"]}
/// ```
pub fn define_window_url() -> Result<Url, String> {
    let window = window().ok_or("Window is None")?;
    let href = window
        .location()
        .href()
        .map_err(|err| format!("{:?}", err))?;
    let mut url = Url::parse(&href).map_err(|err| err.to_string())?;
    url.set_path("");
    info!("Window location: {:?}", url);
    Ok(url)
}
