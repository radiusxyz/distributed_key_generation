mod config_option;
mod config_path;

use std::{fs, path::PathBuf};

pub use config_option::*;
pub use config_path::*;
use radius_sdk::signature::ChainType;
use serde::{Deserialize, Serialize};

use crate::{
    error::Error,
    types::{Address, SigningKey},
};

pub const DEFAULT_HOME_PATH: &str = ".radius";
pub const DATABASE_DIR_NAME: &str = "database";
pub const CONFIG_FILE_NAME: &str = "Config.toml";
pub const SIGNING_KEY: &str = "signing_key";

const DEFAULT_EXTERNAL_RPC_URL: &str = "http://127.0.0.1:3000";
const DEFAULT_INTERNAL_RPC_URL: &str = "http://127.0.0.1:4000";
const DEFAULT_CLUSTER_RPC_URL: &str = "http://127.0.0.1:5000";

const DEFAULT_RADIUS_FOUNDATION_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const DEFAULT_CHAIN_TYPE: &str = "Ethereum";

const DEFAULT_PARTIAL_KEY_GENERATION_CYCLE: u64 = 5;
const DEFAULT_PARTIAL_KEY_AGGREGATION_CYCLE: u64 = 4;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    path: PathBuf,

    external_rpc_url: String,
    internal_rpc_url: String,
    cluster_rpc_url: String,
    seed_cluster_rpc_url: Option<String>,

    signing_key: SigningKey,

    radius_foundation_address: Address,
    chain_type: ChainType,

    partial_key_generation_cycle: u64,
    partial_key_aggregation_cycle: u64,
}

impl Config {
    pub fn load(config_option: &mut ConfigOption) -> Result<Self, Error> {
        let config_path = match config_option.path.as_mut() {
            Some(config_path) => config_path.clone(),
            None => {
                let config_path: PathBuf = ConfigPath::default().as_ref().into();
                config_option.path = Some(config_path.clone());
                config_path
            }
        };

        // Read config file
        let config_file_path = config_path.join(CONFIG_FILE_NAME);
        let config_string =
            fs::read_to_string(config_file_path).map_err(Error::LoadConfigOption)?;

        // Parse String to TOML String
        let config_file: ConfigOption =
            toml::from_str(&config_string).map_err(Error::ParseTomlString)?;

        // Merge configs from CLI input
        let merged_config_option = config_file.merge(config_option);

        // Read signing key
        let signing_key_path = config_path.join(SIGNING_KEY);
        let signing_key = SigningKey::from(fs::read_to_string(signing_key_path).unwrap());

        Ok(Config {
            path: config_path,
            external_rpc_url: merged_config_option.external_rpc_url.unwrap(),
            internal_rpc_url: merged_config_option.internal_rpc_url.unwrap(),
            cluster_rpc_url: merged_config_option.cluster_rpc_url.unwrap(),
            seed_cluster_rpc_url: merged_config_option.seed_cluster_rpc_url.clone(),
            signing_key,
            radius_foundation_address: merged_config_option
                .radius_foundation_address
                .unwrap()
                .into(),
            // TODO: stompesi
            // chain_type: merged_config_option.chain_type.unwrap().into(),
            chain_type: ChainType::Ethereum,

            partial_key_generation_cycle: merged_config_option
                .partial_key_generation_cycle
                .unwrap(),
            partial_key_aggregation_cycle: merged_config_option
                .partial_key_aggregation_cycle
                .unwrap(),
        })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn database_path(&self) -> PathBuf {
        self.path.join(DATABASE_DIR_NAME)
    }

    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    pub fn radius_foundation_address(&self) -> &Address {
        &self.radius_foundation_address
    }

    pub fn chain_type(&self) -> &ChainType {
        &self.chain_type
    }

    pub fn address(&self) -> Address {
        self.signing_key().get_address()
    }

    pub fn external_rpc_url(&self) -> &String {
        &self.external_rpc_url
    }

    pub fn internal_rpc_url(&self) -> &String {
        &self.internal_rpc_url
    }

    pub fn partial_key_generation_cycle(&self) -> u64 {
        self.partial_key_generation_cycle
    }

    pub fn partial_key_aggregation_cycle(&self) -> u64 {
        self.partial_key_aggregation_cycle
    }

    pub fn cluster_rpc_url(&self) -> &String {
        &self.cluster_rpc_url
    }

    pub fn seed_cluster_rpc_url(&self) -> &Option<String> {
        &self.seed_cluster_rpc_url
    }
}
