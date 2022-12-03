extern crate proc_macro;

use crate::derive::entity::derive_entity;
use crate::derive::model::derive_model;
use proc_macro2::TokenStream;
use std::iter::FromIterator;
use syn::{parse_macro_input, DeriveInput, Error};

mod attributes;
mod derive;

#[proc_macro_derive(DeriveModel, attributes(mgw_conf))]
pub fn proc_derive_model(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = input.clone();
    let DeriveInput { data, attrs, .. } = parse_macro_input!(input as DeriveInput);
    let input = parse_macro_input!(t as DeriveInput);
    TokenStream::from_iter([derive_model(input, data, &attrs).unwrap_or_else(Error::into_compile_error)]).into()
}

#[proc_macro_derive(DeriveEntity, attributes(mgw_conf))]
pub fn proc_derive_entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = input.clone();
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    let _input = parse_macro_input!(t as DeriveInput);
    TokenStream::from_iter([derive_entity(ident, data.clone()).unwrap()]).into()
}
