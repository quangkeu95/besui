use std::{collections::HashMap, fs::File, io::BufReader, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ChainId(u64);

impl TryFrom<u64> for ChainId {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Ok(ChainId(value))
    }
}

impl TryFrom<String> for ChainId {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.parse::<u64>() {
            Ok(usigned_value) => return Ok(usigned_value.try_into()?),
            Err(_) => return Err(format!("Error parsing chain id from string: {}", value)),
        };
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChainConfig {
    pub name: String,
    pub chain: String,
    #[serde(alias = "chainId")]
    pub chain_id: ChainId,
    #[serde(default)]
    pub rpc: Vec<String>,
}

pub fn read_chain_config_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<ChainConfig>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut cfg: Vec<ChainConfig> = serde_json::from_reader(reader)?;

    cfg = cfg
        .into_iter()
        .map(|mut item| {
            let filtered_rpc = item
                .rpc
                .into_iter()
                .filter_map(|rpc| {
                    if !rpc.contains("INFURA_API_KEY") {
                        return Some(rpc);
                    }
                    None
                })
                .collect::<Vec<String>>();

            item.rpc = filtered_rpc;
            item
        })
        .collect::<Vec<ChainConfig>>();

    Ok(cfg)
}

pub fn read_custom_rpc_from_file<P: AsRef<Path>>(
    path: P,
) -> anyhow::Result<HashMap<ChainId, Vec<String>>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let cfg: HashMap<String, Vec<String>> = serde_json::from_reader(reader)?;
    let converted_cfg = cfg
        .into_iter()
        .filter_map(|(key, value)| {
            if value.is_empty() {
                return None;
            }
            let chain_id: ChainId = key.try_into().unwrap();
            Some((chain_id, value))
        })
        .collect::<HashMap<ChainId, Vec<String>>>();
    Ok(converted_cfg)
}

#[cfg(test)]
mod test {
    use super::*;
    use claims::assert_ok;

    #[test]
    fn test_read_chain_config_from_file() {
        let project_root = assert_ok!(besui_utils::file::get_project_root());
        let chains_config = assert_ok!(read_chain_config_from_file(
            project_root.join("configuration").join("chains.json")
        ));
        assert_ne!(chains_config.len(), 0);
    }
}
