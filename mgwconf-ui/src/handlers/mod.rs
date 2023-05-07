mod configuration;
mod home;

use mgwconf_network::AppConfig;

use crate::app::{ActiveBlock, RouteId, UiAppTrait};
use crate::event::Key;

pub fn handle_app<A, C>(key: Key, app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match key {
        Key::Esc => {
            handle_escape(app);
        }
        _ => handle_route(key, app),
    }
}

fn handle_route<A, C>(key: Key, app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let current_route = app.get_current_route();
    match current_route.id {
        RouteId::Home => home::handler(key, app),
        RouteId::Configuration => configuration::handler(key, app),
    }
}

pub fn handle_input<A, C>(key: Key, app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    match key {
        Key::Esc => {
            app.get_user_input_mut().clear();
            handle_escape(app);
        }
        Key::Enter => {
            app.get_user_input_mut().clear();
        }
        Key::Backspace => {
            if !app.get_user_input().is_empty() {
                app.get_user_input_mut().pop();
            }
        }
        Key::Char(c) => {
            if app.get_user_input_mut().len() < 48 {
                app.get_user_input_mut().push(c);
            }
        }
        _ => {}
    }
}

fn handle_escape<A, C>(app: &mut A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
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
        ActiveBlock::Detailed => {
            app.set_current_route_state(Some(ActiveBlock::TabSelected), None);
        }
        ActiveBlock::Dialog => app.reset_navigation_stack(),
        _ => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
        }
    }
}
