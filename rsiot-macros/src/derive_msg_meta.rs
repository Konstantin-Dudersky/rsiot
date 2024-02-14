use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput};

pub fn derive_msg_meta(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = &ast.data;
    let data = extract_enum_data(data);

    let mut enum_variants_ts = TokenStream2::new();
    let mut enum_variants_cmp_source = TokenStream2::new();
    let mut enum_variants_cmp_process = TokenStream2::new();
    let mut enum_variants_cmp_set = TokenStream2::new();
    let mut enum_variants_rt_msg = TokenStream2::new();
    for variant in &data.variants {
        let variant_name = &variant.ident;
        enum_variants_ts.extend(quote! {
            #name::#variant_name(msg_content) => msg_content.ts.clone(),
        });
        enum_variants_cmp_source.extend(quote! {
            #name::#variant_name(msg_content) => msg_content.cmp_source.clone(),
        });
        enum_variants_cmp_process.extend(quote! {
            #name::#variant_name(msg_content) => msg_content.cmp_process.clone(),
        });
        enum_variants_cmp_set.extend(quote! {
            #name::#variant_name(msg_content) => msg_content.cmp_set(cmp_id),
        });
        enum_variants_rt_msg.extend(quote! {
            #name::#variant_name(msg_content) => msg_content.value.fmt_value(template),
        });
    }

    let expanded = quote! {
        impl MsgMeta for #name {
            fn ts(&self) -> msg_meta::Timestamp {
                match self {
                    #enum_variants_ts
                }
            }

            fn cmp_source(&self) -> Option<msg_meta::ComponentId> {
                match self {
                    #enum_variants_cmp_source
                }
            }

            fn cmp_process(&self) -> Option<msg_meta::ComponentId> {
                match self {
                    #enum_variants_cmp_process
                }
            }

            fn cmp_set(&mut self, cmp_id: &msg_meta::ComponentId) {
                match self {
                    #enum_variants_cmp_set
                }
            }

            fn fmt_value(&self, template: &str) -> String {
                match self {
                    #enum_variants_rt_msg
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
