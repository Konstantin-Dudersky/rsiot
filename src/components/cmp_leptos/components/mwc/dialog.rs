use leptos::*;

/// [Документация](https://material-web.dev/components/dialog/#interactive-demo)
///
/// Добавить в `index.js`:
///
/// ```js
/// import "@material/web/dialog/dialog";
/// ```
#[component]
pub fn Dialog(#[prop(into)] open: Signal<()>) -> impl IntoView {
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
        <md-dialog node_ref=node_ref id="dialog" >
            <div slot="headline">headline</div>
            <form id="form" slot="content" method="dialog">
                <span>supportingText</span>
            </form>
            <div slot="actions">
                <md-filled-button form="form" value="close">Close</md-filled-button>
                <md-filled-button form="form" value="ok" autofocus>OK</md-filled-button>
            </div>
        </md-dialog>
    }
}
