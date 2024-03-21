use log::{debug, info};
use mgwconf_network::AppConfig;
use std::{any::Any, error::Error, net::{IpAddr, SocketAddr, ToSocketAddrs}, path::PathBuf};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter};

use clap::Parser;

use crate::playbook::Playbook;

#[derive(Parser, Debug, Default, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Debug mode
    #[clap(short = 'd', long, action = clap::ArgAction::SetTrue, default_value = "false")]
    debug: bool,
    /// create secret
    #[clap(long = "create_secret", action = clap::ArgAction::SetTrue, default_value = "false")]
    pub create_secret: bool,
    /// pass vault key
    #[clap(short = 'k')]
    pub vault_key: Option<String>,
    #[clap(short = 'c', long = "command", required = false)]
    pub commands: Option<Vec<String>>,
    #[clap(short = 'p', long = "playbook", required = false)]
    pub playbook: Option<String>,
    #[clap(long = "remote_addr")]
    pub remote_addr: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
    loaded: bool,
    pub remote_addr: SocketAddr,
    pub root_ca_path: String,

    pub tick_rate: u64,
    pub commands: Vec<String>,
    pub playbook: Option<Playbook>,
}

impl Config {
    pub fn init(args: &Args) -> Result<Config, Box<dyn Error>> {
        let remote_addr = if let Some(ip) = &args.remote_addr {
            ip.to_socket_addrs().expect("Unable to resolve domain").next()
        } else {
            "127.0.0.1:9003".parse().unwrap()
        };
        let config = Config {
            commands: args.commands.clone().unwrap_or_default(),
            debug: args.debug,
            loaded: false,
            remote_addr,
            root_ca_path: "/home/mita/sources/mgwconf/CA.pem".to_owned(),
            tick_rate: 250,
            playbook: args.playbook.to_owned().map(|v| v.into()),
        };
        info!("Config has been loadded successfully");
        debug!("Config values {:?}", config);
        Ok(config)
    }

    pub fn playbook(&self) -> &Option<Playbook> {
        &self.playbook
    }

    #[allow(dead_code)]
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn init_logging(&mut self) {
        if self.loaded {
            return;
        }

        let mut log_path = PathBuf::from("./logs");
        if !log_path.is_dir() {
            println!("logs directory doesn't exist");
        }
        log_path.push(env!("CARGO_PKG_NAME"));
        let file_appender = tracing_appender::rolling::daily(log_path.parent().unwrap(), log_path.file_name().unwrap());
        let appender_format = if self.debug {
            format!("{}=debug,{}=debug", env!("CARGO_PKG_NAME"), "mgwc")
        } else {
            format!("{}=info,{}=info", env!("CARGO_PKG_NAME"), "mgwc")
        };
        let filter = EnvFilter::builder().parse(appender_format).unwrap();
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::Layer::new().with_writer(file_appender).with_line_number(true).with_ansi(false))
            .with(tracing_subscriber::fmt::layer())
            .init();
        info!("Config has been loadded successfully");
        self.loaded = true;
    }
}

impl AppConfig for Config {
    fn remote_ip(&self) -> IpAddr {
        self.remote_addr.ip()
    }

    fn remote_port(&self) -> u16 {
        self.remote_addr.port()
    }

    fn root_ca_path(&self) -> String {
        self.root_ca_path.to_owned()
    }
    fn tickrate(&self) -> u64 {
        self.tick_rate
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
