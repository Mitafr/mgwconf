use crate::ui::prelude::*;

const CONFIGURATION_USER_TAB: [&'static str; 5] = [&"Certificates", &"SAGs", &"Business Application", &"Profiles", &"Api Proxy"];

pub fn draw_configuration_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    draw_selectable_list(f, app, layout_chunk, "", &CONFIGURATION_USER_TAB, (true, true), Some(0));
}

pub fn draw_configuration<B>(f: &mut Frame<B>, _app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let configuration = Block::default().title(Span::styled("Configuration", Style::default())).borders(Borders::ALL).title_alignment(Alignment::Center);
    f.render_widget(configuration, layout_chunk);
}
