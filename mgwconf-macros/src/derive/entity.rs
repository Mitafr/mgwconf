use proc_macro2::{Ident, Span, TokenStream};

use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, Fields, Meta};

pub fn derive_entity(_ident: Ident, data: Data) -> Result<TokenStream, syn::Error> {
    let mut columns_trait: Punctuated<_, Comma> = Punctuated::new();
    let mut primary_field: Punctuated<_, Comma> = Punctuated::new();
    if let Data::Struct(ref item_struct) = data {
        if let Fields::Named(ref fields) = item_struct.fields {
            for field in &fields.named {
                if let Some(_ident) = &field.ident {
                    for attr in field.attrs.iter() {
                        if let Some(ident) = attr.path.get_ident() {
                            if ident != "mgw_conf" {
                                continue;
                            }
                        } else {
                            continue;
                        }

                        // single param
                        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                            for meta in list.iter() {
                                match meta {
                                    Meta::NameValue(nv) => {
                                        println!("{:?}", nv);
                                    }
                                    Meta::Path(p) => {
                                        if let Some(name) = p.get_ident() {
                                            if name == "primary_name" {
                                                primary_field.push(field.ident.clone());
                                            }
                                        }
                                    }
                                    Meta::List(nv) => println!("{:?}", nv),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if let Data::Struct(item_struct) = &data {
        if let Fields::Named(ref fields) = item_struct.fields {
            for field in &fields.named {
                if let Some(ident) = &field.ident {
                    const RAW_IDENTIFIER: &str = "r#";
                    let original_field_name = ident.to_string().trim_start_matches(RAW_IDENTIFIER).to_string();
                    let field_name = Ident::new(&original_field_name, Span::call_site());
                    let mut delete_attr = false;
                    for attr in field.attrs.iter() {
                        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                            for meta in list.iter() {
                                match meta {
                                    Meta::Path(p) => {
                                        if let Some(i) = p.get_ident() {
                                            if i == "delete_attr" {
                                                delete_attr = true;
                                            }
                                        }
                                    }
                                    Meta::List(_l) => {}
                                    Meta::NameValue(_nv) => {}
                                }
                            }
                        }
                        let mut match_row = quote! { Self::#field_name };
                        if delete_attr {
                            match_row = quote! { #match_row.delete_attr() };
                        }
                        columns_trait.push(match_row);
                    }
                }
            }
        }
    }

    let primary_name = { primary_field.first() };

    Ok(TokenStream::from_iter([
        quote!(
            #[automatically_derived]
            impl super::prelude::CollectionEntityTrait for self::Entities {
                fn as_any(&self) -> &dyn super::prelude::Any {
                    self
                }
            }
        ),
        quote!(
            #[automatically_derived]
            impl InnerEntityTrait for Entity {
                fn get_name(&self) -> &str {
                    &self.#primary_name
                }
            }
        ),
    ]))
}
