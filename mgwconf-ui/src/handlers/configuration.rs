use mgwconf_network::{event::IoEvent, AppConfig};

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
        a if a == ActiveBlock::TabSelected && app.get_configuration_state().is_tab_selected() => handle_inner_conf(&key, app).await,
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
        k if [Key::Enter, Key::Tab].contains(k) && !app.get_configuration_state().is_tab_selected() => {
            app.set_current_route_state(Some(ActiveBlock::TabSelected), None);
            match app.get_configuration_state().current_tab() {
                0 => app.dispatch(IoEvent::GetAllCertificates).await.unwrap(),
                1 => app.dispatch(IoEvent::GetAllSags).await.unwrap(),
                2 => app.dispatch(IoEvent::GetAllBusinessApplications).await.unwrap(),
                3 => app.dispatch(IoEvent::GetAllProfiles).await.unwrap(),
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
        k if k == &Key::Tab => {
            app.get_configuration_state_mut().unselect_current();
            app.set_current_route_state(Some(ActiveBlock::Tab), None);
        }
        k if [Key::Down].contains(k) => app.get_configuration_state_mut().next(),
        k if [Key::Up].contains(k) => app.get_configuration_state_mut().back(),
        k if *k == Key::Enter && app.get_configuration_state().selected_entity().is_none() => match dispatch_post(app).await {
            Ok(_) => get_all_after(app).await.unwrap(),
            Err(e) => log::error!("{}", e),
        },
        k if *k == Key::Enter && app.get_configuration_state().selected_entity().is_some() => {
            app.set_current_route_state(Some(ActiveBlock::Detailed), None);
        }
        k if *k == Key::Delete && app.get_configuration_state().selected_entity().is_some() => {
            todo!();
            /*
            match app.get_configuration_state().current_selected() {
                TabId::CERTIFICATE => {
                    // let t = &*app
                    //     .get_configuration_state()
                    //     .selected_entity()
                    //     .unwrap()
                    //     .as_any()
                    //     .downcast_ref::<mgwconf_network::model::certificate::Entity>()
                    //     .expect("Wasn't a trusty printer!");
                    // app.dispatch(IoEvent::DeleteCertificate(t.clone())).await.unwrap();
                    // app.dispatch(IoEvent::GetAllCertificates).await.unwrap();
                }
                TabId::SAG => {
                    // let t = &*app
                    //     .get_configuration_state()
                    //     .selected_entity()
                    //     .unwrap()
                    //     .as_any()
                    //     .downcast_ref::<mgwconf_network::model::sag::Entity>()
                    //     .expect("Wasn't a trusty printer!");
                    // app.dispatch(IoEvent::DeleteSag(t.clone())).await.unwrap();
                    // app.dispatch(IoEvent::GetAllSags).await.unwrap();
                }
                TabId::BUSINESSAPPLICATION => {
                    let t = &*app
                        .get_configuration_state()
                        .selected_entity()
                        .unwrap()
                        .as_any()
                        .downcast_ref::<mgwconf_network::model::business_application::Entity>()
                        .expect("Wasn't a trusty printer!");
                    app.dispatch(IoEvent::DeleteBusinessApplication(t.clone())).await.unwrap();
                    app.dispatch(IoEvent::GetAllBusinessApplications).await.unwrap();
                }
                _ => {}
            }
            app.get_configuration_state_mut().reload();*/
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
        TabId::CERTIFICATE => app.dispatch(IoEvent::PostCertificate).await.map_err(anyhow::Error::from),
        TabId::SAG => app.dispatch(IoEvent::PostSag).await.map_err(anyhow::Error::from),
        TabId::BUSINESSAPPLICATION => app.dispatch(IoEvent::PostBusinessApplication).await.map_err(anyhow::Error::from),
        TabId::PROFILE => app.dispatch(IoEvent::PostProfile).await.map_err(anyhow::Error::from),
        _ => Ok(()),
    }
}

async fn get_all_after<A, C>(app: &mut A) -> Result<(), anyhow::Error>
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match app.get_configuration_state().current_selected() {
        TabId::CERTIFICATE => app.dispatch(IoEvent::GetAllCertificates).await,
        TabId::SAG => app.dispatch(IoEvent::GetAllSags).await.map_err(anyhow::Error::from),
        TabId::BUSINESSAPPLICATION => app.dispatch(IoEvent::GetAllBusinessApplications).await.map_err(anyhow::Error::from),
        TabId::PROFILE => app.dispatch(IoEvent::GetAllProfiles).await.map_err(anyhow::Error::from),
        _ => Ok(()),
    }
}

fn handle_detailed<A, C>(key: &Key, _app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match key {
        _ => {}
    }
}
