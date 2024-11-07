use leptos::prelude::*;

/// Тип отображения поля ввода
pub enum TextFieldKind {
    /// `md-filled-text-field`
    Filled,
    /// `md-outlined-text-field`
    Outlined,
}

#[component]
pub fn TextField(
    /// Тип отображения поля ввода
    #[prop(default = TextFieldKind::Outlined)]
    kind: TextFieldKind,

    /// Состояние для hmi
    #[prop(into)]
    value: Signal<f64>,

    on_input: impl Fn(&str) + 'static,

    #[prop(default = false)] readonly: bool,
) -> impl IntoView {
    match kind {
        TextFieldKind::Filled => {
            view! { <md-filled-text-field readOnly=readonly></md-filled-text-field> }.into_any()
        }
        TextFieldKind::Outlined => view! {
            <md-outlined-text-field
                readOnly=readonly
                value=move || value.get()
                on:click=move |ev| {
                    let value = event_target_value(&ev);
                    on_input(&value)
                }
            >
            </md-outlined-text-field>
        }
        .into_any(),
    }
}
