extern crate proc_macro;

use crate::derive::model::derive_model;
use proc_macro2::TokenStream;
use std::iter::FromIterator;
use syn::{parse_macro_input, DeriveInput, Error};

mod attributes;
mod derive;

#[proc_macro_derive(DeriveModel, attributes(mgw_conf))]
pub fn derive_all(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let t = input.clone();
    let DeriveInput { data, attrs, .. } = parse_macro_input!(input as DeriveInput);
    let input = parse_macro_input!(t as DeriveInput);
    TokenStream::from_iter([derive::entity::derive_entity(data.clone()).unwrap(), derive_model(input, data, &attrs).unwrap_or_else(Error::into_compile_error)]).into()
}
