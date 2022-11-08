mod configuration;

use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{ActiveBlock, App};

pub fn handle_app(key: KeyEvent, app: &mut App) {
    let key_code = key.code;
    match key_code {
        KeyCode::Esc => {
            handle_escape(app);
        }
        k if k == KeyCode::Char('a') && app.get_current_route().id != crate::app::RouteId::Configuration => app.push_navigation_stack(crate::app::RouteId::Configuration, ActiveBlock::Tab),
        _ => handle_route(key, app),
    }
}

fn handle_route(key: KeyEvent, app: &mut App) {
    let current_route = app.get_current_route();
    match current_route.id {
        crate::app::RouteId::Home => todo!(),
        crate::app::RouteId::Configuration => configuration::handler(key, app),
        crate::app::RouteId::SecretsDialog => todo!(),
    }
}

pub fn handle_input(key: KeyEvent, app: &mut App) {
    let key_code = key.code;
    match key_code {
        KeyCode::Esc => {
            app.input.clear();
            handle_escape(app);
        }
        KeyCode::Enter => {
            app.input.clear();
        }
        KeyCode::Backspace => {
            if !app.input.is_empty() {
                app.input.pop();
            }
        }
        KeyCode::Char(c) => {
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
