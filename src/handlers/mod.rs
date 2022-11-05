use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{ActiveBlock, App};

pub fn handle_app(key: KeyEvent, app: &mut App) {
    let key_code = key.code;
    match key_code {
        KeyCode::Esc => {
            handle_escape(app);
        }
        KeyCode::Char('v') => app.push_navigation_stack(crate::app::RouteId::SecretsDialog, ActiveBlock::Dialog),
        KeyCode::Char('a') => app.push_navigation_stack(crate::app::RouteId::Configuration, ActiveBlock::Home),
        _ => {}
    }
}

pub fn input_handler(key: KeyEvent, app: &mut App) {
    let key_code = key.code;
    match key_code {
        KeyCode::Esc => {
            app.input.clear();
            handle_escape(app);
        }
        KeyCode::Enter => {
            app.set_secret(Some(app.input.clone()));
            app.input.clear();
        }
        KeyCode::Backspace => {
            if app.input.len() > 0 {
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
        ActiveBlock::Dialog => app.reset_navigation_stack(),
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
