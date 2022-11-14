use crate::{app::App, event::Key, ui::prelude::ActiveBlock};

pub fn handler(key: Key, app: &mut App) {
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Home => handle_route(&key, app),
        _ => handle_exit(&key, app),
    }
}

fn handle_route(key: &Key, app: &mut App) {
    match key {
        k if k == &Key::Char('c') && app.get_current_route().id != crate::app::RouteId::Configuration => {
            if app.connectivity_test {
                app.push_navigation_stack(crate::app::RouteId::Configuration, ActiveBlock::Tab);
            } else {
                app.dispatch(crate::network::IoEvent::Ping).unwrap();
            }
        }
        _ => {}
    }
}

fn handle_exit(key: &Key, app: &mut App) {
    if *key == Key::Esc {
        app.force_exit = true;
    }
}
