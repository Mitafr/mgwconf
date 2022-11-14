use proc_macro2::TokenStream;

use quote::quote;

pub fn derive_entity() -> TokenStream {
    TokenStream::from_iter([
        quote!(
            #[automatically_derived]
            impl super::EntityTrait for self::Entities {}
        ),
        quote!(
            #[automatically_derived]
            impl InnerEntityTrait for Entity {}
        ),
    ])
}
