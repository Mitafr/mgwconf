use mgwconf_network::AppConfig;

use crate::app::state::State;
use crate::app::UiAppTrait;
use crate::ui::prelude::*;

use crate::ui::{
    configuration::{draw_configuration, draw_configuration_user_block},
    home::draw_home,
};

use self::configuration::{draw_detailed_entity, draw_error};

pub mod configuration;
pub mod fmt;
pub mod home;
pub mod prelude;
pub mod utils;

pub fn draw_main_layout<A, C>(f: &mut Frame, app: &A)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .margin(0)
        .split(f.size());

    if app.get_current_route().id == RouteId::Home {
        draw_home(f, app, parent_layout[0]);
    } else {
        draw_routes(f, app, parent_layout[0]);
    }
}

pub fn draw_routes<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    match (current_route.id, current_route.active_block) {
        (RouteId::Home, _) => {
            draw_home(f, app, chunks[1]);
        }
        (RouteId::Configuration, ActiveBlock::Error) => {
            if app.get_configuration_state().is_tab_selected() {
                draw_error(f, app, layout_chunk);
            }
        }
        (RouteId::Configuration, ActiveBlock::Detailed) => {
            if app.get_configuration_state().is_tab_selected()
                && app.get_configuration_state().selected_entity().is_some()
            {
                draw_detailed_entity(f, app, layout_chunk);
            }
        }
        (RouteId::Configuration, ActiveBlock::TabSelected) => {
            if app.get_configuration_state().is_tab_selected() {
                draw_configuration(f, app, chunks[1]);
            }
            draw_configuration_user_block(f, app, chunks[0]);
        }
        _ => {
            draw_configuration_user_block(f, app, chunks[0]);
        }
    };
}
