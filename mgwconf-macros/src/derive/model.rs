use proc_macro2::TokenStream;

use quote::quote;

use syn::{Attribute, Data, Lit};

use crate::attributes::derive_attr;

pub fn derive_model(input: syn::DeriveInput, _data: Data, attrs: &[Attribute]) -> Result<TokenStream, syn::Error> {
    let ident = input.ident;
    let mgw_attr = derive_attr::MgwConf::try_from_attributes(attrs)?.unwrap();
    let route = {
        if let Lit::Str(litstr) = &mgw_attr.route {
            litstr.value()
        } else {
            String::new()
        }
    };
    Ok(quote!(
        #[async_trait]
        #[automatically_derived]
        impl super::ModelTrait for #ident {
            type Entity = self::Entities;
            type Inner = self::Entity;

            async fn get(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<Self::Entity, anyhow::Error> {
                let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
                let res = client.get(route_url(config, #route)).headers(header).send().await?;
                if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
                    return Err(anyhow::Error::msg(format!("{:?}", res)));
                }
                let res = res.json::<Self::Entity>().await?;

                debug!("{:?}", res);

                Ok(res)
            }

            async fn post(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
                let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
                let mut test = Self::Inner::default();

                debug!("{:?}", test);

                client.post(route_url(config, #route)).json(&test).headers(header).send().await?;
                Ok(())
            }

            async fn delete(mut app: MutexGuard<'_, App>, client: &Client, config: &Config) -> Result<(), anyhow::Error> {
                let header = generate_api_header(&app, crate::app::vault::SecretType::Configuration);
                let mut test = Self::Inner::default();

                debug!("{:?}", test);

                client.delete(route_url(config, #route)).json(&test).headers(header).send().await?;
                Ok(())
            }
        }
    ))
}
