//! Geyser client module for the arbitrage engine

use crate::error::Result;

/// Geyser gRPC client for streaming Solana data
/// Temporarily disabled due to proto compilation issues
#[derive(Debug, Default)]
pub struct GeyserClient {
    // TODO: Add gRPC client connection when proto issues are resolved
}

impl GeyserClient {
    /// Create a new Geyser client
    pub async fn new(_endpoint: &str) -> Result<Self> {
        // TODO: Initialize gRPC connection
        Ok(Self {})
    }
}
