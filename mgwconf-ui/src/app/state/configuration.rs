use mgwconf_network::model::certificate::Entities as CertificateEntities;
use mgwconf_network::model::profile::Entities as Profiles;
use mgwconf_network::model::sag::Entities as SagEntities;
use mgwconf_network::model::CollectionEntityTrait;
use mgwconf_network::model::{business_application::Entities as BusinessApplications, InnerEntityTrait};

use super::{State, TabId};

#[derive(Debug)]
pub struct ConfigurationState {
    tab_id: usize,
    tab_len: usize,
    selected_tab: Option<usize>,
    pan_id: usize,
    pan_len: usize,

    waiting: bool,

    in_panel: bool,
    pub sags: SagEntities,
    pub certificates: CertificateEntities,
    pub business_applications: BusinessApplications,
    pub profiles: Profiles,

    current_entity: Option<Box<dyn InnerEntityTrait>>,
}

impl Default for ConfigurationState {
    fn default() -> Self {
        ConfigurationState {
            tab_id: 0,
            tab_len: 5,
            selected_tab: None,
            in_panel: false,
            sags: SagEntities::default(),
            certificates: CertificateEntities::default(),
            business_applications: BusinessApplications::default(),
            profiles: Profiles::default(),
            pan_id: 0,
            pan_len: 0,
            waiting: false,

            current_entity: None,
        }
    }
}

impl State for ConfigurationState {
    fn next(&mut self) {
        if self.in_panel && self.pan_len > 0 {
            if self.pan_id + 1 >= self.pan_len {
                self.pan_id = 0;
            } else {
                self.pan_id += 1;
            }
            self.select_entity();
        } else if self.tab_id + 1 >= self.tab_len {
            self.tab_id = 0;
        } else {
            self.tab_id += 1;
        }
    }

    fn back(&mut self) {
        if self.in_panel && self.pan_len > 0 {
            if self.pan_id == 0 {
                self.pan_id = self.pan_len - 1;
            } else {
                self.pan_id -= 1;
            }
            self.select_entity();
        } else if self.tab_id == 0 {
            self.tab_id = self.tab_len - 1;
        } else {
            self.tab_id -= 1;
        }
    }

    fn reload(&mut self) {
        self.select_entity();
    }

    fn select_entity(&mut self) {
        if !self.in_panel || self.pan_id == 0 {
            self.current_entity = None;
            return;
        }
        let entity: Option<Box<dyn InnerEntityTrait>> = match self.current_selected() {
            TabId::CERTIFICATE => self.certificates.get(self.pan_id - 1),
            TabId::SAG => self.sags.get(self.pan_id - 1),
            TabId::BUSINESSAPPLICATION => self.business_applications.get(self.pan_id - 1),
            TabId::PROFILE => self.sags.get(self.pan_id - 1),
            TabId::APIPROXY => self.sags.get(self.pan_id - 1),
        };
        if let Some(e) = entity {
            self.current_entity = Some(e);
        }
    }

    fn selected_entity(&self) -> Option<Box<&dyn InnerEntityTrait>> {
        self.current_entity.as_ref()?;
        let entity = self.current_entity.as_ref().unwrap();
        Some(Box::new(&**entity))
    }

    fn current_tab(&self) -> usize {
        self.tab_id
    }

    fn current_pan(&self) -> usize {
        self.pan_id
    }

    fn current_selected(&self) -> TabId {
        TabId::from(self.selected_tab.unwrap_or(0))
    }

    fn select_current(&mut self) {
        self.pan_id = 0;
        self.selected_tab = Some(self.tab_id);
        self.in_panel = true;
        self.update_pan_len();
    }

    fn update_pan_len(&mut self) {
        match self.tab_id {
            0 => self.pan_len = self.certificates.0.len() + 1,
            1 => self.pan_len = self.sags.0.len() + 1,
            2 => self.pan_len = self.business_applications.0.len() + 1,
            _ => {}
        }
        if self.pan_len > 0 {
            self.waiting = false;
        }
    }

    fn unselect_current(&mut self) {
        self.selected_tab = None;
        self.in_panel = false;
        self.current_entity = None;
    }

    fn is_tab_selected(&self) -> bool {
        self.selected_tab.is_some()
    }

    fn wait_for_load(&mut self) {
        self.waiting = true;
    }
    fn waiting_for_load(&self) -> bool {
        self.waiting
    }
}
