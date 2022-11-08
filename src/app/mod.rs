use anyhow::Result;
use std::sync::mpsc::Sender;
use tui::layout::Rect;

pub mod vault;

use crate::{config::Config, network::IoEvent, ui::configuration::CONFIGURATION_USER_TAB};

use self::vault::{SecretType, SecretsVault};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ActiveBlock {
    Empty,
    Error,
    HelpMenu,
    Home,
    Tab,
    TabSelected,
    Dialog,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum RouteId {
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

pub struct ConfigurationState {
    tab_id: usize,
    tab_len: usize,
    selected_tab: Option<usize>,

    in_panel: bool,
}

impl Default for ConfigurationState {
    fn default() -> Self {
        ConfigurationState {
            tab_id: 0,
            tab_len: CONFIGURATION_USER_TAB.len(),
            selected_tab: None,
            in_panel: false,
        }
    }
}

impl ConfigurationState {
    pub fn next(&mut self) {
        if self.tab_id + 1 >= self.tab_len {
            self.tab_id = 0;
        } else {
            self.tab_id += 1;
        }
    }

    pub fn back(&mut self) {
        if self.tab_id <= 0 {
            self.tab_id = self.tab_len - 1;
        } else {
            self.tab_id -= 1;
        }
    }

    pub fn current(&self) -> usize {
        self.tab_id
    }

    pub fn current_selected(&self) -> usize {
        if let Some(t) = self.selected_tab {
            return t;
        }
        0
    }

    pub fn select_current(&mut self) {
        self.selected_tab = Some(self.tab_id);
        self.in_panel = true;
    }

    pub fn unselect_current(&mut self) {
        self.selected_tab = None;
        self.in_panel = false;
    }

    pub fn is_tab_selected(&self) -> bool {
        self.selected_tab.is_some()
    }
}

pub struct App {
    pub size: Rect,
    navigation_stack: Vec<Route>,
    pub selected_configuration_tab: Option<usize>,
    pub vault: Option<SecretsVault>,
    pub input: String,
    pub connectivity_test: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub config: Option<Config>,
    pub configuration_state: ConfigurationState,

    initialized: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            size: Rect::default(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_configuration_tab: None,
            vault: None,
            input: String::new(),
            connectivity_test: false,
            io_tx: None,
            config: None,
            initialized: false,
            configuration_state: ConfigurationState::default(),
        }
    }
}

impl App {
    pub async fn new(io_tx: Sender<IoEvent>, config: Config, vault_key: &str) -> App {
        App {
            io_tx: Some(io_tx),
            config: Some(config),
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
}
