mod derive_msg_meta;
mod create_signal_from_msg;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, parse_str, DeriveInput};

#[proc_macro_derive(MsgMeta)]
#[proc_macro_error]
pub fn derive_into_eav(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    derive_msg_meta::derive_msg_meta(&input)
}

/// Макрос для создания сигналов фреймворка leptos из сообщений.
///
/// Принимает на вход строку, содержащую вариант сообщения, без вложенного значения.
///
/// # Пример
///
/// ```rust
/// let (signal, signal_set) = create_signal_from_msg!(ExampleMessage::ValueInstantF64);
/// ```
#[proc_macro]
pub fn create_signal_from_msg(msg: TokenStream) -> TokenStream {
    let parts = &msg
        .to_string()
        .split("-")
        .map(String::from)
        .collect::<Vec<String>>();
    // default: &msg(Default::default()),

    let default = format!(
        "Message::new_full({}(Default::default())){}",
        parts.join("("),
        ")".repeat(parts.len() - 1)
    );
    let code = r#"
    create_signal_from_msg::create(create_signal_from_msg::Config {
        default: &default,
        fn_input: |msg| match msg {
            &msg(content) => Some(content.clone()),
            _ => None,
        },
        fn_output: |value| Some(&msg(MsgContent::new(value))),
    })
"#;
    let code = code.replace("&msg", &msg.to_string());
    let code = code.replace("&default", &default);
    let code = code.replace('\"', "");
    let code = parse_str::<syn::Expr>(&code).unwrap();

    TokenStream::from(quote! {
        #code
    })
}

