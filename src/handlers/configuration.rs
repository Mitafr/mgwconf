use crate::{app::App, ui::prelude::ActiveBlock};

use crossterm::event::{KeyCode, KeyEvent};

pub fn handler(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Tab => app.configuration_state.next(),
        KeyCode::Enter => {
            app.set_current_route_state(Some(ActiveBlock::Empty), None);
            app.configuration_state.select_current();
        }
        _ => {}
    }
}
