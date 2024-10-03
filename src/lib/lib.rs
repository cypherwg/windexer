pub mod rpc;
pub mod compression;
pub mod indexer;
pub mod storage;
pub mod api;
pub mod wasm;
pub mod metrics;
pub mod utils;

use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
    pub solana: SolanaSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub redis_url: String,
    pub scylla_nodes: Vec<String>,
    pub clickhouse_url: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug, Deserialize)]
pub struct SolanaSettings {
    pub rpc_url: String,
    pub ws_url: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name(&format!("config/environments/{}", run_mode)).required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?;

        s.try_deserialize()
    }
}