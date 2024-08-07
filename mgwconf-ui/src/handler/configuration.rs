use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use mgwconf_network::{
    event::IoEvent,
    mgw_configuration::models::BusinessApplicationEntity,
    mgw_configuration::models::{
        ApiGatewayInfoEntity, ApplicationProfileEntity, CertificateEntity, ForwardProxyEntity,
        SagEntity,
    },
    AppConfig,
};

use crate::app::{
    state::{State, TabId},
    UiAppTrait,
};
use crate::event::Key;
use crate::ui::prelude::ActiveBlock;

pub async fn handler<A, C>(key: Key, app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Tab => handle_tab(&key, app).await,
        a if a == ActiveBlock::TabSelected && app.get_configuration_state().is_tab_selected() => {
            handle_inner_conf(&key, app).await
        }
        ActiveBlock::Detailed => handle_detailed(&key, app),
        _ => {}
    }
}

async fn handle_tab<A, C>(key: &Key, app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match key {
        k if [Key::Down].contains(k) && !app.get_configuration_state().is_tab_selected() => {
            app.get_configuration_state_mut().next();
        }
        k if [Key::Up].contains(k) && !app.get_configuration_state().is_tab_selected() => {
            app.get_configuration_state_mut().back();
        }
        k if [Key::Enter, Key::Tab].contains(k)
            && !app.get_configuration_state().is_tab_selected() =>
        {
            app.set_current_route_state(Some(ActiveBlock::TabSelected), None);
            match app.get_configuration_state().current_tab() {
                0 => app.dispatch(IoEvent::GetAllCertificates).await.unwrap(),
                1 => app.dispatch(IoEvent::GetAllSags).await.unwrap(),
                2 => app
                    .dispatch(IoEvent::GetAllBusinessApplications)
                    .await
                    .unwrap(),
                3 => app.dispatch(IoEvent::GetAllProfiles).await.unwrap(),
                4 => app
                    .dispatch(IoEvent::GetAllApiGatewayInfoEntity)
                    .await
                    .unwrap(),
                5 => app
                    .dispatch(IoEvent::GetAllForwardProxyEntity)
                    .await
                    .unwrap(),
                _ => {}
            }
            app.get_configuration_state_mut().wait_for_load();
            app.get_configuration_state_mut().select_current();
        }
        _ => {}
    }
}

async fn handle_inner_conf<A, C>(key: &Key, app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match key {
        &Key::Tab => {
            execute!(std::io::stdout(), EnableMouseCapture).unwrap();
            app.get_configuration_state_mut().unselect_current();
            app.set_current_route_state(Some(ActiveBlock::Tab), None);
        }
        k if [Key::Down].contains(k) => app.get_configuration_state_mut().next(),
        k if [Key::Up].contains(k) => app.get_configuration_state_mut().back(),
        k if *k == Key::Enter && app.get_configuration_state().selected_entity().is_none() => {
            match dispatch_post(app).await {
                Ok(_) => get_all_after(app).await.unwrap(),
                Err(e) => log::error!("{}", e),
            }
        }
        k if *k == Key::Enter && app.get_configuration_state().selected_entity().is_some() => {
            execute!(std::io::stdout(), DisableMouseCapture).unwrap();
            app.set_current_route_state(Some(ActiveBlock::Detailed), None);
        }
        k if *k == Key::Delete && app.get_configuration_state().selected_entity().is_some() => {
            let any_entity = app.get_configuration_state().selected_entity().unwrap();
            match app.get_configuration_state().current_selected() {
                TabId::CERTIFICATE => {
                    let entity = any_entity
                        .as_any()
                        .downcast_ref::<CertificateEntity>()
                        .expect("Wasn't a trusty printer!")
                        .to_owned();
                    app.dispatch(IoEvent::DeleteCertificate(entity))
                        .await
                        .unwrap();
                    app.dispatch(IoEvent::GetAllCertificates).await.unwrap();
                }
                TabId::SAG => {
                    let entity = any_entity
                        .as_any()
                        .downcast_ref::<SagEntity>()
                        .expect("Wasn't a trusty printer!")
                        .to_owned();
                    app.dispatch(IoEvent::DeleteSag(entity)).await.unwrap();
                    app.dispatch(IoEvent::GetAllSags).await.unwrap();
                }
                TabId::BUSINESSAPPLICATION => {
                    let entity: BusinessApplicationEntity = any_entity
                        .as_any()
                        .downcast_ref::<BusinessApplicationEntity>()
                        .expect("Wasn't a trusty printer!")
                        .to_owned();
                    app.dispatch(IoEvent::DeleteBusinessApplication(entity))
                        .await
                        .unwrap();
                    app.dispatch(IoEvent::GetAllBusinessApplications)
                        .await
                        .unwrap();
                }
                TabId::PROFILE => {
                    let entity = any_entity
                        .as_any()
                        .downcast_ref::<ApplicationProfileEntity>()
                        .expect("Wasn't a trusty printer!")
                        .to_owned();
                    app.dispatch(IoEvent::DeleteProfile(entity)).await.unwrap();
                    app.dispatch(IoEvent::GetAllProfiles).await.unwrap();
                }
                TabId::APIPROXY => {
                    let entity = any_entity
                        .as_any()
                        .downcast_ref::<ApiGatewayInfoEntity>()
                        .expect("Wasn't a trusty printer!")
                        .to_owned();
                    app.dispatch(IoEvent::DeleteApiGatewayInfoEntity(entity))
                        .await
                        .unwrap();
                    app.dispatch(IoEvent::GetAllApiGatewayInfoEntity)
                        .await
                        .unwrap();
                }
                TabId::FORWARDPROXY => {
                    let entity = any_entity
                        .as_any()
                        .downcast_ref::<ForwardProxyEntity>()
                        .expect("Wasn't a trusty printer!")
                        .to_owned();
                    app.dispatch(IoEvent::DeleteForwardProxyEntity(entity))
                        .await
                        .unwrap();
                    app.dispatch(IoEvent::GetAllForwardProxyEntity)
                        .await
                        .unwrap();
                }
            }
            app.get_configuration_state_mut().reload();
        }
        _ => {}
    }
}

async fn dispatch_post<A, C>(app: &mut A) -> Result<(), anyhow::Error>
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match app.get_configuration_state().current_selected() {
        TabId::CERTIFICATE => app
            .dispatch(IoEvent::PostCertificate(CertificateEntity {
                alias: String::from("test"),
                certificate_x509: String::from("test"),
                private_key: None,
            }))
            .await
            .map_err(anyhow::Error::from),
        TabId::SAG => app
            .dispatch(IoEvent::PostSag(SagEntity {
                hostname: String::from("test3"),
                port: 48002,
                message_partner_name: Some(String::from("SAG MP")),
                user_dns: vec![String::from("cn=apitest,ou=apicore,o=agrifrpp,o=swift")],
                lau_key: Some(String::from("Abcd1234Abcd1234Abcd1234Abcd1234")),
                ssl_dn: Some(String::from("ss")),
                active: Some(false),
                public_certificate_alias: Some(String::from("test")),
            }))
            .await
            .map_err(anyhow::Error::from),
        TabId::BUSINESSAPPLICATION => app
            .dispatch(IoEvent::PostBusinessApplication(
                BusinessApplicationEntity::default(),
            ))
            .await
            .map_err(anyhow::Error::from),
        TabId::PROFILE => app
            .dispatch(IoEvent::PostProfile(ApplicationProfileEntity::default()))
            .await
            .map_err(anyhow::Error::from),
        TabId::APIPROXY => app
            .dispatch(IoEvent::PostApiGatewayInfoEntity(
                ApiGatewayInfoEntity::default(),
            ))
            .await
            .map_err(anyhow::Error::from),
        TabId::FORWARDPROXY => app
            .dispatch(IoEvent::PostForwardProxyEntity(
                ForwardProxyEntity::default(),
            ))
            .await
            .map_err(anyhow::Error::from),
    }
}

async fn get_all_after<A, C>(app: &mut A) -> Result<(), anyhow::Error>
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match app.get_configuration_state().current_selected() {
        TabId::CERTIFICATE => app.dispatch(IoEvent::GetAllCertificates).await,
        TabId::SAG => app
            .dispatch(IoEvent::GetAllSags)
            .await
            .map_err(anyhow::Error::from),
        TabId::BUSINESSAPPLICATION => app
            .dispatch(IoEvent::GetAllBusinessApplications)
            .await
            .map_err(anyhow::Error::from),
        TabId::PROFILE => app
            .dispatch(IoEvent::GetAllProfiles)
            .await
            .map_err(anyhow::Error::from),
        TabId::APIPROXY => app
            .dispatch(IoEvent::GetAllApiGatewayInfoEntity)
            .await
            .map_err(anyhow::Error::from),
        TabId::FORWARDPROXY => app
            .dispatch(IoEvent::GetAllForwardProxyEntity)
            .await
            .map_err(anyhow::Error::from),
    }
}

fn handle_detailed<A, C>(key: &Key, _app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    if key == &Key::Enter {}
}
