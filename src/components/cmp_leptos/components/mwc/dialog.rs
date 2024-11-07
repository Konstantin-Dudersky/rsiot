use leptos::{html, prelude::*};

/// [Документация](https://material-web.dev/components/dialog/#interactive-demo)
///
/// Добавить в `index.js`:
///
/// ```js
/// import "@material/web/dialog/dialog";
/// ```
#[component]
pub fn Dialog<F1, IV1, F2, IV2, F3, IV3>(
    /// Заголовок диалога
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Headline }
    /// ```
    headline: F1,

    /// Содержимое диалога
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Content }
    /// ```
    content: F2,

    /// Содержимое области действий
    ///
    /// **Примеры**
    ///
    /// ```rust
    /// || view! { Actions }
    /// ```
    actions: F3,

    /// Сигнал `Signal<()>` для открытия окна
    #[prop(into)]
    open: Signal<()>,
) -> impl IntoView
where
    F1: Fn() -> IV1,
    IV1: IntoView,
    F2: Fn() -> IV2,
    IV2: IntoView,
    F3: Fn() -> IV3,
    IV3: IntoView,
{
    let node_ref: NodeRef<html::Custom> = create_node_ref();

    create_effect(move |v| {
        open.get();
        if v.is_some() {
            node_ref
                .get()
                .unwrap()
                .set_attribute("open", "true")
                .unwrap();
        }
    });

    view! {
        <md-dialog node_ref=node_ref id="dialog">
            <div slot="headline">{headline()}</div>
            <div slot="content">{content()}</div>
            <div slot="actions">{actions()}</div>
        </md-dialog>
    }
}
