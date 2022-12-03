use anyhow::Result;
use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::Sender,
};
use tui::layout::Rect;

pub mod state;
pub mod vault;

use crate::{config::Config, network::IoEvent};

use self::{
    state::{configuration::ConfigurationState, State},
    vault::{SecretType, SecretsVault},
};

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
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RouteId {
    Home,
    Configuration,
}

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Home,
    hovered_block: ActiveBlock::Empty,
};

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

impl std::fmt::Display for SecretType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecretType::Configuration => write!(f, "Configuration"),
            SecretType::Monitoring => write!(f, "Monitoring"),
            SecretType::Management => write!(f, "Management"),
            SecretType::Encrypt => write!(f, "Encrypt"),
        }
    }
}
#[derive(Debug)]
pub struct App {
    pub config: Option<Config>,
    pub configuration_state: ConfigurationState,
    pub connectivity_test: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub input: String,
    navigation_stack: Vec<Route>,
    pub size: Rect,
    pub selected_configuration_tab: Option<usize>,
    pub vault: Option<SecretsVault>,

    initialized: bool,
    pub force_exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            config: None,
            configuration_state: ConfigurationState::default(),
            connectivity_test: false,
            io_tx: None,
            input: String::new(),
            navigation_stack: vec![DEFAULT_ROUTE],
            size: Rect::default(),
            selected_configuration_tab: None,
            vault: None,
            initialized: false,
            force_exit: false,
        }
    }
}

impl App {
    pub async fn new(io_tx: Sender<IoEvent>, config: Config, vault_key: &str) -> App {
        App {
            config: Some(config),
            io_tx: Some(io_tx),
            vault: Some(SecretsVault::new(vault_key)),
            ..Default::default()
        }
    }

    pub async fn init(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        self.io_tx.as_ref().unwrap().send(IoEvent::Ping)?;
        self.initialized = true;
        Ok(())
    }

    pub fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn pop_navigation_stack(&mut self) {
        log::info!("{}", self.navigation_stack.len());
        log::info!("{:#?}", self.navigation_stack);
        if self.navigation_stack.len() <= 1 {
            self.force_exit = true;
        }
        self.navigation_stack.pop();
    }

    pub fn set_current_route_state(&mut self, active_block: Option<ActiveBlock>, hovered_block: Option<ActiveBlock>) {
        let mut current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }

    pub fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
        if !self.navigation_stack.last().map(|last_route| last_route.id == next_route_id).unwrap_or(false) {
            self.navigation_stack.push(Route {
                id: next_route_id,
                active_block: next_active_block,
                hovered_block: next_active_block,
            });
        }
    }

    pub fn reset_navigation_stack(&mut self) {
        self.navigation_stack.clear();
        self.navigation_stack.push(DEFAULT_ROUTE);
    }

    pub fn reset_selected_states(&mut self) {
        self.configuration_state.unselect_current();
    }

    pub fn dispatch(&mut self, io_event: IoEvent) -> Result<()> {
        self.io_tx.as_ref().unwrap().send(io_event)?;
        Ok(())
    }

    pub fn ask_secrets(&mut self) -> Result<()> {
        let mut secret = String::new();
        for s in SecretType::iterator() {
            self.ask_secret(&mut secret, *s);
        }
        print!("\x1B[2J\x1B[1;1H");
        Ok(())
    }

    fn ask_secret(&mut self, s: &mut String, stype: SecretType) {
        println!("Pleaser enter {} API KEY", stype);
        let _ = stdout().flush();
        stdin().read_line(s).expect("Did not enter a correct string");
        s.pop();
        self.vault.as_ref().unwrap().create_secret(stype, s.to_owned());
        s.clear()
    }

    pub fn update_on_tick(&mut self) {
        if !self.configuration_state.waiting_for_load() {
            self.configuration_state.update_pan_len();
        }
    }
}
