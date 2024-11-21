//! Компонент для интеграции видеопотока с сервиса go2rtc

use leptos::prelude::*;

/// Компонент для интеграции видеопотока с сервиса go2rtc
#[component]
pub fn Go2rtc<'a>(
    /// Адрес хоста, на котором развернут сервис go2rtc; по-умолчанию `localhost`
    #[prop(default = String::from("localhost"))]
    hostname: String,

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
    // let url = format!("http://{hostname}:{port}/webrtc.html?src={camera}");
    let url = format!("http://{hostname}:{port}/stream.html?src={camera}");

    view! { <iframe src=url width=width height=height allow="fullscreen;"></iframe> }
    // TODO: добавить scrolling="no"
}
