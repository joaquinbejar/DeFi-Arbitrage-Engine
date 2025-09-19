//! Cross-DEX Router Program
//!
//! This program enables optimal routing of token swaps across multiple DEXes
//! on Solana, including Raydium, Orca, Meteora, and Jupiter.
//!
//! Features:
//! - Multi-hop routing for optimal prices
//! - Cross-DEX arbitrage detection
//! - Slippage protection
//! - Fee optimization
//! - Real-time price discovery

#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(missing_docs)]

use anchor_lang::prelude::*;

pub mod func;
pub mod models;

use func::*;
use models::*;

declare_id!("CrossDEXRouter11111111111111111111111111111");

#[program]
pub mod cross_dex_router {
    use super::*;

    /// Initialize the cross-DEX router program
    pub fn initialize(
        ctx: Context<Initialize>,
        max_hops: u8,
        default_slippage: u16,
        routing_fee: u16,
    ) -> Result<()> {
        initialize_router(ctx, max_hops, default_slippage, routing_fee)
    }

    /// Register a new DEX for routing
    pub fn register_dex(ctx: Context<RegisterDex>, dex_info: DexInfo) -> Result<()> {
        register_dex_handler(ctx, dex_info)
    }

    /// Find and execute optimal route across DEXes
    pub fn execute_optimal_route(
        ctx: Context<ExecuteOptimalRoute>,
        input_amount: u64,
        min_output_amount: u64,
        max_slippage: Option<u16>,
        preferred_dexes: Vec<String>,
    ) -> Result<()> {
        execute_optimal_route_handler(
            ctx,
            input_amount,
            min_output_amount,
            max_slippage,
            preferred_dexes,
        )
    }

    /// Get quote for optimal route without executing
    pub fn get_route_quote(
        ctx: Context<GetRouteQuote>,
        input_amount: u64,
        preferred_dexes: Vec<String>,
    ) -> Result<()> {
        get_route_quote_handler(ctx, input_amount, preferred_dexes)
    }

    /// Update DEX performance metrics
    pub fn update_dex_metrics(
        ctx: Context<UpdateDexMetrics>,
        volume: u64,
        swap_count: u32,
        success_rate: u16,
        average_slippage: u16,
    ) -> Result<()> {
        update_dex_metrics_handler(ctx, volume, swap_count, success_rate, average_slippage)
    }

    /// Update router configuration (admin only)
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        max_hops: Option<u8>,
        default_slippage: Option<u16>,
        routing_fee: Option<u16>,
        is_active: Option<bool>,
    ) -> Result<()> {
        update_config_handler(ctx, max_hops, default_slippage, routing_fee, is_active)
    }
}
