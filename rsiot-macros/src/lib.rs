mod derive_msg_meta;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(MsgMeta)]
#[proc_macro_error]
pub fn derive_into_eav(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    derive_msg_meta::derive_msg_meta(&input)
}
