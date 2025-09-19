//! DeFi Arbitrage Engine
//!
//! High-performance Solana DeFi arbitrage engine for detecting and executing
//! profitable arbitrage opportunities across multiple DEXs.

pub mod config;
pub mod database;
pub mod dex;
pub mod engine;
pub mod error;
pub mod geyser;
pub mod metrics;
pub mod models;
pub mod server;
pub mod strategy;
pub mod utils;

pub use config::Config;
pub use engine::ArbitrageEngine;
pub use error::{ArbitrageError, Result};

/// Initialize the arbitrage engine with default configuration
pub async fn init() -> Result<ArbitrageEngine> {
    let config = Config::from_env()?;
    ArbitrageEngine::new(config).await
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[allow(clippy::const_is_empty)]
    async fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
