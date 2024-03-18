use std::fmt;

use mgwconf_network::model::InnerEntityTrait;

pub mod configuration;

pub enum TabId {
    CERTIFICATE = 0,
    SAG = 1,
    BUSINESSAPPLICATION = 2,
    PROFILE = 3,
    APIPROXY = 4,
    FORWARDPROXY = 5,
}

impl From<usize> for TabId {
    fn from(v: usize) -> Self {
        match v {
            x if x == TabId::CERTIFICATE as usize => TabId::CERTIFICATE,
            x if x == TabId::SAG as usize => TabId::SAG,
            x if x == TabId::BUSINESSAPPLICATION as usize => TabId::BUSINESSAPPLICATION,
            x if x == TabId::PROFILE as usize => TabId::PROFILE,
            x if x == TabId::APIPROXY as usize => TabId::APIPROXY,
            x if x == TabId::FORWARDPROXY as usize => TabId::FORWARDPROXY,
            _ => TabId::CERTIFICATE,
        }
    }
}

impl fmt::Display for TabId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TabId::CERTIFICATE => write!(f, "Certificates"),
            TabId::SAG => write!(f, "Sags"),
            TabId::BUSINESSAPPLICATION => write!(f, "Business Applications"),
            TabId::PROFILE => write!(f, "Profiles"),
            TabId::APIPROXY => write!(f, "Api Proxy"),
            TabId::FORWARDPROXY => write!(f, "Forward Proxy"),
        }
    }
}

pub trait State {
    fn next(&mut self);
    fn back(&mut self);
    fn reload(&mut self);

    fn select_entity(&mut self);
    fn selected_entity(&self) -> Option<Box<&dyn InnerEntityTrait>>;

    fn update_pan_len(&mut self);

    fn current_pan(&self) -> usize;
    fn current_tab(&self) -> usize;
    fn current_selected(&self) -> TabId;
    fn select_current(&mut self);
    fn unselect_current(&mut self);

    fn is_tab_selected(&self) -> bool;

    fn waiting_for_load(&self) -> bool;
    fn wait_for_load(&mut self);
}
