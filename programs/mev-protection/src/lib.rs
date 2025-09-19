//! MEV Protection Program
//!
//! This Anchor program provides protection against MEV attacks,
//! particularly sandwich attacks, by implementing various defensive mechanisms.

#![allow(unexpected_cfgs)]
#![allow(deprecated)]
#![allow(missing_docs)]

use anchor_lang::prelude::*;

pub mod func;
pub mod models;

use func::*;
use models::*;

declare_id!("MevProtect111111111111111111111111111111111");

/// MEV Protection Program Module
///
/// Contains all the instruction handlers for MEV protection operations
#[program]
pub mod mev_protection {
    use super::*;

    /// Initialize the MEV protection program
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_handler(ctx)
    }

    /// Create a protected transaction with MEV safeguards
    pub fn create_protected_transaction(
        ctx: Context<CreateProtectedTransaction>,
        transaction_data: TransactionParams,
        protection_level: ProtectionLevel,
    ) -> Result<()> {
        create_protected_transaction_handler(ctx, transaction_data, protection_level)
    }

    /// Execute a protected transaction with MEV checks
    pub fn execute_protected_transaction(ctx: Context<ExecuteProtectedTransaction>) -> Result<()> {
        execute_protected_transaction_handler(ctx)
    }

    /// Cancel a protected transaction
    pub fn cancel_protected_transaction(ctx: Context<CancelProtectedTransaction>) -> Result<()> {
        cancel_protected_transaction_handler(ctx)
    }

    /// Update MEV protection configuration (admin only)
    pub fn update_protection_config(
        ctx: Context<UpdateProtectionConfig>,
        max_price_impact: Option<u16>,
        min_time_delay: Option<i64>,
        max_slippage_protection: Option<u16>,
        is_active: Option<bool>,
    ) -> Result<()> {
        update_protection_config_handler(
            ctx,
            max_price_impact,
            min_time_delay,
            max_slippage_protection,
            is_active,
        )
    }

    /// Report MEV attack attempt
    pub fn report_mev_attack(
        ctx: Context<ReportMevAttack>,
        attack_details: AttackDetails,
    ) -> Result<()> {
        report_mev_attack_handler(ctx, attack_details)
    }
}
