use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput};

pub fn derive_into_eav(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let data = extract_enum_data(data);

    let mut enum_variants = TokenStream2::new();
    for variant in &data.variants {
        let variant_name = &variant.ident;
        enum_variants.extend(quote! {
            #name::#variant_name(value) => value.into_eav(),
        })
    }

    let expanded = quote! {
        impl IntoEav for #name {
            fn into_eav(&self) -> Vec<Eav> {
                match self {
                    #enum_variants
                }
            }
        }
    };
    TokenStream::from(expanded)
}

fn extract_enum_data(data: &Data) -> &DataEnum {
    match data {
        Data::Enum(data_enum) => data_enum,
        _ => abort_call_site!("IntoEav macro only for enum"),
    }
}
