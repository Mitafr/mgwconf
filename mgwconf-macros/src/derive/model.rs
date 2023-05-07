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
        impl<A: AppTrait<C>, C: AppConfig> super::prelude::ModelTrait<A, C> for #ident {
            type Collection = self::Entities;
            type Entity = self::Entity;

            async fn get(mut app: MutexGuard<'_, A>, client: &Client, config: &C) -> Result<Self::Entity, anyhow::Error> {
                let header = generate_api_header(&app, mgwconf_vault::SecretType::Configuration);
                let res = client.get(route_url(config, #route)).headers(header).send().await?;
                if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
                    return Err(anyhow::Error::msg(format!("{:?}", res)));
                }
                let res = res.json::<Self::Entity>().await?;

                debug!("{:?}", res);

                Ok(res)
            }

            async fn get_all(mut app: MutexGuard<'_, A>, client: &Client, config: &C) -> Result<Self::Collection, anyhow::Error> {
                let header = generate_api_header(&app, mgwconf_vault::SecretType::Configuration);
                let res = client.get(route_url(config, #route)).headers(header).send().await?;
                if ![StatusCode::OK, StatusCode::NO_CONTENT].contains(&res.status()) {
                    return Err(anyhow::Error::msg(format!("{:?}", res)));
                }
                let res = res.json::<Self::Collection>().await?;

                debug!("{:?}", res);

                Ok(res)
            }

            async fn post(mut app: MutexGuard<'_, A>, client: &Client, config: &C) -> Result<(), anyhow::Error> {
                let header = generate_api_header(&app, mgwconf_vault::SecretType::Configuration);
                let mut test = Self::Entity::default();

                debug!("{:?}", test);

                client.post(route_url(config, #route)).json(&test).headers(header).send().await?;
                Ok(())
            }

            async fn delete(mut app: MutexGuard<'_, A>, client: &Client, config: &C) -> Result<(), anyhow::Error> {
                let header = generate_api_header(&app, mgwconf_vault::SecretType::Configuration);
                let mut test = Self::Entity::default();

                debug!("{:?}", test);

                client.delete(route_url(config, #route)).json(&test).headers(header).send().await?;
                Ok(())
            }
        }
    ))
}
