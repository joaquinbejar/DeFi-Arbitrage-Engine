//! Configuration management for the arbitrage engine

use crate::error::{ArbitrageError, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub solana: SolanaConfig,
    pub arbitrage: ArbitrageConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub commitment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageConfig {
    pub min_profit_threshold: f64,
    pub max_slippage: f64,
    pub max_price_impact: f64,
    pub execution_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: num_cpus::get(),
            },
            database: DatabaseConfig {
                url: "postgresql://localhost/arbitrage".to_string(),
                max_connections: 10,
                min_connections: 1,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                max_connections: 10,
            },
            solana: SolanaConfig {
                rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
                ws_url: "wss://api.mainnet-beta.solana.com".to_string(),
                commitment: "confirmed".to_string(),
            },
            arbitrage: ArbitrageConfig {
                min_profit_threshold: 0.01, // 1%
                max_slippage: 0.005,        // 0.5%
                max_price_impact: 0.02,     // 2%
                execution_timeout: 30,      // 30 seconds
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        // Server configuration
        if let Ok(host) = env::var("SERVER_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = env::var("SERVER_PORT") {
            config.server.port = port
                .parse()
                .map_err(|_| ArbitrageError::config("Invalid SERVER_PORT value"))?;
        }

        // Database configuration
        if let Ok(url) = env::var("DATABASE_URL") {
            config.database.url = url;
        }

        // Redis configuration
        if let Ok(url) = env::var("REDIS_URL") {
            config.redis.url = url;
        }

        // Solana configuration
        if let Ok(url) = env::var("SOLANA_RPC_URL") {
            config.solana.rpc_url = url;
        }
        if let Ok(url) = env::var("SOLANA_WS_URL") {
            config.solana.ws_url = url;
        }

        // Logging configuration
        if let Ok(level) = env::var("LOG_LEVEL") {
            config.logging.level = level;
        }

        Ok(config)
    }

    /// Load configuration from a TOML file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| ArbitrageError::config(format!("Failed to read config file: {}", e)))?;

        toml::from_str(&content)
            .map_err(|e| ArbitrageError::config(format!("Failed to parse config file: {}", e)))
    }
}
