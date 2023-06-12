pub mod derive_attr {
    use bae2::FromAttributes;

    #[derive(FromAttributes, Debug)]
    pub struct MgwConf {
        pub route: syn::Lit,
    }
}
