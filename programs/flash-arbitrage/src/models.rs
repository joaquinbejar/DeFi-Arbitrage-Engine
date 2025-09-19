//! Data models, structures, events, and errors for the Flash Arbitrage Program

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

// Account structs
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteFlashArbitrage<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = user,
        space = 8 + ArbitrageState::INIT_SPACE,
        seeds = [b"arbitrage", user.key().as_ref()],
        bump
    )]
    pub arbitrage_state: Account<'info, ArbitrageState>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_account: Account<'info, TokenAccount>,

    /// CHECK: Flash loan provider account
    pub flash_loan_provider: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

/// Account context for withdrawing collected fees
#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    /// Program configuration account
    pub config: Account<'info, Config>,

    /// Authority that can withdraw fees
    pub authority: Signer<'info>,

    #[account(mut)]
    /// Account holding the collected fees
    pub fee_account: Account<'info, TokenAccount>,

    #[account(mut)]
    /// Destination account to receive withdrawn fees
    pub destination_account: Account<'info, TokenAccount>,

    /// SPL Token program
    pub token_program: Program<'info, Token>,
}

/// Account context for emergency pause functionality
#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    /// Program configuration account to be paused
    pub config: Account<'info, Config>,

    /// Authority that can trigger emergency pause
    pub authority: Signer<'info>,
}

// Data structs
/// Program configuration and global state
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub fee_rate: u16,             // Fee rate in basis points
    pub max_slippage: u16,         // Max slippage in basis points
    pub total_volume: u64,         // Total volume processed
    pub total_fees_collected: u64, // Total fees collected
    pub is_paused: bool,           // Emergency pause flag
    pub bump: u8,                  // PDA bump
}

/// State of an arbitrage operation
#[account]
#[derive(InitSpace)]
pub struct ArbitrageState {
    pub user: Pubkey,
    pub flash_loan_amount: u64,
    pub min_profit: u64,
    #[max_len(5)]
    pub routes: Vec<ArbitrageRoute>,
    pub status: ArbitrageStatus,
    pub start_time: i64,
    pub end_time: i64,
    pub gross_profit: u64,
    pub net_profit: u64,
    pub fees_paid: u64,
}

/// Single route in an arbitrage strategy
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ArbitrageRoute {
    #[max_len(20)]
    pub dex: String, // DEX name (e.g., "raydium", "orca")
    pub token_in: Pubkey,     // Input token mint
    pub token_out: Pubkey,    // Output token mint
    pub expected_output: u64, // Expected output amount
    pub pool_address: Pubkey, // Pool/market address
}

/// Status of arbitrage execution
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum ArbitrageStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Result of executing a single route
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RouteResult {
    pub input_amount: u64,
    pub output_amount: u64,
    pub fees_paid: u64,
    pub slippage: u16,
}

/// Result of flash loan operation
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FlashLoanResult {
    pub amount_borrowed: u64,
    pub fee: u64,
    pub success: bool,
}

// Events
/// Event emitted when program is initialized
#[event]
pub struct ProgramInitialized {
    pub authority: Pubkey,
    pub fee_rate: u16,
    pub max_slippage: u16,
}

/// Event emitted when arbitrage is executed
#[event]
pub struct ArbitrageExecuted {
    pub user: Pubkey,
    pub flash_loan_amount: u64,
    pub gross_profit: u64,
    pub net_profit: u64,
    pub total_fees: u64,
    pub routes_count: u8,
}

/// Event emitted when a route is executed
#[event]
pub struct RouteExecuted {
    pub route_index: u8,
    pub dex: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub fees_paid: u64,
}

/// Event emitted when configuration is updated
#[event]
pub struct ConfigUpdated {
    pub authority: Pubkey,
    pub fee_rate: u16,
    pub max_slippage: u16,
    pub is_paused: bool,
}

/// Event emitted when fees are withdrawn
#[event]
pub struct FeesWithdrawn {
    pub authority: Pubkey,
    pub amount: u64,
    pub destination: Pubkey,
}

/// Event emitted when emergency pause is activated
#[event]
pub struct EmergencyPauseActivated {
    pub authority: Pubkey,
    pub timestamp: i64,
}

// Errors
/// Errors that can occur in the flash arbitrage program
#[error_code]
pub enum FlashArbitrageError {
    #[msg("Program is currently paused")]
    ProgramPaused,

    #[msg("No arbitrage routes provided")]
    EmptyRoutes,

    #[msg("Too many routes (maximum 5)")]
    TooManyRoutes,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Amount too large")]
    AmountTooLarge,

    #[msg("Insufficient funds for operation")]
    InsufficientFunds,

    #[msg("Profit below minimum threshold")]
    ProfitTooLow,

    #[msg("Slippage exceeds maximum allowed")]
    SlippageTooHigh,

    #[msg("Unsupported DEX")]
    UnsupportedDex,

    #[msg("Fee rate too high (maximum 10%)")]
    FeeTooHigh,

    #[msg("Flash loan failed")]
    FlashLoanFailed,

    #[msg("Route execution failed")]
    RouteExecutionFailed,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Invalid configuration")]
    InvalidConfig,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
