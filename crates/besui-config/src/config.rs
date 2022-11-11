use anyhow::{self, Context};
use config::{Config, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, sync::Arc};
use tracing::info;

#[derive(Debug, Deserialize, Clone)]
pub struct HttpServer {
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String, // env
    pub logging: DatabaseLogging,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseLogging {
    pub level: String, // env
    pub enabled: bool, // env
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn parse() -> anyhow::Result<AppConfig> {
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
            .add_source(
                Environment::with_prefix("BESUI")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?;

        s.try_deserialize::<AppConfig>()
            .context("error parsing configuration")
    }
}

lazy_static! {
    static ref GLOBAL_CONFIG: Arc<AppConfig> = {
        let app_config = match AppConfig::parse() {
            Ok(app_config) => app_config,
            Err(e) => panic!("{}", e),
        };
        Arc::new(app_config)
    };
}

pub fn get_global_config() -> Arc<AppConfig> {
    Arc::clone(&GLOBAL_CONFIG)
}
