//! Функция `sleep` зависит от платформы

#[cfg(target_arch = "wasm32")]
pub use gloo::timers::future::sleep;
#[cfg(not(target_arch = "wasm32"))]
pub use tokio::time::sleep;
