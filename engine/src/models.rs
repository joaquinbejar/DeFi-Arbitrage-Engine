//! Data models for the arbitrage engine

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Arbitrage opportunity model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    /// Unique identifier for the opportunity
    pub id: String,
    /// First token in the arbitrage pair
    pub token_a: String,
    /// Second token in the arbitrage pair
    pub token_b: String,
    /// First DEX in the arbitrage route
    pub dex_a: String,
    /// Second DEX in the arbitrage route
    pub dex_b: String,
    /// Profit percentage for this opportunity
    pub profit_percentage: Decimal,
    /// Absolute profit amount in base currency
    pub profit_amount: Decimal,
    /// Timestamp when the opportunity was created
    pub created_at: DateTime<Utc>,
    /// Timestamp when the opportunity expires
    pub expires_at: DateTime<Utc>,
}

/// Trade execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeResult {
    /// Unique identifier for the trade result
    pub id: String,
    /// Reference to the arbitrage opportunity
    pub opportunity_id: String,
    /// Current status of the trade execution
    pub status: TradeStatus,
    /// Timestamp when the trade was executed
    pub executed_at: DateTime<Utc>,
    /// Actual profit realized from the trade
    pub actual_profit: Option<Decimal>,
    /// Gas cost incurred during execution
    pub gas_cost: Decimal,
    /// Net profit after deducting gas costs
    pub net_profit: Option<Decimal>,
}

/// Trade execution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeStatus {
    /// Trade is waiting to be executed
    Pending,
    /// Trade is currently being executed
    Executing,
    /// Trade executed successfully
    Success,
    /// Trade execution failed
    Failed,
    /// Trade was cancelled before execution
    Cancelled,
}
