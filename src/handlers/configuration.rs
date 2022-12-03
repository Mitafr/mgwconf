use crate::{
    app::{
        state::{State, TabId},
        App,
    },
    event::Key,
    network::IoEvent,
    ui::prelude::ActiveBlock,
};

pub fn handler(key: Key, app: &mut App) {
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Tab => handle_tab(&key, app),
        a if a == ActiveBlock::TabSelected && app.configuration_state.is_tab_selected() => handle_inner_conf(&key, app),
        ActiveBlock::Detailed => handle_detailed(&key, app),
        _ => {}
    }
}

fn handle_tab(key: &Key, app: &mut App) {
    match key {
        k if [Key::Down].contains(k) && !app.configuration_state.is_tab_selected() => {
            app.configuration_state.next();
        }
        k if [Key::Up].contains(k) && !app.configuration_state.is_tab_selected() => {
            app.configuration_state.back();
        }
        k if [Key::Enter, Key::Tab].contains(k) && !app.configuration_state.is_tab_selected() => {
            app.set_current_route_state(Some(ActiveBlock::TabSelected), None);
            match app.configuration_state.current_tab() {
                0 => app.dispatch(IoEvent::GetAllCertificates).unwrap(),
                1 => app.dispatch(IoEvent::GetAllSags).unwrap(),
                2 => app.dispatch(IoEvent::GetAllBusinessApplications).unwrap(),
                _ => {}
            }
            app.configuration_state.wait_for_load();
            app.configuration_state.select_current();
        }
        _ => {}
    }
}

fn handle_inner_conf(key: &Key, app: &mut App) {
    match key {
        k if k == &Key::Tab => {
            app.configuration_state.unselect_current();
            app.set_current_route_state(Some(ActiveBlock::Tab), None);
        }
        k if [Key::Down].contains(k) => app.configuration_state.next(),
        k if [Key::Up].contains(k) => app.configuration_state.back(),
        k if *k == Key::Enter && app.configuration_state.selected_entity().is_none() => match app.configuration_state.current_selected() {
            TabId::CERTIFICATE => app.dispatch(IoEvent::PostCertificate).unwrap(),
            TabId::SAG => app.dispatch(IoEvent::PostSag).unwrap(),
            TabId::BUSINESSAPPLICATION => app.dispatch(IoEvent::PostBusinessApplication).unwrap(),
            _ => {}
        },
        k if *k == Key::Enter && app.configuration_state.selected_entity().is_some() => {
            app.set_current_route_state(Some(ActiveBlock::Detailed), None);
        }
        k if *k == Key::Delete => match app.configuration_state.current_selected() {
            TabId::CERTIFICATE => app.dispatch(IoEvent::DeleteCertificate).unwrap(),
            TabId::SAG => app.dispatch(IoEvent::DeleteSag).unwrap(),
            TabId::BUSINESSAPPLICATION => app.dispatch(IoEvent::DeleteBusinessApplication).unwrap(),
            _ => {}
        },
        _ => {}
    }
}

fn handle_detailed(key: &Key, _app: &mut App) {
    match key {
        _ => {}
    }
}
