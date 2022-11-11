mod configuration;
mod home;

use crate::{
    app::{ActiveBlock, App},
    event::Key,
};

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ => handle_route(key, app),
    }
}

fn handle_route(key: Key, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.id {
        crate::app::RouteId::Home => home::handler(key, app),
        crate::app::RouteId::Configuration => configuration::handler(key, app),
    }
}

pub fn handle_input(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.input.clear();
            handle_escape(app);
        }
        Key::Enter => {
            app.input.clear();
        }
        Key::Backspace => {
            if !app.input.is_empty() {
                app.input.pop();
            }
        }
        Key::Char(c) => {
            if app.input.len() < 48 {
                app.input.push(c);
            }
        }
        _ => {}
    }
}

fn handle_escape(app: &mut App) {
    match app.get_current_route().active_block {
        ActiveBlock::Error => {
            app.pop_navigation_stack();
        }
        ActiveBlock::Home => {
            app.pop_navigation_stack();
        }
        ActiveBlock::TabSelected => {
            app.set_current_route_state(Some(ActiveBlock::Tab), None);
            app.reset_selected_states();
        }
        ActiveBlock::Dialog => app.reset_navigation_stack(),
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
