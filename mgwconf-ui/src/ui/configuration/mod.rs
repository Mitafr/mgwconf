use mgwconf_network::mgw_configuration::InnerEntityTrait;
use mgwconf_network::AppConfig;
use ratatui::widgets::Wrap;

use crate::app::state::State;
use crate::ui::prelude::*;

pub const CONFIGURATION_USER_TAB: [&str; 6] = [
    "Certificates",
    "SAGs",
    "Business Application",
    "Profiles",
    "Api Proxy",
    "Forward Proxy",
];

pub fn draw_configuration_user_block<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &CONFIGURATION_USER_TAB,
        (true, true),
        Some(app.get_configuration_state().current_tab()),
        Borders::ALL,
    );
}

pub fn draw_configuration<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let configuration = Block::default()
        .title(Span::styled(
            app.get_configuration_state().current_selected().to_string(),
            Style::default(),
        ))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left);
    f.render_widget(configuration, layout_chunk);
    let area = centered_rect(97, 90, layout_chunk);
    match app.get_configuration_state().current_tab() {
        0 => draw_configuration_certificates(f, app, area),
        1 => draw_configuration_sags(f, app, area),
        2 => draw_configuration_business_applications(f, app, area),
        3 => draw_configuration_profiles(f, app, area),
        4 => draw_configuration_api_proxies(f, app, area),
        5 => draw_configuration_forward_proxies(f, app, area),
        _ => {}
    }
}

pub fn draw_error<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let configuration = Block::default()
        .title(Span::styled(
            app.get_configuration_state().current_selected().to_string(),
            Style::default(),
        ))
        .borders(Borders::ALL)
        .title_alignment(Alignment::Left);
    f.render_widget(configuration, layout_chunk);
    let area = centered_rect(97, 50, layout_chunk);
    let error = app.pop_error();
    let paragraph = Paragraph::new(error.unwrap().root_cause().to_string())
        .style(Style::default().bg(Color::Reset).fg(Color::Red))
        .block(Block::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, area);
}

pub fn draw_configuration_sags<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let sags = &app.get_configuration_state().sags;
    let mut sags_str = Vec::new();
    sags_str.push("Add SAG".to_owned());
    sags_str.extend(
        sags.iter()
            .map(|s| s.hostname.to_owned())
            .collect::<Vec<String>>(),
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &sags_str,
        (true, true),
        Some(app.get_configuration_state().current_pan()),
        Borders::NONE,
    );
}

pub fn draw_configuration_certificates<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let certificates = &app.get_configuration_state().certificates;
    let mut certificates_str = Vec::new();
    certificates_str.push("Add Certificate".to_owned());
    certificates_str.extend(
        certificates
            .iter()
            .map(|s| s.alias.to_owned())
            .collect::<Vec<String>>(),
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &certificates_str,
        (true, true),
        Some(app.get_configuration_state().current_pan()),
        Borders::NONE,
    );
}

pub fn draw_configuration_business_applications<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let business_applications = &app.get_configuration_state().business_applications;
    let mut business_applications_str = Vec::new();
    business_applications_str.push("Add Business Application".to_owned());
    business_applications_str.extend(
        business_applications
            .iter()
            .map(|s| s.application_name.to_owned())
            .collect::<Vec<String>>(),
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &business_applications_str,
        (true, true),
        Some(app.get_configuration_state().current_pan()),
        Borders::NONE,
    );
}

pub fn draw_configuration_profiles<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let profiles = &app.get_configuration_state().profiles;
    let mut profiles_str = Vec::new();
    profiles_str.push("Add Profile".to_owned());
    profiles_str.extend(
        profiles
            .iter()
            .map(|s| s.profile_name.to_owned())
            .collect::<Vec<String>>(),
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &profiles_str,
        (true, true),
        Some(app.get_configuration_state().current_pan()),
        Borders::NONE,
    );
}

pub fn draw_configuration_api_proxies<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let api_proxies = &app.get_configuration_state().apiproxy;
    let mut api_proxies_str = Vec::new();
    api_proxies_str.push("Add Api Proxy".to_owned());
    api_proxies_str.extend(
        api_proxies
            .iter()
            .map(
                |s: &mgwconf_network::mgw_configuration::models::ApiGatewayInfoEntity| {
                    s.environment.to_string()
                },
            )
            .collect::<Vec<String>>(),
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &api_proxies_str,
        (true, true),
        Some(app.get_configuration_state().current_pan()),
        Borders::NONE,
    );
}

pub fn draw_configuration_forward_proxies<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let forward_proxies = &app.get_configuration_state().forwardproxy;
    let mut forward_proxies_str = Vec::new();
    forward_proxies_str.push("Add Forward Proxy".to_owned());
    forward_proxies_str.extend(
        forward_proxies
            .iter()
            .map(
                |s: &mgwconf_network::mgw_configuration::models::ForwardProxyEntity| {
                    s.hostname.to_owned()
                },
            )
            .collect::<Vec<String>>(),
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "",
        &forward_proxies_str,
        (true, true),
        Some(app.get_configuration_state().current_pan()),
        Borders::NONE,
    );
}

pub fn draw_detailed_entity<A, C>(f: &mut Frame, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    C: AppConfig,
{
    let entity: Box<&dyn InnerEntityTrait> =
        app.get_configuration_state().selected_entity().unwrap();
    let configuration = Block::default()
        .title(format!(
            "{} - {}",
            entity.as_ref().entity_type(),
            entity.as_ref().name()
        ))
        .title_alignment(Alignment::Left);
    f.render_widget(configuration, layout_chunk);
    let area = centered_rect(100, 95, layout_chunk);
    let paragraph = Paragraph::new(entity.to_string())
        .style(Style::default().bg(Color::Reset).fg(Color::White))
        .wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}
