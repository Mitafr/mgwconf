use crate::ui::prelude::*;

use crate::ui::{
    configuration::{draw_configuration, draw_configuration_user_block},
    home::{draw_home, draw_secrets_dialog},
    utils::get_main_layout_margin,
};

pub mod configuration;
pub mod home;
pub mod prelude;
pub mod utils;

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let margin = get_main_layout_margin(app);
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .margin(margin)
        .split(f.size());

    if app.get_current_route().id == RouteId::Home {
        draw_home(f, app, parent_layout[0]);
    } else {
        draw_routes(f, app, parent_layout[0]);
    }
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::Home => {
            draw_home(f, app, chunks[1]);
        }
        RouteId::Configuration => {
            if current_route.active_block == ActiveBlock::TabSelected && app.configuration_state.is_tab_selected() {
                draw_configuration(f, app, chunks[1]);
            }
            draw_configuration_user_block(f, app, chunks[0]);
        }
        RouteId::SecretsDialog => {
            draw_secrets_dialog(f, app);
        }
    };
}
