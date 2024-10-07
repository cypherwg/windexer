use serde::Deserialize;
use std::fs;
use crate::utils::error::Result;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub solana_rpc_url: String,
    pub api_port: u16,
    pub metrics_port: u16,
    pub log_level: String,
    pub wasm_dir: String,
}

pub fn load_config() -> Result<Config> {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/default.toml".to_string());
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}