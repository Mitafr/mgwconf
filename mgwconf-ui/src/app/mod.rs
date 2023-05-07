use anyhow::Result;
use async_trait::async_trait;
use mgwconf_network::{model::CollectionEntityTrait, AppConfig, AppTrait, IoEvent};
use mgwconf_vault::{SecretType, SecretsVault};
use std::{
    io::{stdin, stdout, Write},
    sync::mpsc::Sender,
};

pub mod state;

use crate::config::Config;

use self::state::{configuration::ConfigurationState, State};

#[async_trait]
pub trait UiAppTrait<C: AppConfig>: AppTrait<C> {
    fn update_on_tick(&mut self);
    fn get_current_route(&self) -> &Route;
    fn get_current_route_mut(&mut self) -> &mut Route;
    fn pop_navigation_stack(&mut self);
    fn set_current_route_state(&mut self, active_block: Option<ActiveBlock>, hovered_block: Option<ActiveBlock>);
    fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock);
    fn reset_navigation_stack(&mut self);
    fn reset_selected_states(&mut self);

    fn get_configuration_state(&self) -> &ConfigurationState;
    fn get_configuration_state_mut(&mut self) -> &mut ConfigurationState;
    fn get_user_input(&self) -> &str;
    fn get_user_input_mut(&mut self) -> &mut String;

    fn get_force_exit(&self) -> bool;
    fn force_exit(&mut self);
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

#[derive(Debug)]
pub struct UiApp {
    pub config: Option<Config>,
    pub configuration_state: ConfigurationState,
    pub connectivity_test: bool,
    io_tx: Option<Sender<IoEvent>>,
    pub input: String,
    navigation_stack: Vec<Route>,
    pub selected_configuration_tab: Option<usize>,
    pub vault: Option<SecretsVault>,

    initialized: bool,
    pub force_exit: bool,
}

impl Default for UiApp {
    fn default() -> Self {
        UiApp {
            config: None,
            configuration_state: ConfigurationState::default(),
            connectivity_test: false,
            io_tx: None,
            input: String::new(),
            navigation_stack: vec![DEFAULT_ROUTE],
            selected_configuration_tab: None,
            vault: None,
            initialized: false,
            force_exit: false,
        }
    }
}

impl UiApp {
    pub async fn new(io_tx: Sender<IoEvent>, config: Config, vault_key: &str) -> UiApp {
        UiApp {
            config: Some(config),
            io_tx: Some(io_tx),
            vault: Some(SecretsVault::new(vault_key)),
            ..Default::default()
        }
    }
}

#[async_trait]
impl<C: AppConfig> AppTrait<C> for UiApp {
    async fn init(&mut self) -> Result<()> {
        if self.initialized {
            return Ok(());
        }
        self.io_tx.as_ref().unwrap().send(IoEvent::Ping)?;
        self.initialized = true;
        Ok(())
    }

    fn dispatch(&mut self, io_event: IoEvent) -> Result<()> {
        self.io_tx.as_ref().unwrap().send(io_event)?;
        Ok(())
    }

    fn ask_secrets(&mut self) -> Result<()> {
        let mut secret = String::new();
        for s in SecretType::iterator() {
            <UiApp as AppTrait<C>>::ask_secret(self, &mut secret, *s);
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

    fn is_connected(&self) -> bool {
        self.connectivity_test
    }

    fn set_connected(&mut self, connected: bool) {
        self.connectivity_test = connected;
    }

    fn vault(&self) -> Option<&SecretsVault> {
        self.vault.as_ref()
    }

    fn config(&self) -> Box<dyn AppConfig> {
        Box::new(self.config.as_ref().unwrap().clone())
    }

    fn handle_network_response<T>(&mut self, event: IoEvent, res: T)
    where
        T: CollectionEntityTrait,
    {
        match event {
            IoEvent::Ping => todo!(),
            IoEvent::GetAllBusinessApplications => todo!(),
            IoEvent::GetAllCertificates => self.configuration_state.certificates = res.as_any().downcast_ref::<mgwconf_network::model::certificate::Entities>().unwrap().clone(),
            IoEvent::GetAllSags => self.configuration_state.sags = res.as_any().downcast_ref::<mgwconf_network::model::sag::Entities>().unwrap().clone(),
            IoEvent::PostBusinessApplication => todo!(),
            IoEvent::PostCertificate => todo!(),
            IoEvent::PostSag => todo!(),
            IoEvent::DeleteBusinessApplication => todo!(),
            IoEvent::DeleteCertificate => todo!(),
            IoEvent::DeleteSag => todo!(),
        }
    }
}

#[async_trait]
impl<C: AppConfig> UiAppTrait<C> for UiApp {
    fn get_current_route(&self) -> &Route {
        self.navigation_stack.last().unwrap_or(&DEFAULT_ROUTE)
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    fn pop_navigation_stack(&mut self) {
        log::info!("{}", self.navigation_stack.len());
        log::info!("{:#?}", self.navigation_stack);
        if self.navigation_stack.len() <= 1 {
            self.force_exit = true;
        }
        self.navigation_stack.pop();
    }

    fn set_current_route_state(&mut self, active_block: Option<ActiveBlock>, hovered_block: Option<ActiveBlock>) {
        let mut current_route = <UiApp as UiAppTrait<C>>::get_current_route_mut(self);
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }

    fn push_navigation_stack(&mut self, next_route_id: RouteId, next_active_block: ActiveBlock) {
        if !self.navigation_stack.last().map(|last_route| last_route.id == next_route_id).unwrap_or(false) {
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
}
