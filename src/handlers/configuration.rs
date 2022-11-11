use crate::{app::App, network::IoEvent, ui::prelude::ActiveBlock};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handler(key: KeyEvent, app: &mut App) {
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Tab => handle_tab(&key, app),
        a if a == ActiveBlock::TabSelected && app.configuration_state.is_tab_selected() => handle_inner_conf(&key, app),
        _ => {}
    }
}

fn handle_tab(key: &KeyEvent, app: &mut App) {
    match key.code {
        k if [KeyCode::Down].contains(&k) && !app.configuration_state.is_tab_selected() => {
            app.configuration_state.next();
        }
        k if [KeyCode::Up].contains(&k) && !app.configuration_state.is_tab_selected() => {
            app.configuration_state.back();
        }
        k if [KeyCode::Enter, KeyCode::Tab].contains(&k) && !app.configuration_state.is_tab_selected() => {
            app.set_current_route_state(Some(ActiveBlock::TabSelected), None);
            app.configuration_state.select_current();
            match app.configuration_state.current() {
                0 => app.dispatch(IoEvent::GetAllCertificate).unwrap(),
                1 => app.dispatch(IoEvent::GetAllSags).unwrap(),
                _ => {}
            }
        }
        _ => {}
    }
}

fn handle_inner_conf(key: &KeyEvent, app: &mut App) {
    match key.code {
        k if k == KeyCode::Tab => {
            app.configuration_state.unselect_current();
            app.set_current_route_state(Some(ActiveBlock::Tab), None);
        }
        k if [KeyCode::Down].contains(&k) => {
            // Move inner block
        }
        k if [KeyCode::Up].contains(&k) => {
            // Move inner block
        }
        k if k == KeyCode::Enter => match app.configuration_state.current_selected() {
            0 => app.dispatch(IoEvent::PostCertificate).unwrap(),
            1 => app.dispatch(IoEvent::PostSag).unwrap(),
            _ => {}
        },
        _ => {}
    }
}
