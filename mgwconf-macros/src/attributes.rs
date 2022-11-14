pub mod derive_attr {
    use bae::FromAttributes;

    #[derive(Default, FromAttributes, Debug)]
    pub struct MgwConf {
        pub entity: Option<syn::Ident>,
        pub route: Option<syn::Ident>,
    }
}
