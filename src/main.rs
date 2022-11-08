use log::{error, info};
use network::{IoEvent, Network};
use std::{
    io::{self, stdin, stdout, Stdout, Write},
    panic,
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

    let vault_key = ask_master_key();
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();
    let mut app = App::new(sync_io_tx, config.clone(), &vault_key).await;
    app.vault.as_mut().expect("Vault not initialized correctly").read_all_secrets();

    let app = Arc::new(Mutex::new(app));

    let cloned_app = Arc::clone(&app);
    let orig = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        error!("{}", panic_info);
        close_application(None).unwrap();
        orig(panic_info);
        std::process::exit(1);
    }));
    std::thread::spawn(move || {
        let mut net = Network::new(&app, &config).expect("Network Error");
        start_tokio(sync_io_rx, &mut net);
    });
    if args.ui {
        start_ui(&cloned_app).await.unwrap();
    } else if args.create_secret {
        let mut app = cloned_app.lock().await;
        app.ask_secrets()?;
        println!("{:?}", app.vault);
    }
    Ok(())
}

#[tokio::main]
async fn start_tokio(io_rx: std::sync::mpsc::Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        match network.handle_network_event(io_event).await {
            Ok(r) => {
                info!("{:#?}", r)
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }
}

fn ask_master_key() -> String {
    let mut vault_key = String::new();
    println!("Pleaser enter MASTER VAULT KEY");
    let _ = stdout().flush();
    stdin().read_line(&mut vault_key).expect("Did not enter a correct string");
    vault_key.pop();
    print!("\x1B[2J\x1B[1;1H");
    vault_key
}

fn close_application(terminal: Option<&mut Terminal<CrosstermBackend<Stdout>>>) -> Result<()> {
    if let Some(term) = terminal {
        term.show_cursor()?;
    }
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

async fn start_ui(app: &Arc<Mutex<App>>) -> Result<(), anyhow::Error> {
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
        app.init().await.unwrap();
        // Get the size of the screen on each loop to account for resize event
        if let Ok(size) = terminal.backend().size() {
            if is_first_render || app.size != size {
                app.size = size;
            }
        };

        terminal.draw(|f| {
            ui::draw_main_layout(f, &app);
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
                        if app.get_current_route().active_block == ActiveBlock::Empty || app.get_current_route().active_block == ActiveBlock::Tab {
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
                    handlers::handle_input(key, &mut app);
                } else {
                    handlers::handle_app(key, &mut app)
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    close_application(Some(&mut terminal))
}