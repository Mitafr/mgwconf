extern crate proc_macro;

use crate::derive::model::derive_model;
use derive::entity::derive_entity;
use proc_macro2::TokenStream;
use std::iter::FromIterator;
use syn::{parse_macro_input, DeriveInput};

mod attributes;
mod derive;

#[proc_macro_derive(DeriveModel, attributes(mgw_conf))]
pub fn derive_all(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from_iter([derive_entity(), derive_model(input)]).into()
}
