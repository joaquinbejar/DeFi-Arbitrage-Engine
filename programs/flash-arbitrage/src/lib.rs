//! Flash Arbitrage Program
//!
//! This Anchor program enables atomic arbitrage execution with flash loans,
//! allowing traders to exploit price differences across DEXes without
//! requiring upfront capital.

#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(missing_docs)]

use anchor_lang::prelude::*;

pub mod func;
pub mod models;

use func::*;
use models::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Flash Arbitrage Program Module
///
/// Contains all the instruction handlers for flash arbitrage operations
#[program]
pub mod flash_arbitrage {
    use super::*;

    /// Initialize the flash arbitrage program
    pub fn initialize(ctx: Context<Initialize>, fee_rate: u16, max_slippage: u16) -> Result<()> {
        initialize_handler(ctx, fee_rate, max_slippage)
    }

    /// Execute flash arbitrage across multiple DEXes
    pub fn execute_flash_arbitrage(
        ctx: Context<ExecuteFlashArbitrage>,
        flash_loan_amount: u64,
        min_profit: u64,
        routes: Vec<ArbitrageRoute>,
    ) -> Result<()> {
        execute_flash_arbitrage_handler(ctx, flash_loan_amount, min_profit, routes)
    }

    /// Update program configuration (admin only)
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        fee_rate: Option<u16>,
        max_slippage: Option<u16>,
        is_paused: Option<bool>,
    ) -> Result<()> {
        update_config_handler(ctx, fee_rate, max_slippage, is_paused)
    }

    /// Withdraw collected fees (admin only)
    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
        withdraw_fees_handler(ctx, amount)
    }

    /// Emergency pause (admin only)
    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        emergency_pause_handler(ctx)
    }
}
