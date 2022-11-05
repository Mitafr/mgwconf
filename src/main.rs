use network::{IoEvent, Network};
use std::{
    io::stdout,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::Mutex;

use anyhow::Result;
use app::{ActiveBlock, App};
use clap::Parser;
use crossterm::{
    cursor::MoveTo,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

mod app;
mod config;
mod handlers;
mod network;
mod ui;

use crate::config::{Args, Config};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut config = Config::init(&args).expect("Could not init config");
    config.init_logging();

    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    let app = Arc::new(Mutex::new(App::new(sync_io_tx, config.clone()).await));
    let cloned_app = Arc::clone(&app);
    std::thread::spawn(move || {
        let mut net = Network::new(&app, &config);
        start_tokio(sync_io_rx, &mut net);
    });
    // The UI must run in the "main" thread
    start_ui(&cloned_app).await?;
    Ok(())
}

#[tokio::main]
async fn start_tokio<'a>(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}
async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    // Terminal initialization
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let is_first_render = true;

    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();
    'main: loop {
        let mut app = app.lock().await;
        app.init().await;
        // Get the size of the screen on each loop to account for resize event
        if let Ok(size) = terminal.backend().size() {
            // Reset the help menu is the terminal was resized
            if is_first_render || app.size != size {
                app.size = size;
            }
        };

        let current_route = app.get_current_route();
        terminal.draw(|mut f| match current_route.active_block {
            _ => {
                ui::draw_main_layout(&mut f, &app);
            }
        })?;

        // if current_route.active_block == ActiveBlock::Input {
        //     terminal.show_cursor()?;
        // } else {
        //     terminal.hide_cursor()?;
        // }
        terminal.hide_cursor()?;

        let cursor_offset = 2;
        terminal.backend_mut().execute(MoveTo(cursor_offset, cursor_offset))?;

        let timeout = tick_rate.checked_sub(last_tick.elapsed()).unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        if app.get_current_route().active_block == ActiveBlock::Empty {
                            break 'main;
                        }
                    }
                    KeyCode::Char('c') => {
                        if key.modifiers == KeyModifiers::CONTROL {
                            break 'main;
                        }
                    }
                    _ => {}
                }
                if app.get_current_route().active_block == ActiveBlock::Dialog {
                    handlers::input_handler(key, &mut app);
                } else {
                    handlers::handle_app(key, &mut app)
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            //app.on_tick();
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    Ok(())
}
