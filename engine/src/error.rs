//! Error types for the arbitrage engine

use thiserror::Error;

/// Result type alias for the arbitrage engine
pub type Result<T> = std::result::Result<T, ArbitrageError>;

/// Main error type for the arbitrage engine
#[derive(Error, Debug)]
pub enum ArbitrageError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Database operation errors
    #[error("Database error: {0}")]
    Database(String), // Temporarily changed from sqlx::Error due to version conflicts

    /// Redis connection and operation errors
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    /// Solana RPC client errors
    #[error("Solana client error: {0}")]
    SolanaClient(#[from] solana_client::client_error::ClientError),

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP request errors
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// WebSocket connection and communication errors
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Arbitrage calculation and analysis errors
    #[error("Arbitrage calculation error: {0}")]
    Calculation(String),

    /// DEX integration and interaction errors
    #[error("DEX integration error: {0}")]
    DexIntegration(String),

    /// Blockchain transaction errors
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Insufficient liquidity for trade execution
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    /// Price impact exceeds acceptable threshold
    #[error("Price impact too high: {0}%")]
    PriceImpactTooHigh(f64),

    /// Slippage tolerance exceeded during execution
    #[error("Slippage tolerance exceeded")]
    SlippageExceeded,

    /// Internal system errors
    #[error("Internal error: {0}")]
    Internal(String),
}

impl ArbitrageError {
    /// Creates a new configuration error
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    /// Creates a new WebSocket error
    pub fn websocket(msg: impl Into<String>) -> Self {
        Self::WebSocket(msg.into())
    }

    /// Creates a new calculation error
    pub fn calculation(msg: impl Into<String>) -> Self {
        Self::Calculation(msg.into())
    }

    /// Creates a new DEX integration error
    pub fn dex_integration(msg: impl Into<String>) -> Self {
        Self::DexIntegration(msg.into())
    }

    /// Creates a new transaction error
    pub fn transaction(msg: impl Into<String>) -> Self {
        Self::Transaction(msg.into())
    }

    /// Creates a new internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
