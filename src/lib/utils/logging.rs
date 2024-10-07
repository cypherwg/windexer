use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use crate::utils::error::Result;

pub fn init_logger() -> Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    Ok(())
}