use mgwconf_network::AppConfig;

use crate::ui::prelude::*;

pub fn draw_home<A, B, C>(f: &mut Frame<B>, app: &A, layout_chunk: Rect)
where
    A: UiAppTrait<C>,
    B: Backend,
    C: AppConfig,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .horizontal_margin(2)
        .vertical_margin(2)
        .split(layout_chunk);

    let welcome = Block::default().title(Span::styled("Home", Style::default())).title_alignment(Alignment::Left).borders(Borders::ALL);
    let current_user = whoami::username();
    let paragraph = Paragraph::new(format!(
        "Utilisateur : {}\nRemote IP : {}\nRemote Port : {}\nConnectivity : {}",
        current_user,
        app.config().remote_ip(),
        app.config().remote_port(),
        if app.is_connected() { "OK" } else { "KO" }
    ))
    .style(Style::default().bg(Color::Reset).fg(Color::White))
    .block(Block::default())
    .alignment(Alignment::Left);

    f.render_widget(welcome, layout_chunk);
    f.render_widget(paragraph, chunks[0]);
}
