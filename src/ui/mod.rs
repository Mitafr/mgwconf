use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use mgwconf_network::AppConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tui::{backend::CrosstermBackend, Terminal};

use std::io::{stdout, Stdout};

use mgwconf_network::IoEvent;
use std::sync::mpsc::Sender;

pub use mgwconf_ui::{
    app::{UiApp, UiAppTrait},
    event::Key,
};

use mgwconf_ui::config::{Args, Config};

pub fn close_application(terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>) -> Result<(), anyhow::Error> {
    if let Some(term) = terminal {
        term.show_cursor()?;
    }
    disable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

pub async fn create_app(io_tx: Sender<IoEvent>) -> (Arc<Mutex<UiApp>>, Config) {
    use crate::ask_master_key;
    use clap::Parser;

    let args = Args::parse();
    let vault_key = if args.vault_key.is_some() { args.vault_key.as_ref().unwrap().to_owned() } else { ask_master_key() };

    let config = Config::init(&args).unwrap();
    (Arc::new(Mutex::new(UiApp::new(io_tx, config.clone(), &vault_key).await)), config)
}

pub async fn start_ui<C>(app: Arc<Mutex<UiApp>>) -> Result<(), anyhow::Error>
where
    C: AppConfig,
{
    use std::time::{Duration, Instant};

    use mgwconf_network::AppTrait;
    use mgwconf_ui::app::ActiveBlock;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let config = <UiApp as AppTrait<C>>::config(&*app.lock().await);

    let tick_rate = Duration::from_millis(config.tickrate());
    let events = mgwconf_ui::event::Events::new(config.tickrate());
    let mut last_tick = Instant::now();

    'main: loop {
        let mut app = app.lock().await;
        <UiApp as AppTrait<C>>::init(&mut app).await?;

        terminal.draw(|f| {
            mgwconf_ui::ui::draw_main_layout::<UiApp, _, Config>(f, &*app);
        })?;

        terminal.hide_cursor()?;

        let cursor_offset = 2;
        terminal.backend_mut().execute(MoveTo(cursor_offset, cursor_offset))?;

        let current_route = <UiApp as UiAppTrait<C>>::get_current_route(&app);

        match events.next()? {
            mgwconf_ui::event::Event::Input(key) => {
                if key == Key::Esc && (current_route.active_block == ActiveBlock::Empty || current_route.active_block == ActiveBlock::Tab) {
                    break 'main;
                }

                let current_active_block = current_route.active_block;

                if current_active_block == ActiveBlock::Dialog {
                    mgwconf_ui::handlers::handle_input::<UiApp, Config>(key, &mut *app);
                } else {
                    mgwconf_ui::handlers::handle_app::<UiApp, Config>(key, &mut *app)
                }
            }
            mgwconf_ui::event::Event::Tick => {
                if <UiApp as UiAppTrait<C>>::get_force_exit(&app) {
                    break 'main;
                }
                <UiApp as UiAppTrait<C>>::update_on_tick(&mut app);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    close_application(Some(&mut terminal))
}
