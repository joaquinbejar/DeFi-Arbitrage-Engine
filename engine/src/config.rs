//! Configuration management for the arbitrage engine

use crate::error::{ArbitrageError, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration settings
    pub server: ServerConfig,
    /// Database connection configuration
    pub database: DatabaseConfig,
    /// Redis cache configuration
    pub redis: RedisConfig,
    /// Solana blockchain configuration
    pub solana: SolanaConfig,
    /// Arbitrage strategy configuration
    pub arbitrage: ArbitrageConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Server host address
    pub host: String,
    /// Server port number
    pub port: u16,
    /// Number of worker threads
    pub workers: usize,
}

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Database connection URL
    pub url: String,
    /// Maximum number of database connections
    pub max_connections: u32,
    /// Minimum number of database connections
    pub min_connections: u32,
}

/// Redis cache configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    /// Redis connection URL
    pub url: String,
    /// Maximum number of Redis connections
    pub max_connections: u32,
}

/// Solana blockchain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaConfig {
    /// Solana RPC endpoint URL
    pub rpc_url: String,
    /// Solana WebSocket endpoint URL
    pub ws_url: String,
    /// Transaction commitment level
    pub commitment: String,
}

/// Arbitrage strategy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageConfig {
    /// Minimum profit threshold to execute trades
    pub min_profit_threshold: f64,
    /// Maximum acceptable slippage percentage
    pub max_slippage: f64,
    /// Maximum acceptable price impact percentage
    pub max_price_impact: f64,
    /// Trade execution timeout in seconds
    pub execution_timeout: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (debug, info, warn, error)
    pub level: String,
    /// Log output format (json, plain)
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
