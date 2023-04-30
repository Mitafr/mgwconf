use crate::app::{RouteId, UiAppTrait};
use crate::event::Key;
use crate::ui::prelude::ActiveBlock;
use mgwconf_network::IoEvent;

pub fn handler<A>(key: Key, app: &mut A)
where
    A: UiAppTrait,
{
    let route = app.get_current_route();
    match route.active_block {
        ActiveBlock::Home => handle_route(&key, app),
        _ => handle_exit(&key, app),
    }
}

fn handle_route<A>(key: &Key, app: &mut A)
where
    A: UiAppTrait,
{
    match key {
        k if k == &Key::Char('c') && app.get_current_route().id != RouteId::Configuration => {
            if app.is_connected() {
                app.push_navigation_stack(RouteId::Configuration, ActiveBlock::Tab);
            } else {
                app.dispatch(IoEvent::Ping).unwrap();
            }
        }
        _ => {}
    }
}

fn handle_exit<A>(key: &Key, app: &mut A)
where
    A: UiAppTrait,
{
    if *key == Key::Esc {
        app.force_exit();
    }
}
