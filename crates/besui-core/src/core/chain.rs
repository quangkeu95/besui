use besui_config::{ChainConfig, APP_CONFIG_INSTANCE};
pub struct ChainMonitor {
    pub chains_config: Vec<ChainConfig>,
}

impl ChainMonitor {
    pub fn new() -> Self {
        // let chains_config = APP_CONFIG_INSTANCE.chains_config.clone();
        ChainMonitor {
            chains_config: Vec::new(),
        }
    }
}
