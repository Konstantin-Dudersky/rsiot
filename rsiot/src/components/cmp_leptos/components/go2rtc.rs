//! Компонент для интеграции видеопотока с сервиса go2rtc

use leptos::*;

/// Компонент для интеграции видеопотока с сервиса go2rtc
#[component]
pub fn Go2rtc<'a>(
    /// Адрес хоста, на котором развернут сервис go2rtc; по-умолчанию `localhost`
    #[prop(default = "localhost")]
    hostname: &'a str,

    /// Порт сервиса, по-умолчанию 1984
    #[prop(default = 1984)]
    port: u16,

    /// Название камеры из конфигурации
    camera: &'a str,

    /// Ширина iframe
    #[prop(default = "100%")]
    width: &'static str,

    /// Высота iframe
    #[prop(default = "600px")]
    height: &'static str,
) -> impl IntoView {
    let url = format!("http://{hostname}:{port}/webrtc.html?src={camera}");

    view! { <iframe src=url width=width height=height allow="fullscreen;" scrolling="no"></iframe> }
}
