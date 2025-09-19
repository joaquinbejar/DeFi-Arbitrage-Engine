//! Error types for the arbitrage engine

use thiserror::Error;

/// Result type alias for the arbitrage engine
pub type Result<T> = std::result::Result<T, ArbitrageError>;

/// Main error type for the arbitrage engine
#[derive(Error, Debug)]
pub enum ArbitrageError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Database error: {0}")]
    Database(String), // Temporarily changed from sqlx::Error due to version conflicts

    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Solana client error: {0}")]
    SolanaClient(#[from] solana_client::client_error::ClientError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Arbitrage calculation error: {0}")]
    Calculation(String),

    #[error("DEX integration error: {0}")]
    DexIntegration(String),

    #[error("Transaction error: {0}")]
    Transaction(String),

    #[error("Insufficient liquidity")]
    InsufficientLiquidity,

    #[error("Price impact too high: {0}%")]
    PriceImpactTooHigh(f64),

    #[error("Slippage tolerance exceeded")]
    SlippageExceeded,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl ArbitrageError {
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    pub fn websocket(msg: impl Into<String>) -> Self {
        Self::WebSocket(msg.into())
    }

    pub fn calculation(msg: impl Into<String>) -> Self {
        Self::Calculation(msg.into())
    }

    pub fn dex_integration(msg: impl Into<String>) -> Self {
        Self::DexIntegration(msg.into())
    }

    pub fn transaction(msg: impl Into<String>) -> Self {
        Self::Transaction(msg.into())
    }

    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}
