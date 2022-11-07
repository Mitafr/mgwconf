use crate::ui::prelude::*;

pub fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
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
        app.config.as_ref().unwrap().remote_ip,
        app.config.as_ref().unwrap().remote_port,
        app.connectivity_test
    ))
    .style(Style::default().bg(Color::Reset).fg(Color::White))
    .block(Block::default())
    .alignment(Alignment::Left);

    f.render_widget(welcome, layout_chunk);
    f.render_widget(paragraph, chunks[0]);
}

pub fn draw_secrets_dialog<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let current_secret = &app.vault.current_secret;

    let block = Block::default().title(format!("{} (use tabs to switch to other secrets)", current_secret)).borders(Borders::ALL);
    let area = centered_rect(80, 40, f.size());
    let input = Paragraph::new(app.input.clone())
        .style(Style::default().bg(Color::Reset).fg(Color::White))
        .block(Block::default())
        .alignment(Alignment::Left);
    let input_area = centered_rect(50, 10, f.size());
    f.render_widget(block, area);
    f.render_widget(input, input_area);
}
