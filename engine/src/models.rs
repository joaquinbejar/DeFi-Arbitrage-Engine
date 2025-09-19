//! Data models for the arbitrage engine

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Arbitrage opportunity model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub id: String,
    pub token_a: String,
    pub token_b: String,
    pub dex_a: String,
    pub dex_b: String,
    pub profit_percentage: Decimal,
    pub profit_amount: Decimal,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

/// Trade execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    pub id: String,
    pub opportunity_id: String,
    pub status: TradeStatus,
    pub executed_at: DateTime<Utc>,
    pub actual_profit: Option<Decimal>,
    pub gas_cost: Decimal,
    pub net_profit: Option<Decimal>,
}

/// Trade execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeStatus {
    Pending,
    Executing,
    Success,
    Failed,
    Cancelled,
}
