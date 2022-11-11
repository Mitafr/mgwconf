use crate::{app::App, ui::prelude::ActiveBlock};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handler(key: KeyEvent, app: &mut App) {
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Home => handle_route(&key, app),
        _ => {}
    }
}

fn handle_route(key: &KeyEvent, app: &mut App) {
    match key.code {
        k if k == KeyCode::Char('c') && app.get_current_route().id != crate::app::RouteId::Configuration => {
            if app.connectivity_test {
                app.push_navigation_stack(crate::app::RouteId::Configuration, ActiveBlock::Tab);
            } else {
                app.dispatch(crate::network::IoEvent::Ping).unwrap();
            }
        }
        _ => {}
    }
}
