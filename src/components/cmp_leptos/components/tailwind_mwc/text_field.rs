use leptos::*;

pub enum TextFieldKind {
    Outline,
}

#[component]
pub fn TextField(#[prop(default=TextFieldKind::Outline)] kind: TextFieldKind) -> impl IntoView {
    view! {
        <div class="relative h-14">
            <label for="name" class="absolute -top-2 left-2 inline-block bg-white px-1 text-xs font-medium text-gray-900">Name</label>
            <input type="text" name="name" id="name"
                class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6" placeholder="Jane Smith" />
        </div>
    }
}
