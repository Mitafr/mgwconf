use proc_macro2::TokenStream;

use quote::quote;

pub fn derive_model(input: syn::DeriveInput) -> TokenStream {
    let ident = input.ident;
    quote!(
        #[async_trait]
        #[automatically_derived]
        impl super::ModelTrait for #ident {
            type Entity = self::Entities;
            type Inner = self::Entity;

            async fn get(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<Self::Entity, anyhow::Error> {
                let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
                let res = client.get(route_url(config, &Self::default().route)).headers(header).send().await?;
                if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
                    return Err(anyhow::Error::msg(format!("{:?}", res)));
                }
                Ok(res.json::<Self::Entity>().await?)
            }

            async fn post(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
                let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
                let mut test = Self::Inner::default();

                debug!("{:?}", test);

                client.post(route_url(config, &Self::default().route)).json(&test).headers(header).send().await?;
                Ok(())
            }

        }
    )
}
