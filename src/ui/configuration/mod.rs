use crate::ui::prelude::*;

pub const CONFIGURATION_USER_TAB: [&str; 5] = ["Certificates", "SAGs", "Business Application", "Profiles", "Api Proxy"];

pub fn draw_configuration_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    draw_selectable_list(f, app, layout_chunk, "", &CONFIGURATION_USER_TAB, (true, true), Some(app.configuration_state.current()), Borders::ALL);
}

pub fn draw_configuration<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let configuration = Block::default()
        .title(Span::styled(CONFIGURATION_USER_TAB[app.configuration_state.current_selected()], Style::default()))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Center);
    f.render_widget(configuration, layout_chunk);
    let area = centered_rect(97, 90, layout_chunk);
    match app.configuration_state.current() {
        1 => draw_configuration_sags(f, app, area),
        _ => {}
    }
}

pub fn draw_configuration_sags<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let mut sags = Vec::new();
    sags.push(String::from("SAG1"));
    sags.push(String::from("SAG2"));
    sags.push(String::from("SAG3"));

    draw_selectable_list(f, app, layout_chunk, "", &sags, (true, true), Some(0), Borders::NONE);
}
