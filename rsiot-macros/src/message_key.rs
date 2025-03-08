use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use proc_macro_error::abort_call_site;
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields};

pub fn derive_msg_key(ast: &DeriveInput) -> TokenStream {
    let enum_ident = &ast.ident;
    let data = &ast.data;
    let data = extract_enum_data(data);

    // Пустое перечисление
    //
    // Хотя cargo-expand показывает, что код корректный, все равно показывается ошибка
    // rust-analyzer: missing match arm: type ENUM_IDENT is non-empty
    if data.variants.is_empty() {
        let expanded = quote! {
            impl MsgKey for #enum_ident {
                fn key(&self) -> String {
                    "".to_string()
                }
            }
        };
        return TokenStream::from(expanded);
    }

    let mut enum_variants = TokenStream2::new();

    for variant in &data.variants {
        let variant_ident = &variant.ident;
        let variant_name_str = variant_ident.to_string();

        match &variant.fields {
            Fields::Named(_) => enum_variants.extend(quote! {
                #enum_ident::#variant_ident{..} => {
                    #variant_name_str
                },
            }),
            Fields::Unnamed(test) => {
                // Можно напечатать для проверки вывода
                let _full_unnamed = format!("{:?}", test.unnamed);

                let ty = &test.unnamed[0].ty;
                match ty {
                    // Если вложенный идентификатор соответствует идентификатору варианта
                    syn::Type::Path(type_path)
                        if type_path.path.segments[0].ident == *variant_ident =>
                    {
                        enum_variants.extend(quote! {
                            #enum_ident::#variant_ident(msg_content) => {
                                let key_variant = (*msg_content).key();
                                if key_variant.is_empty() {
                                    #variant_name_str
                                } else {
                                    &format!("{}-{}", #variant_name_str, key_variant)
                                }
                            },
                        });
                    }
                    _ => enum_variants.extend(quote! {
                        #enum_ident::#variant_ident(..) => {
                            #variant_name_str
                        },
                    }),
                }
            }
            Fields::Unit => enum_variants.extend(quote! {
                #enum_ident::#variant_ident => {
                    #variant_name_str
                },
            }),
        }
    }

    let expanded = quote! {
        impl MsgKey for #enum_ident {
            fn key(&self) -> String {
                let key = match self {
                    #enum_variants
                };
                key.to_string()
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_enum_data(data: &Data) -> &DataEnum {
    match data {
        Data::Enum(data_enum) => data_enum,
        _ => abort_call_site!("MagKey macro only for enum"),
    }
}
