use anyhow::Result;
use config::{Config, ConfigError, Environment, File};
use getset::Getters;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, sync::Arc};
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct HttpServer {
    pub port: usize,
}

#[derive(Debug, Deserialize)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub rpc_url: Vec<String>,
}

#[derive(Debug, Deserialize, Getters)]
#[allow(unused)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub chains: Vec<ChainConfig>,
}

impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        // we have a default file is required and an environment specific config file is optional
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        info!("Using RUN_MODE = {:?}", run_mode);

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("src/config/default"))
            // Add in the current environment file
            // Default to 'devnet' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("src/config/{}", run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("src/config/local").required(false))
            .build()?;

        s.try_deserialize::<AppConfig>()
    }
}

lazy_static! {
    static ref GLOBAL_CONFIG: Arc<AppConfig> = {
        let app_config = match AppConfig::new() {
            Ok(app_config) => app_config,
            Err(e) => panic!("{}", e),
        };
        Arc::new(app_config)
    };
}

pub fn get_global_config() -> Arc<AppConfig> {
    Arc::clone(&GLOBAL_CONFIG)
}
