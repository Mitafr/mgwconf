use log::{debug, info};
use mgwconf_network::AppConfig;
use std::{
    any::Any,
    error::Error,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter};

use clap::{ArgMatches, Parser};

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
    /// pass ca
    #[clap(long = "ca")]
    pub root_ca_path: Option<String>,
    #[clap(long = "remote_addr")]
    pub remote_addr: Option<String>,
}

impl From<ArgMatches> for Args {
    fn from(m: ArgMatches) -> Self {
        Args {
            create_secret: false,
            vault_key: Some(m.get_one::<String>("k").unwrap().clone()),
            debug: m.get_flag("debug"),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
    loaded: bool,
    pub remote_addr: SocketAddr,
    pub root_ca_path: String,

    pub tick_rate: u64,
}

impl Config {
    pub fn init(args: &Args) -> Result<Config, Box<dyn Error>> {
        let remote_addr = if let Some(ip) = &args.remote_addr {
            ip.parse::<SocketAddr>().unwrap_or("127.0.0.1:9003".parse().unwrap())
        } else {
            "127.0.0.1:9003".parse().unwrap()
        };
        let config = Config {
            debug: args.debug,
            loaded: false,
            remote_addr,
            root_ca_path: args.root_ca_path.clone().unwrap_or_else(|| "./CA.pem".to_owned()),
            tick_rate: 160,
        };
        info!("Config has been loadded successfully");
        debug!("Config values {:?}", config);
        Ok(config)
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
            format!("debug,{}=debug,{}=debug", env!("CARGO_PKG_NAME"), "mgwc_ui")
        } else {
            format!("info,{}=info,{}=info", env!("CARGO_PKG_NAME"), "mgwc_ui")
        };
        let filter = EnvFilter::builder().parse(appender_format).unwrap();
        tracing_subscriber::registry()
            .with(filter)
            .with(tracing_subscriber::fmt::Layer::new().with_writer(file_appender).with_line_number(true).with_ansi(false))
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
