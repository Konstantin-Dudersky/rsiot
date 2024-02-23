mod create_signal_from_msg;
mod derive_msg_meta;

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
/// let (signal, signal_set) = create_signal_from_msg!("Custom-ValueInstantF64");
/// ```
#[proc_macro]
pub fn create_signal_from_msg(msg: TokenStream) -> TokenStream {
    let code = create_signal_from_msg::create_signal_from_msg(&msg.to_string());
    let code = parse_str::<syn::Expr>(&code).unwrap();
    TokenStream::from(quote! {
        #code
    })
}

/// Макрос для упрощения создания сообщения
///
/// Принимает на вход строку вида `Variant1-Variant2-Variant3::value`
#[proc_macro]
pub fn message_new(msg: TokenStream) -> TokenStream {
    let code = create_signal_from_msg::message_new(&msg.to_string());
    let code = parse_str::<syn::Expr>(&code).unwrap();
    TokenStream::from(quote! {
        #code
    })
}
