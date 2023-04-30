use std::{error::Error, net::IpAddr, path::PathBuf};

use log::{debug, info, warn, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use clap::Parser;

#[derive(Parser, Debug, Clone)]
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
}

#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
    loaded: bool,
    pub remote_ip: IpAddr,
    pub remote_port: u16,
    pub root_ca_path: String,

    pub tick_rate: u64,
}

impl Config {
    pub fn init(args: &Args) -> Result<Config, Box<dyn Error>> {
        let remote_ip = IpAddr::from([127, 0, 0, 1]);
        let config = Config {
            debug: args.debug,
            loaded: false,
            remote_ip,
            remote_port: 9003,
            root_ca_path: "/home/mita/sources/mgwconf/CA.pem".to_owned(),
            tick_rate: 250,
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

        let mut log_path = PathBuf::from("logs");
        if !log_path.is_dir() {
            warn!("logs directory doesn't exist");
        }
        log_path.push("mgwc.log");
        if !log_path.is_file() {
            warn!("logs file doesn't exist and will be created");
        }
        let in_file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S.%f)} {l} {f} {L} - {m}{n}")))
            .build(log_path)
            .unwrap();

        let filter = if self.debug { LevelFilter::Debug } else { LevelFilter::Info };

        let config = log4rs::Config::builder()
            .appender(Appender::builder().filter(Box::new(ThresholdFilter::new(filter))).build("in_file", Box::new(in_file)))
            .build(Root::builder().appender("in_file").build(filter))
            .unwrap();

        log4rs::init_config(config).unwrap();
        info!("Config has been loadded successfully");
        self.loaded = true;
    }
}
