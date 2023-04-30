#[cfg(feature = "ui")]
use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
#[cfg(feature = "ui")]
use std::sync::Arc;
#[cfg(feature = "ui")]
use tokio::sync::Mutex;
#[cfg(feature = "ui")]
use tui::{backend::CrosstermBackend, Terminal};

#[cfg(feature = "ui")]
use std::io::Stdout;

#[cfg(feature = "ui")]
pub use mgwconf_ui::{
    app::{UiApp, UiAppTrait},
    event::Key,
};

#[cfg(feature = "ui")]
pub fn close_application(terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>) -> Result<(), anyhow::Error> {
    if let Some(term) = terminal {
        term.show_cursor()?;
    }
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

#[cfg(feature = "ui")]
pub async fn start_ui<A>(app: &Arc<Mutex<A>>) -> Result<(), anyhow::Error>
where
    A: UiAppTrait,
{
    use std::{
        io::stdout,
        time::{Duration, Instant},
    };

    use mgwconf_ui::app::ActiveBlock;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(app.lock().await.get_config().as_ref().unwrap().tick_rate);
    let events = mgwconf_ui::event::Events::new(app.lock().await.get_config().as_ref().unwrap().tick_rate);
    let mut last_tick = Instant::now();

    'main: loop {
        let mut app = app.lock().await;
        app.init().await.unwrap();

        terminal.draw(|f| {
            mgwconf_ui::ui::draw_main_layout(f, &*app);
        })?;

        terminal.hide_cursor()?;

        let cursor_offset = 2;
        terminal.backend_mut().execute(MoveTo(cursor_offset, cursor_offset))?;

        match events.next()? {
            mgwconf_ui::event::Event::Input(key) => {
                if key == Key::Esc && (app.get_current_route().active_block == ActiveBlock::Empty || app.get_current_route().active_block == ActiveBlock::Tab) {
                    break 'main;
                }

                let current_active_block = app.get_current_route().active_block;

                if current_active_block == ActiveBlock::Dialog {
                    mgwconf_ui::handlers::handle_input(key, &mut *app);
                } else {
                    mgwconf_ui::handlers::handle_app(key, &mut *app)
                }
            }
            mgwconf_ui::event::Event::Tick => {
                if app.get_force_exit() {
                    break 'main;
                }
                app.update_on_tick();
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    close_application(Some(&mut terminal))
}
