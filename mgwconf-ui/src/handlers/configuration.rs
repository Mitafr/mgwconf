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
        k if *k == Key::Enter && app.get_configuration_state().selected_entity().is_none() => match app.get_configuration_state().current_selected() {
            TabId::CERTIFICATE => app.dispatch(IoEvent::PostCertificate).await.unwrap(),
            TabId::SAG => app.dispatch(IoEvent::PostSag).await.unwrap(),
            TabId::BUSINESSAPPLICATION => app.dispatch(IoEvent::PostBusinessApplication).await.unwrap(),
            TabId::PROFILE => app.dispatch(IoEvent::PostProfile).await.unwrap(),
            _ => {}
        },
        k if *k == Key::Enter && app.get_configuration_state().selected_entity().is_some() => {
            app.set_current_route_state(Some(ActiveBlock::Detailed), None);
        }
        k if *k == Key::Delete => match app.get_configuration_state().current_selected() {
            TabId::CERTIFICATE => app.dispatch(IoEvent::DeleteCertificate).await.unwrap(),
            TabId::SAG => app.dispatch(IoEvent::DeleteSag).await.unwrap(),
            TabId::BUSINESSAPPLICATION => app.dispatch(IoEvent::DeleteBusinessApplication).await.unwrap(),
            _ => {}
        },
        _ => {}
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
