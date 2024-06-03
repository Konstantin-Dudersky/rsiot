use leptos::*;

/// [Документация](https://material-web.dev/components/dialog/#interactive-demo)
///
/// Добавить в `index.js`:
///
/// ```js
/// import "@material/web/dialog/dialog";
/// ```
#[component]
pub fn Dialog(open: RwSignal<bool>) -> impl IntoView {
    let (local_open, local_open_set) = create_signal(false);

    create_effect(move |_| {
        if open.get() {
            open.set(false);
            local_open_set.set(true);
        }
    });

    let script = include_str!("./dialog.js");

    view! {
        // <md-dialog open=move || open.get()>
        //     <div slot="headline">
        //         Dialog title
        //     </div>
        //     <form slot="content" id="form-id" method="dialog">
        //         A simple dialog with free-form content.
        //     </form>
        //     <div slot="actions">
        //         <md-text-button form="form-id">Ok</md-text-button>
        //     </div>
        // </md-dialog>


        <md-dialog open=move || local_open.get()  id="dialog">
            <div slot="headline">headline</div>
            <form id="form" slot="content" method="dialog">
                <span>supportingText</span>
            </form>
            <div slot="actions">
                <md-filled-button form="form" value="close">Close</md-filled-button>
                <md-filled-button form="form" value="ok" autofocus>OK</md-filled-button>
            </div>
        </md-dialog>
        <script inner_html=script></script>
    }
}
