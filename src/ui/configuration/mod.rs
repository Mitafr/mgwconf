use crate::{app::state::State, ui::prelude::*};

pub const CONFIGURATION_USER_TAB: [&str; 5] = ["Certificates", "SAGs", "Business Application", "Profiles", "Api Proxy"];

pub fn draw_configuration_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    draw_selectable_list(f, app, layout_chunk, "", &CONFIGURATION_USER_TAB, (true, true), Some(app.configuration_state.current_tab()), Borders::ALL);
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
    match app.configuration_state.current_tab() {
        0 => draw_configuration_certificates(f, app, area),
        1 => draw_configuration_sags(f, app, area),
        _ => {}
    }
}

pub fn draw_configuration_sags<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let sags = &app.configuration_state.sags;
    let mut sags_str = Vec::new();
    sags_str.push("Add SAG".to_owned());
    sags_str.extend(sags.to_vec_string());
    draw_selectable_list(f, app, layout_chunk, "", &sags_str, (true, true), Some(app.configuration_state.current_pan()), Borders::NONE);
}

pub fn draw_configuration_certificates<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let certificates = &app.configuration_state.certificates;
    let mut certificates_str = Vec::new();
    certificates_str.push("Add Certificate".to_owned());
    certificates_str.extend(certificates.to_vec_string());
    draw_selectable_list(f, app, layout_chunk, "", &certificates_str, (true, true), Some(app.configuration_state.current_pan()), Borders::NONE);
}
