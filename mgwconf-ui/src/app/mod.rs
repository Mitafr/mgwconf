use anyhow::{Error, Result};
use async_trait::async_trait;
use crossterm::{
    cursor::MoveTo,
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use mgwconf_network::{
    event::IoEvent,
    mgw_configuration::{apis::ResponseContent, models::*},
    AppConfig, AppTrait,
};
use mgwconf_vault::{SecretType, SecretsVault};
use ratatui::{backend::CrosstermBackend, Terminal};
use serde::{Deserialize, Serialize};
use std::{io::stdout, sync::Arc};
use tokio::sync::mpsc::Sender;
use tokio::sync::{Mutex, Notify};

pub mod state;

use crate::{
    config::Config,
    event::{Event, Events, Key},
    handler::{handle_app, handle_input},
    ui::draw_main_layout,
};

use self::state::{configuration::ConfigurationState, State};

#[async_trait]
pub trait UiAppTrait<C: AppConfig>: AppTrait<C> {
    fn update_on_tick(&mut self);
    fn get_current_route(&self) -> &Route;
    fn get_current_route_mut(&mut self) -> &mut Route;
    fn pop_navigation_stack(&mut self);
    fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    );
    fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock);
    fn reset_navigation_stack(&mut self);
    fn reset_selected_states(&mut self);

    fn get_configuration_state(&self) -> &ConfigurationState;
    fn get_configuration_state_mut(&mut self) -> &mut ConfigurationState;
    fn get_user_input(&self) -> &str;
    fn get_user_input_mut(&mut self) -> &mut String;

    fn get_force_exit(&self) -> bool;
    fn force_exit(&mut self);

    fn pop_error(&self) -> Option<&Error>;
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ActiveBlock {
    Empty,
    Error,
    HelpMenu,
    Home,
    Tab,
    TabSelected,
    Dialog,
    Detailed,
    Editing,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Configuration,
}

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Home,
    hovered_block: ActiveBlock::Empty,
};

#[derive(Debug, Copy, Clone)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

#[derive(Debug)]
pub struct UiApp {
    pub config: Option<Config>,
    pub configuration_state: ConfigurationState,
    pub connectivity_test: bool,
    io_tx: Sender<IoEvent>,
    pub input: String,
    navigation_stack: Vec<Route>,
    error_queue: Vec<Error>,
    pub selected_configuration_tab: Option<usize>,
    pub vault: Option<SecretsVault>,

    initialized: bool,
    pub force_exit: bool,
}

impl UiApp {
    pub async fn new(io_tx: Sender<IoEvent>, mut config: Config, vault_key: &str) -> UiApp {
        config.init_logging();
        let vault = match SecretsVault::new(vault_key) {
            Ok(v) => v,
            Err(e) => {
                log::error!("Can't decode secret vault : {}", e);
                panic!("Can't decode secret vault : {}", e);
            }
        };
        UiApp {
            config: Some(config),
            io_tx,
            vault: Some(vault),
            configuration_state: ConfigurationState::default(),
            connectivity_test: false,
            input: String::new(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_configuration_tab: None,
            initialized: false,
            force_exit: false,
            error_queue: Vec::new(),
        }
    }
}

#[async_trait]
impl AppTrait<Config> for UiApp {
    async fn init(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        log::info!("Initilizing UiApp...");
        self.io_tx.send(IoEvent::Ping).await?;
        log::info!("Ping sent...");
        self.initialized = true;
        Ok(())
    }

    async fn dispatch(&self, io_event: IoEvent) -> Result<(), anyhow::Error> {
        self.io_tx.send(io_event).await?;
        Ok(())
    }

    fn ask_secrets(master: &str) -> Result<()> {
        let mut secret = String::new();
        for s in SecretType::iterator() {
            <UiApp as AppTrait<Config>>::ask_secret(master, &mut secret, *s);
        }
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    #[cfg(not(feature = "store"))]
    #[allow(unused_variables)]
    fn ask_secret(master: &str, s: &mut String, stype: SecretType) {
        panic!("To create a vault store you must enable store features");
    }

    #[cfg(feature = "store")]
    fn ask_secret(master: &str, s: &mut String, stype: SecretType) {
        use std::io::{stdin, stdout, Write};
        println!("Pleaser enter {} API KEY", stype);
        let _ = stdout().flush();
        stdin()
            .read_line(s)
            .expect("Did not enter a correct string");
        s.pop();
        SecretsVault::new(master)
            .unwrap()
            .create_secret(stype, s.to_owned())
            .unwrap();
        s.clear()
    }

    fn is_connected(&self) -> bool {
        self.connectivity_test
    }

    fn set_connected(&mut self, connected: bool) {
        self.connectivity_test = connected;
    }

    fn vault(&self) -> Option<&SecretsVault> {
        self.vault.as_ref()
    }

    fn config(&self) -> &Config {
        self.config.as_ref().unwrap()
    }

    fn handle_network_response<'a, T: Deserialize<'a> + Serialize>(
        &mut self,
        event: IoEvent,
        res: ResponseContent<T>,
    ) {
        match event {
            IoEvent::Ping => todo!(),
            IoEvent::GetAllProfiles => {
                if let Some(entity) = res.entity {
                    self.configuration_state.profiles =
                        serde_json::from_value::<Vec<ApplicationProfileEntity>>(
                            serde_json::to_value(entity).unwrap(),
                        )
                        .unwrap();
                }
            }
            IoEvent::GetAllBusinessApplications => {
                if let Some(entity) = res.entity {
                    self.configuration_state.business_applications =
                        serde_json::from_value::<Vec<BusinessApplicationEntity>>(
                            serde_json::to_value(entity).unwrap(),
                        )
                        .unwrap();
                }
            }
            IoEvent::GetAllCertificates => {
                if let Some(entity) = res.entity {
                    self.configuration_state.certificates =
                        serde_json::from_value::<Vec<CertificateEntity>>(
                            serde_json::to_value(entity).unwrap(),
                        )
                        .unwrap();
                }
            }
            IoEvent::GetAllSags => {
                if let Some(entity) = res.entity {
                    self.configuration_state.sags = serde_json::from_value::<Vec<SagEntity>>(
                        serde_json::to_value(entity).unwrap(),
                    )
                    .unwrap();
                }
            }
            IoEvent::GetAllApiGatewayInfoEntity => {
                if let Some(entity) = res.entity {
                    self.configuration_state.apiproxy =
                        serde_json::from_value::<Vec<ApiGatewayInfoEntity>>(
                            serde_json::to_value(entity).unwrap(),
                        )
                        .unwrap();
                }
            }
            IoEvent::GetAllForwardProxyEntity => {
                if let Some(entity) = res.entity {
                    self.configuration_state.forwardproxy =
                        serde_json::from_value::<Vec<ForwardProxyEntity>>(
                            serde_json::to_value(entity).unwrap(),
                        )
                        .unwrap();
                }
            }
            IoEvent::PostCertificate(_) => {}
            IoEvent::PostSag(_) => {}
            //IoEvent::PostBusinessApplication => todo!(),
            //IoEvent::PostCertificate => todo!(),
            //IoEvent::PostSag => todo!(),
            //IoEvent::PostProfile => todo!(),
            //IoEvent::DeleteBusinessApplication(_e) => todo!(),
            //IoEvent::DeleteCertificate(_e) => todo!(),
            //IoEvent::DeleteSag(_e) => todo!(),
            _ => todo!(),
        }
    }

    fn handle_network_error(&mut self, error: Error) {
        log::error!("Handling this error : {}", error);
        self.error_queue.push(error);
        if <UiApp as UiAppTrait<Config>>::get_current_route(self).id != RouteId::Home {
            <UiApp as UiAppTrait<Config>>::set_current_route_state(
                self,
                Some(ActiveBlock::Error),
                None,
            );
        }
    }

    async fn run(
        app: Arc<Mutex<UiApp>>,
        _notifier: Option<Arc<Notify>>,
    ) -> Result<(), anyhow::Error> {
        use std::time::{Duration, Instant};

        use mgwconf_network::AppTrait;

        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        enable_raw_mode()?;

        let backend = CrosstermBackend::new(stdout);

        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor()?;

        let tickrate = <UiApp as AppTrait<Config>>::config(&*app.lock().await).tickrate();

        let tick_rate = Duration::from_millis(tickrate);
        let mut events = Events::new(tickrate);
        let mut last_tick = Instant::now();

        'main: loop {
            let mut app = app.lock().await;
            <UiApp as AppTrait<Config>>::init(&mut app).await?;

            terminal.draw(|f| {
                draw_main_layout::<UiApp, Config>(f, &*app);
            })?;

            let cursor_offset = 2;
            terminal
                .backend_mut()
                .execute(MoveTo(cursor_offset, cursor_offset))?;

            let current_route = <UiApp as UiAppTrait<Config>>::get_current_route(&app);

            match events.next().unwrap()? {
                Event::Input(key) => {
                    if key == Key::Esc
                        && (current_route.active_block == ActiveBlock::Empty
                            || current_route.active_block == ActiveBlock::Tab)
                    {
                        break 'main;
                    }

                    let current_active_block = current_route.active_block;

                    if current_active_block == ActiveBlock::Dialog {
                        handle_input::<UiApp, Config>(key, &mut *app);
                    } else {
                        handle_app::<UiApp, Config>(key, &mut *app).await
                    }
                }
                Event::Tick => {
                    if <UiApp as UiAppTrait<Config>>::get_force_exit(&app) {
                        break 'main;
                    }
                    if <UiApp as UiAppTrait<Config>>::get_current_route(&app).active_block
                        == ActiveBlock::Error
                    {
                        std::thread::sleep(Duration::from_secs(2));
                        <UiApp as UiAppTrait<Config>>::reset_navigation_stack(&mut app);
                        <UiApp as UiAppTrait<Config>>::push_navigation_stack(
                            &mut app,
                            RouteId::Home,
                            ActiveBlock::Home,
                        );
                        *<UiApp as UiAppTrait<Config>>::get_configuration_state_mut(&mut app) =
                            ConfigurationState::default();
                    }
                    <UiApp as UiAppTrait<Config>>::update_on_tick(&mut app);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        terminal.show_cursor()?;
        disable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}

#[async_trait]
impl UiAppTrait<Config> for UiApp {
    fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    fn pop_navigation_stack(&mut self) {
        if self.navigation_stack.len() <= 1 {
            self.force_exit = true;
        }
        self.navigation_stack.pop();
    }

    fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let current_route = <UiApp as UiAppTrait<Config>>::get_current_route_mut(self);
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }

    fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
        if !self
            .navigation_stack
            .last()
            .map(|last_route| last_route.id == next_route_id)
            .unwrap_or(false)
        {
            self.navigation_stack.push(Route {
                id: next_route_id,
                active_block: next_active_block,
                hovered_block: next_active_block,
            });
        }
    }

    fn reset_navigation_stack(&mut self) {
        self.navigation_stack.clear();
        self.navigation_stack.push(DEFAULT_ROUTE);
    }

    fn reset_selected_states(&mut self) {
        self.configuration_state.unselect_current();
    }

    fn update_on_tick(&mut self) {
        if !self.configuration_state.waiting_for_load() {
            self.configuration_state.update_pan_len();
        }
    }

    fn get_configuration_state(&self) -> &ConfigurationState {
        &self.configuration_state
    }

    fn get_configuration_state_mut(&mut self) -> &mut ConfigurationState {
        &mut self.configuration_state
    }

    fn get_user_input(&self) -> &str {
        &self.input
    }

    fn get_user_input_mut(&mut self) -> &mut String {
        &mut self.input
    }

    fn force_exit(&mut self) {
        self.force_exit = true;
    }

    fn get_force_exit(&self) -> bool {
        self.force_exit
    }

    fn pop_error(&self) -> Option<&Error> {
        self.error_queue.last()
    }
}
