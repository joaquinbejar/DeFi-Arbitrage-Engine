//! Data models and structures for the Cross-DEX Router Program

use anchor_lang::prelude::*;

// Account structs
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + RouterConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, RouterConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(dex_info: DexInfo)]
pub struct RegisterDex<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + DexRegistry::INIT_SPACE,
        seeds = [b"dex", dex_info.name.as_bytes()],
        bump
    )]
    pub dex_registry: Account<'info, DexRegistry>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ExecuteOptimalRoute<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, RouterConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + RouteState::INIT_SPACE,
        seeds = [b"route", user.key().as_ref()],
        bump
    )]
    pub route_state: Account<'info, RouteState>,

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub input_token_account: Account<'info, anchor_spl::token::TokenAccount>,

    #[account(mut)]
    pub output_token_account: Account<'info, anchor_spl::token::TokenAccount>,

    #[account(mut)]
    pub fee_account: Account<'info, anchor_spl::token::TokenAccount>,

    pub token_program: Program<'info, anchor_spl::token::Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetRouteQuote<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, RouterConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + QuoteState::INIT_SPACE,
        seeds = [b"quote", user.key().as_ref()],
        bump
    )]
    pub quote_state: Account<'info, QuoteState>,

    #[account(mut)]
    pub user: Signer<'info>,

    /// CHECK: Input token mint
    pub input_token_mint: AccountInfo<'info>,

    /// CHECK: Output token mint
    pub output_token_mint: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateDexMetrics<'info> {
    #[account(
        mut,
        seeds = [b"dex", dex_registry.dex_info.name.as_bytes()],
        bump
    )]
    pub dex_registry: Account<'info, DexRegistry>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, RouterConfig>,

    pub authority: Signer<'info>,
}

// Data structs
/// Configuration for the cross-DEX router
#[account]
#[derive(InitSpace)]
pub struct RouterConfig {
    pub authority: Pubkey,
    pub max_hops: u8,
    pub default_slippage: u16,
    pub routing_fee: u16,
    pub total_routes_executed: u64,
    pub total_volume: u64,
    pub total_fees_collected: u64,
    pub is_active: bool,
    pub bump: u8,
}

/// Registry of supported DEX protocols
#[account]
#[derive(InitSpace)]
pub struct DexRegistry {
    pub dex_info: DexInfo,
    pub total_volume: u64,
    pub total_swaps: u32,
    pub success_rate: u16,     // In basis points
    pub average_slippage: u16, // In basis points
    pub last_updated: i64,
}

/// State of an active route execution
#[account]
#[derive(InitSpace)]
pub struct RouteState {
    pub user: Pubkey,
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub max_slippage: u16,
    pub route: OptimalRoute,
    pub status: RouteStatus,
    pub start_time: i64,
    pub end_time: i64,
    pub actual_output: u64,
    pub total_fees: u64,
    pub actual_slippage: u16,
}

/// State of a route quote
#[account]
#[derive(InitSpace)]
pub struct QuoteState {
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub route: OptimalRoute,
    pub expected_output: u64,
    pub estimated_fees: u64,
    pub price_impact: u16,
    pub timestamp: i64,
}

/// Information about a DEX protocol
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DexInfo {
    #[max_len(32)]
    pub name: String,
    pub program_id: Pubkey,
    pub fee_rate: u16, // In basis points
    pub is_active: bool,
    pub supported_tokens: u32, // Number of supported tokens
}

/// Optimal route for token swapping
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct OptimalRoute {
    #[max_len(10)]
    pub hops: Vec<RouteHop>,
    pub expected_output: u64,
    pub total_fees: u64,
    pub price_impact: u16,
}

/// Single hop in a multi-hop route
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct RouteHop {
    #[max_len(32)]
    pub dex: String,
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub expected_output: u64,
    pub fees: u64,
    pub price_impact: u16,
    pub pool_address: Pubkey,
}

/// Status of route execution
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum RouteStatus {
    Finding,
    Executing,
    Completed,
    Failed,
}

/// Result of route execution
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RouteExecutionResult {
    pub output_amount: u64,
    pub total_fees: u64,
    pub hops_executed: u8,
}

/// Result of a single swap operation
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapResult {
    pub input_amount: u64,
    pub output_amount: u64,
    pub fees: u64,
}

// Events
/// Event emitted when router is initialized
#[event]
pub struct RouterInitialized {
    pub authority: Pubkey,
    pub max_hops: u8,
    pub default_slippage: u16,
    pub routing_fee: u16,
}

/// Event emitted when a DEX is registered
#[event]
pub struct DexRegistered {
    pub dex_name: String,
    pub program_id: Pubkey,
    pub fee_rate: u16,
}

/// Event emitted when a route is executed
#[event]
pub struct RouteExecuted {
    pub user: Pubkey,
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub hops_count: u8,
    pub total_fees: u64,
    pub execution_time: i64,
}

/// Event emitted when a hop is executed
#[event]
pub struct HopExecuted {
    pub hop_index: u8,
    pub dex: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub fees: u64,
}

/// Event emitted when a quote is generated
#[event]
pub struct QuoteGenerated {
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub expected_output: u64,
    pub hops_count: u8,
    pub estimated_fees: u64,
    pub price_impact: u16,
}

/// Event emitted when DEX metrics are updated
#[event]
pub struct DexMetricsUpdated {
    pub dex_name: String,
    pub total_volume: u64,
    pub total_swaps: u32,
    pub success_rate: u16,
    pub average_slippage: u16,
}

/// Event emitted when configuration is updated
#[event]
pub struct ConfigUpdated {
    pub authority: Pubkey,
    pub max_hops: u8,
    pub default_slippage: u16,
    pub routing_fee: u16,
    pub is_active: bool,
}

// Errors
/// Errors that can occur in the cross-DEX router
#[error_code]
pub enum CrossDexError {
    #[msg("Invalid DEX name")]
    InvalidDexName,

    #[msg("Fee rate too high")]
    FeeTooHigh,

    #[msg("DEX is not active")]
    DexNotActive,

    #[msg("Router is inactive")]
    RouterInactive,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Slippage too high")]
    SlippageTooHigh,

    #[msg("Route not profitable")]
    RouteNotProfitable,

    #[msg("No route found")]
    NoRouteFound,

    #[msg("Too many hops")]
    TooManyHops,

    #[msg("Unsupported DEX")]
    UnsupportedDex,

    #[msg("Route execution failed")]
    RouteExecutionFailed,

    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,

    #[msg("Price impact too high")]
    PriceImpactTooHigh,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Invalid configuration")]
    InvalidConfig,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
