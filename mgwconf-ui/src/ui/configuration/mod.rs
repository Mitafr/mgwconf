use mgwconf_network::model::InnerEntityTrait;
use mgwconf_network::AppConfig;

use crate::app::state::State;
use crate::ui::prelude::*;

pub const CONFIGURATION_USER_TAB: [&str; 5] = ["Certificates", "SAGs", "Business Application", "Profiles", "Api Proxy"];

pub fn draw_configuration_user_block<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    draw_selectable_list(f, app, layout_chunk, "", &CONFIGURATION_USER_TAB, (true, true), Some(app.get_configuration_state().current_tab()), Borders::ALL);
}

pub fn draw_configuration<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    let configuration = Block::default()
        .title(Span::styled(app.get_configuration_state().current_selected().to_string(), Style::default()))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left);
    f.render_widget(configuration, layout_chunk);
    let area = centered_rect(97, 90, layout_chunk);
    match app.get_configuration_state().current_tab() {
        0 => draw_configuration_certificates(f, app, area),
        1 => draw_configuration_sags(f, app, area),
        2 => draw_configuration_business_applications(f, app, area),
        _ => {}
    }
}

pub fn draw_configuration_sags<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    let sags = &app.get_configuration_state().sags;
    let mut sags_str = Vec::new();
    sags_str.push("Add SAG".to_owned());
    sags_str.extend(sags.to_vec_string());
    draw_selectable_list(f, app, layout_chunk, "", &sags_str, (true, true), Some(app.get_configuration_state().current_pan()), Borders::NONE);
}

pub fn draw_configuration_certificates<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    let certificates = &app.get_configuration_state().certificates;
    let mut certificates_str = Vec::new();
    certificates_str.push("Add Certificate".to_owned());
    certificates_str.extend(certificates.to_vec_string());
    draw_selectable_list(f, app, layout_chunk, "", &certificates_str, (true, true), Some(app.get_configuration_state().current_pan()), Borders::NONE);
}

pub fn draw_configuration_business_applications<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    let business_applications = &app.get_configuration_state().business_applications;
    let mut business_applications_str = Vec::new();
    business_applications_str.push("Add Business Application".to_owned());
    business_applications_str.extend(business_applications.to_vec_string());
    draw_selectable_list(f, app, layout_chunk, "", &business_applications_str, (true, true), Some(app.get_configuration_state().current_pan()), Borders::NONE);
}

pub fn draw_detailed_entity<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    let entity: Box<&dyn InnerEntityTrait> = app.get_configuration_state().selected_entity().unwrap();
    let configuration = Block::default()
        .title(Span::styled(entity.as_ref().get_name(), Style::default()))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left);
    f.render_widget(configuration, layout_chunk);
    let area = centered_rect(97, 90, layout_chunk);
    let paragraph = Paragraph::new(entity.to_string())
        .style(Style::default().bg(Color::Reset).fg(Color::White))
        .block(Block::default())
        .alignment(Alignment::Left);
    f.render_widget(paragraph, area);
}
