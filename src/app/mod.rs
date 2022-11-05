use std::sync::mpsc::Sender;
use tui::layout::Rect;

use crate::{config::Config, network::IoEvent};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    Empty,
    Error,
    HelpMenu,
    Home,
    BasicView,
    Dialog,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    Error,
    Home,
    Configuration,
    SecretsDialog,
}

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Empty,
    hovered_block: ActiveBlock::Empty,
};

#[derive(Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

#[derive(Copy, Clone)]
pub enum SecretType {
    Configuration,
    Monitoring,
    Management,
    Encrypt,
}

impl Default for SecretType {
    fn default() -> Self {
        SecretType::Configuration
    }
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

#[derive(Default)]
pub struct SecretsVault {
    pub configuration: Option<String>,
    pub monitoring: Option<String>,
    pub management: Option<String>,
    pub encrypt: Option<String>,

    pub current_secret: SecretType,
}

pub struct App {
    pub size: Rect,
    navigation_stack: Vec<Route>,
    pub selected_configuration_tab: Option<usize>,
    pub vault: SecretsVault,
    pub input: String,
    pub connectivity_test: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub config: Option<Config>,
}

impl Default for App {
    fn default() -> Self {
        App {
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_configuration_tab: None,
            vault: SecretsVault::default(),
            input: String::new(),
            connectivity_test: false,
            io_tx: None,
            config: None,
        }
    }
}

impl App {
    pub async fn new(io_tx: Sender<IoEvent>, config: Config) -> App {
        App {
            io_tx: Some(io_tx),
            config: Some(config),
            ..App::default()
        }
    }

    pub async fn init(&self) {
        self.io_tx.as_ref().unwrap().send(IoEvent::Ping).unwrap();
    }

    pub fn set_secret(&mut self, value: Option<String>) {
        match self.vault.current_secret {
            SecretType::Configuration => self.vault.configuration = value,
            SecretType::Monitoring => self.vault.monitoring = value,
            SecretType::Management => self.vault.management = value,
            SecretType::Encrypt => self.vault.encrypt = value,
        }
        self.validate_secret(self.vault.current_secret);
    }

    pub fn validate_secret(&self, _stype: SecretType) {}

    pub fn get_current_route(&self) -> &Route {
        // if for some reason there is no route return the default
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn pop_navigation_stack(&mut self) {
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
}
