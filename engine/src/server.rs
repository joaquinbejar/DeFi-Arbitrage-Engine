//! HTTP server module

use crate::error::Result;
use axum::{routing::get, Router};
use std::net::SocketAddr;

/// HTTP server for the arbitrage engine
#[derive(Debug)]
pub struct Server {
    addr: SocketAddr,
}

impl Server {
    /// Create a new server
    pub fn new(host: &str, port: u16) -> Result<Self> {
        let addr = format!("{}:{}", host, port).parse().map_err(|e| {
            crate::error::ArbitrageError::config(format!("Invalid server address: {}", e))
        })?;

        Ok(Self { addr })
    }

    /// Start the server
    pub async fn start(&self) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_check))
            .route("/status", get(status));

        let listener = tokio::net::TcpListener::bind(&self.addr)
            .await
            .map_err(|e| {
                crate::error::ArbitrageError::internal(format!("Failed to bind server: {}", e))
            })?;

        axum::serve(listener, app)
            .await
            .map_err(|e| crate::error::ArbitrageError::internal(format!("Server error: {}", e)))?;

        Ok(())
    }
}

/// Health check endpoint
async fn health_check() -> &'static str {
    "OK"
}

/// Status endpoint
async fn status() -> &'static str {
    "Running"
}
