use anyhow::{anyhow, bail, Context};
use config::{Config, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
    sync::Arc,
};
use tracing::{error, info};

use crate::{
    chains::{read_chain_config_from_file, ChainConfig},
    read_custom_rpc_from_file, ChainId,
};

#[derive(Debug, Deserialize, Clone)]
pub struct HttpServer {
    pub port: u16,
    pub log_level: String,
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
    pub log_level: String,
    pub http_server: HttpServer,
    pub database: DatabaseConfig,
    #[serde(skip_deserializing)]
    pub chains_config: HashMap<ChainId, ChainConfig>,
}

impl AppConfig {
    pub fn parse() -> anyhow::Result<AppConfig> {
        let base_path =
            std::env::current_dir().context("Failed to determine the current directory")?;
        let configuration_dir = base_path.join("configuration");

        // Detect the running environment.
        // Default to `local` if unspecified.
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "local".into())
            .try_into()
            .context("Failed to parse APP_ENVIRONMENT.")?;
        let environment_filename = format!("{}.toml", environment.as_str());

        info!("Using APP_ENVIRONMENT = {:?}", environment);

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::from(configuration_dir.join("base.toml")))
            .add_source(File::from(configuration_dir.join(&environment_filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .prefix_separator("_")
                    .separator("_"),
            )
            .build()?;

        let mut app_config = match s.try_deserialize::<AppConfig>() {
            Ok(setting) => setting,
            Err(e) => bail!("Error parsing configuration: {:?}", e),
        };

        app_config.chains_config = parse_chain_config(&configuration_dir)?;

        Ok(app_config)
        // .context("Error parsing configuration")
    }
}

fn parse_chain_config(
    configuration_dir: &PathBuf,
) -> anyhow::Result<HashMap<ChainId, ChainConfig>> {
    let chains_config = read_chain_config_from_file(configuration_dir.join("chains.json"))?;
    let custom_rpc = read_custom_rpc_from_file(configuration_dir.join("custom_rpc.json"))?;

    let result = chains_config
        .into_iter()
        .map(|mut item| {
            let key = item.chain_id.clone();
            if let Some(custom_rpc_url) = custom_rpc.get(&key) {
                item.rpc = item
                    .rpc
                    .into_iter()
                    .chain(custom_rpc_url.into_iter().map(|item| item.clone()))
                    .collect();
            }

            (key, item)
        })
        .collect::<HashMap<ChainId, ChainConfig>>();
    Ok(result)
}

pub static APP_CONFIG_INSTANCE: Lazy<AppConfig> = Lazy::new(|| {
    let app_config = match AppConfig::parse() {
        Ok(app_config) => app_config,
        Err(e) => panic!("App configuration error: {}", e),
    };
    app_config
});

#[derive(Debug)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(anyhow!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

// lazy_static! {
//     static ref GLOBAL_CONFIG: Arc<AppConfig> = {
//         let app_config = match AppConfig::parse() {
//             Ok(app_config) => app_config,
//             Err(e) => panic!("app configuration error: {}", e),
//         };
//         Arc::new(app_config)
//     };
// }

// pub fn get_global_config() -> &'static AppConfig {
//     APP_CONFIG_INSTANCE::get_or_init(|| {
//         let app_config = match AppConfig::parse() {
//             Ok(app_config) => app_config,
//             Err(e) => panic!("app configuration error: {}", e),
//         };
//         app_config
//     })
//     // Arc::clone(&GLOBAL_CONFIG)
// }
