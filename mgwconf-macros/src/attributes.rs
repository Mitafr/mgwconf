pub mod derive_attr {
    use bae::FromAttributes;

    #[derive(FromAttributes, Debug)]
    pub struct MgwConf {
        pub route: syn::Lit,
    }
}
