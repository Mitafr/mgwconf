use mgwconf_network::AppConfig;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::UiAppTrait;

pub const SMALL_TERMINAL_HEIGHT: u16 = 45;

pub fn draw_selectable_list<A, S, C>(f: &mut Frame, _app: &A, layout_chunk: Rect, title: &str, items: &[S], highlight_state: (bool, bool), selected_index: Option<usize>, borders: Borders)
where
    A: UiAppTrait<C>,
    S: std::convert::AsRef<str>,
    C: AppConfig,
{
    let mut state = ListState::default();
    state.select(selected_index);

    let lst_items: Vec<ListItem> = items.iter().map(|i| ListItem::new(Span::raw(i.as_ref()))).collect();

    let list = List::new(lst_items)
        .block(Block::default().title(Span::styled(title, get_color(highlight_state))).borders(borders).border_style(get_color(highlight_state)))
        .style(Style::default().fg(Color::Reset))
        .highlight_style(get_color(highlight_state).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(list, layout_chunk, &mut state);
}

pub fn get_color((is_active, is_hovered): (bool, bool)) -> Style {
    match (is_active, is_hovered) {
        (true, _) => Style::default().fg(Color::Cyan),
        (false, true) => Style::default().fg(Color::Magenta),
        _ => Style::default().fg(Color::Gray),
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100 - percent_y) / 2), Constraint::Percentage(percent_y), Constraint::Percentage((100 - percent_y) / 2)].as_ref())
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100 - percent_x) / 2), Constraint::Percentage(percent_x), Constraint::Percentage((100 - percent_x) / 2)].as_ref())
        .split(popup_layout[1])[1]
}
