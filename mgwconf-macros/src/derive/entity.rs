use proc_macro2::{Ident, TokenStream};

use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Data, Fields, Meta};

#[derive(Default, Debug)]
struct DeleteQueryAttr {
    ident: Option<Ident>,
    alias: Option<TokenStream>,
}

pub fn derive_entity(_ident: Ident, data: Data) -> Result<TokenStream, syn::Error> {
    let mut primary_field: Punctuated<_, Comma> = Punctuated::new();
    let mut delete_query_attrs = Vec::new();
    if let Data::Struct(ref item_struct) = data {
        if let Fields::Named(ref fields) = item_struct.fields {
            for field in &fields.named {
                if let Some(_ident) = &field.ident {
                    for attr in field.attrs.iter() {
                        if let Some(ident) = attr.path().get_ident() {
                            if ident != "mgw_conf" {
                                continue;
                            }
                        } else {
                            continue;
                        }

                        // single param
                        let mut delete_query_attr = DeleteQueryAttr::default();
                        if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
                            for meta in list.iter() {
                                match meta {
                                    Meta::Path(p) => {
                                        if let Some(name) = p.get_ident() {
                                            if name == "primary_name" {
                                                primary_field.push(field.ident.clone());
                                            } else if name == "delete_query_attr" {
                                                if delete_query_attr.ident.is_none() {
                                                    delete_query_attr.ident = field.ident.clone();
                                                }
                                            }
                                        }
                                    }
                                    Meta::List(l) => {
                                        if let Some(name) = l.path.get_ident() {
                                            if name == "delete_query_attr" {
                                                if delete_query_attr.alias.is_none() {
                                                    delete_query_attr.alias = Some(l.tokens.clone());
                                                    delete_query_attr.ident = field.ident.clone();
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            delete_query_attrs.push(delete_query_attr);
                        }
                    }
                }
            }
        }
    }
    // if let Data::Struct(item_struct) = &data {
    //     if let Fields::Named(ref fields) = item_struct.fields {
    //         for field in &fields.named {
    //             if let Some(ident) = &field.ident {
    //                 const RAW_IDENTIFIER: &str = "r#";
    //                 let original_field_name = ident.to_string().trim_start_matches(RAW_IDENTIFIER).to_string();
    //                 let field_name = Ident::new(&original_field_name, Span::call_site());
    //                 let mut delete_attr = false;
    //                 for attr in field.attrs.iter() {
    //                     if let Ok(list) = attr.parse_args_with(Punctuated::<Meta, Comma>::parse_terminated) {
    //                         for meta in list.iter() {
    //                             match meta {
    //                                 Meta::Path(p) => {
    //                                     if let Some(i) = p.get_ident() {
    //                                         if i == "delete_attr" {
    //                                             delete_attr = true;
    //                                         }
    //                                     }
    //                                 }
    //                                 _ => {}
    //                             }
    //                         }
    //                     }
    //                     if delete_attr {
    //                         let mut match_row = quote! { #field_name };
    //                         match_row = quote! { #match_row.delete_attr() };
    //                         columns_trait.push(match_row);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

    let primary_name = { primary_field.first() };
    let mut delete_query_attributes_names = Vec::new();
    let mut delete_query_attributes = Vec::new();

    for i in delete_query_attrs {
        let ident = i.ident.unwrap();
        delete_query_attributes.push(ident.clone());
        match i.alias {
            Some(alias) => {
                delete_query_attributes_names.push(alias.to_string()[1..alias.to_string().len() - 1].to_string());
            }
            None => {
                delete_query_attributes_names.push(ident.to_string());
            }
        }
    }

    Ok(TokenStream::from_iter([
        quote!(
            #[automatically_derived]
            impl super::CollectionEntityTrait for self::Entities {
                fn as_any(&self) -> &dyn super::prelude::Any {
                    self
                }
                fn get(&self, index: usize) -> Option<Box<dyn InnerEntityTrait>> {
                    match self.0.get(index) {
                        Some(e) => Some(Box::new(e.clone())),
                        None => None,
                    }
                }
            }
        ),
        quote!(
            #[automatically_derived]
            impl super::InnerEntityTrait for Entity {
                fn name(&self) -> &str {
                    &self.#primary_name
                }
                fn delete_query(&self) -> Vec<(String, String)> {
                    Vec::from([#((String::from(#delete_query_attributes_names), self.#delete_query_attributes.to_string())),*])
                }
                fn as_any(&self) -> &dyn Any {
                    self
                }
            }
        ),
    ]))
}
