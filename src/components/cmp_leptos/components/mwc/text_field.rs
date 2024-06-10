use leptos::*;

use crate::components::cmp_plc::plc::library::drives::valve_analog::QHmiStatus;

/// Тип отображения поля ввода
pub enum TextFieldKind {
    Filled,
    Outlined,
}

#[component]
pub fn TextField(
    /// Тип отображения поля ввода
    #[prop(default = TextFieldKind::Outlined)]
    kind: TextFieldKind,

    hmi_status: ReadSignal<QHmiStatus>,
) -> impl IntoView {
    match kind {
        TextFieldKind::Filled => view! { <md-filled-text-field></md-filled-text-field> },
        TextFieldKind::Outlined => view! { <md-outlined-text-field></md-outlined-text-field> },
    }
}
