use crate::{app::App, ui::prelude::ActiveBlock};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handler(key: KeyEvent, app: &mut App) {
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Tab => handle_tab(&key, app),
        ActiveBlock::TabSelected => handle_inner_conf(&key, app),
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
                1 => app.dispatch(crate::network::IoEvent::GetAllSags).unwrap(),
                _ => {}
            }
        }
        _ => {}
    }
}

fn handle_inner_conf(key: &KeyEvent, app: &mut App) {
    match key.code {
        k if k == KeyCode::Tab && app.configuration_state.is_tab_selected() => {
            app.configuration_state.unselect_current();
            app.set_current_route_state(Some(ActiveBlock::Tab), None);
        }
        k if [KeyCode::Down].contains(&k) && app.configuration_state.is_tab_selected() => {
            // Move inner block
        }
        k if [KeyCode::Up].contains(&k) && app.configuration_state.is_tab_selected() => {
            // Move inner block
        }
        _ => {}
    }
}
