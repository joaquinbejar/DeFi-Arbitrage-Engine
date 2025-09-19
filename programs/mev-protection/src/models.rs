//! Data models, structures, events, and errors for the MEV Protection Program

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

// Account structs
/// Account context for initializing the MEV protection system
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + ProtectionConfig::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, ProtectionConfig>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Account context for creating a new protected transaction
#[derive(Accounts)]
pub struct CreateProtectedTransaction<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProtectionConfig>,

    #[account(
        init,
        payer = user,
        space = 8 + ProtectedTransaction::INIT_SPACE,
        seeds = [b"protected_tx", user.key().as_ref()],
        bump
    )]
    pub protected_transaction: Account<'info, ProtectedTransaction>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Account context for executing a protected transaction
#[derive(Accounts)]
pub struct ExecuteProtectedTransaction<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump
    )]
    pub config: Account<'info, ProtectionConfig>,

    #[account(
        mut,
        seeds = [b"protected_tx", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub protected_transaction: Account<'info, ProtectedTransaction>,

    pub user: Signer<'info>,

    #[account(mut)]
    pub input_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub output_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

/// Account context for cancelling a protected transaction
#[derive(Accounts)]
pub struct CancelProtectedTransaction<'info> {
    #[account(
        mut,
        seeds = [b"protected_tx", user.key().as_ref()],
        bump,
        has_one = user
    )]
    /// Protected transaction account to be cancelled
    pub protected_transaction: Account<'info, ProtectedTransaction>,

    /// User who owns the transaction and can cancel it
    pub user: Signer<'info>,
}

/// Account context for updating protection configuration
#[derive(Accounts)]
pub struct UpdateProtectionConfig<'info> {
    /// The protection configuration account to update
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, ProtectionConfig>,

    /// Authority that can update the configuration
    pub authority: Signer<'info>,
}

/// Account context for reporting MEV attacks
#[derive(Accounts)]
pub struct ReportMevAttack<'info> {
    #[account(
        init,
        payer = reporter,
        space = 8 + AttackReport::INIT_SPACE,
        seeds = [b"attack_report", reporter.key().as_ref()],
        bump
    )]
    /// Attack report account to be created
    pub attack_report: Account<'info, AttackReport>,

    #[account(mut)]
    /// User reporting the MEV attack
    pub reporter: Signer<'info>,

    /// System program for account creation
    pub system_program: Program<'info, System>,
}

// Data structs
/// Configuration for MEV protection system
#[account]
#[derive(InitSpace)]
pub struct ProtectionConfig {
    /// Authority that can modify the protection configuration
    pub authority: Pubkey,
    /// Maximum allowed price impact in basis points
    pub max_price_impact: u16,
    /// Minimum time delay for transaction execution in seconds
    pub min_time_delay: i64,
    /// Maximum slippage protection in basis points
    pub max_slippage_protection: u16,
    /// Total number of transactions protected by the system
    pub total_transactions_protected: u64,
    /// Total number of MEV attacks prevented
    pub total_mev_attacks_prevented: u64,
    /// Whether the protection system is currently active
    pub is_active: bool,
    /// PDA bump seed for the configuration account
    pub bump: u8,
}

/// A transaction protected against MEV attacks
#[account]
#[derive(InitSpace)]
pub struct ProtectedTransaction {
    /// Address of the user who created this protected transaction
    pub user: Pubkey,
    /// Parameters defining the transaction details
    pub params: TransactionParams,
    /// Level of MEV protection applied
    pub protection_level: ProtectionLevel,
    /// Specific protection mechanisms enabled
    pub protection_mechanisms: ProtectionMechanisms,
    /// Current status of the transaction
    pub status: TransactionStatus,
    /// Unique nonce for transaction ordering
    pub nonce: u64,
    /// Timestamp when the transaction was created
    pub created_at: i64,
    /// Deadline for transaction execution
    pub execution_deadline: i64,
    /// Timestamp when the transaction was executed
    pub executed_at: i64,
    /// Timestamp when the transaction was cancelled
    pub cancelled_at: i64,
    /// Result of transaction execution if completed
    pub execution_result: Option<ExecutionResult>,
}

/// Report of a detected MEV attack
#[account]
#[derive(InitSpace)]
pub struct AttackReport {
    /// Address of the user who reported the attack
    pub reporter: Pubkey,
    /// Detailed information about the attack
    pub attack_details: AttackDetails,
    /// Timestamp when the attack was reported
    pub reported_at: i64,
    /// Current status of the report
    pub status: ReportStatus,
}

/// Parameters for a protected transaction
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TransactionParams {
    /// Token to be swapped from
    pub input_token: Pubkey,
    /// Token to be swapped to
    pub output_token: Pubkey,
    /// Amount of input tokens to swap
    pub input_amount: u64,
    /// Minimum acceptable output amount
    pub min_output_amount: u64,
    /// Maximum allowed slippage percentage
    pub max_slippage: u16,
    /// Name of the DEX to use for the swap
    #[max_len(50)]
    pub dex: String,
}

/// Level of MEV protection
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum ProtectionLevel {
    /// Basic protection with minimal overhead
    Basic,
    /// Advanced protection with comprehensive checks
    Advanced,
    /// Maximum protection with all available mechanisms
    Maximum,
}

/// Available protection mechanisms
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, InitSpace, Default)]
pub struct ProtectionMechanisms {
    /// Whether time delay protection is enabled
    pub time_delay: bool,
    /// Whether slippage protection is enabled
    pub slippage_protection: bool,
    /// Whether price impact checking is enabled
    pub price_impact_check: bool,
    /// Whether front-run detection is enabled
    pub frontrun_detection: bool,
    /// Whether commit-reveal scheme is enabled
    pub commit_reveal: bool,
    /// Whether private mempool routing is enabled
    pub private_mempool: bool,
}

/// Status of a protected transaction
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum TransactionStatus {
    /// Transaction is pending execution
    Pending,
    /// Transaction has been executed successfully
    Executed,
    /// Transaction was cancelled by user
    Cancelled,
    /// Transaction was blocked due to MEV risk
    Blocked,
}

/// Result of transaction execution
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ExecutionResult {
    /// Amount of input tokens used
    pub input_amount: u64,
    /// Amount of output tokens received
    pub output_amount: u64,
    /// Fee paid for MEV protection
    pub protection_fee: u64,
    /// Gas consumed during execution
    pub gas_used: u64,
    /// Time taken for execution in microseconds
    pub execution_time: u64,
}

/// Details of a detected MEV attack
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct AttackDetails {
    /// Type of MEV attack detected
    pub attack_type: AttackType,
    /// Transaction that was victimized
    pub victim_transaction: Pubkey,
    /// Address of the attacker (if known)
    pub attacker_address: Option<Pubkey>,
    /// Estimated financial damage in tokens
    pub estimated_damage: u64,
    /// Human-readable description of the attack
    #[max_len(200)]
    pub description: String,
}

/// Type of MEV attack
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum AttackType {
    /// No attack detected
    None,
    /// Sandwich attack (front-run + back-run)
    Sandwich,
    /// Front-running attack
    Frontrun,
    /// Back-running attack
    Backrun,
    /// Just-in-time liquidity attack
    JustInTime,
}

/// Status of an attack report
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum ReportStatus {
    /// Report is pending review
    Pending,
    /// Report has been verified as valid
    Verified,
    /// Report has been rejected as invalid
    Rejected,
}

/// Risk level assessment
#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, PartialOrd, Copy, InitSpace,
)]
pub enum RiskLevel {
    /// Low risk level
    Low,
    /// Medium risk level
    Medium,
    /// High risk level
    High,
    /// Critical risk level
    Critical,
}

/// MEV risk analysis result
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MevAnalysis {
    /// Overall risk level assessment
    pub risk_level: RiskLevel,
    /// Numerical risk score (0-1000)
    pub risk_score: u16,
    /// Price impact percentage
    pub price_impact: u16,
    /// Liquidity risk assessment
    pub liquidity_risk: u16,
    /// Timing-based risk factor
    pub timing_risk: u16,
}

/// Sandwich attack detection result
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SandwichDetection {
    /// Whether a sandwich attack was detected
    pub is_detected: bool,
    /// Risk score of the detected pattern
    pub risk_score: u16,
    /// Type of attack detected
    pub attack_type: AttackType,
    /// Confidence level of detection (0-100)
    pub confidence: u16,
}

/// Result of a swap operation
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapResult {
    /// Amount of output tokens received
    pub output_amount: u64,
    /// Gas consumed during execution
    pub gas_used: u64,
    /// Time taken for execution in microseconds
    pub execution_time: u64,
}

// Events
/// Event emitted when MEV protection is initialized
#[event]
pub struct MevProtectionInitialized {
    /// Authority that initialized the protection
    pub authority: Pubkey,
    /// Maximum allowed price impact percentage
    pub max_price_impact: u16,
    /// Minimum time delay for transaction execution
    pub min_time_delay: i64,
    /// Maximum slippage protection percentage
    pub max_slippage_protection: u16,
}

/// Event emitted when a protected transaction is created
#[event]
pub struct ProtectedTransactionCreated {
    /// User who created the protected transaction
    pub user: Pubkey,
    /// Unique identifier for the transaction
    pub transaction_id: u64,
    /// Level of protection applied
    pub protection_level: ProtectionLevel,
    /// Deadline for transaction execution
    pub execution_deadline: i64,
}

/// Event emitted when a protected transaction is executed
#[event]
pub struct ProtectedTransactionExecuted {
    /// Unique identifier for the transaction
    pub transaction_id: u64,
    /// User who executed the transaction
    pub user: Pubkey,
    /// Amount of input tokens
    pub input_amount: u64,
    /// Amount of output tokens received
    pub output_amount: u64,
    /// Fee paid for MEV protection
    pub protection_fee: u64,
    /// Assessed MEV risk level
    pub mev_risk_level: RiskLevel,
}

/// Event emitted when a protected transaction is cancelled
#[event]
pub struct ProtectedTransactionCancelled {
    /// Unique identifier for the transaction
    pub transaction_id: u64,
    /// User who cancelled the transaction
    pub user: Pubkey,
    /// Timestamp when transaction was cancelled
    pub cancelled_at: i64,
}

/// Event emitted when a sandwich attack is detected
#[event]
pub struct SandwichAttackDetected {
    /// Unique identifier for the transaction
    pub transaction_id: u64,
    /// User whose transaction was targeted
    pub user: Pubkey,
    /// Risk score of the detected attack
    pub risk_score: u16,
    /// Type of attack detected
    pub attack_type: AttackType,
}

/// Event emitted when protection configuration is updated
#[event]
pub struct ProtectionConfigUpdated {
    /// Authority that updated the configuration
    pub authority: Pubkey,
    /// Maximum allowed price impact percentage
    pub max_price_impact: u16,
    /// Minimum time delay for transaction execution
    pub min_time_delay: i64,
    /// Maximum slippage protection percentage
    pub max_slippage_protection: u16,
    /// Whether protection is active
    pub is_active: bool,
}

/// Event emitted when an MEV attack is reported
#[event]
pub struct MevAttackReported {
    /// Address of the reporter
    pub reporter: Pubkey,
    /// Type of MEV attack detected
    pub attack_type: AttackType,
    /// Transaction that was victimized
    pub victim_transaction: Pubkey,
    /// Estimated damage in tokens
    pub estimated_damage: u64,
}

// Errors
/// Errors that can occur in the MEV protection program
#[error_code]
pub enum MevProtectionError {
    /// MEV protection is inactive
    #[msg("MEV protection is inactive")]
    ProtectionInactive,

    /// Invalid amount
    #[msg("Invalid amount")]
    InvalidAmount,

    /// Slippage too high
    #[msg("Slippage too high")]
    SlippageTooHigh,

    /// Execution too early
    #[msg("Execution too early")]
    ExecutionTooEarly,

    /// Invalid transaction status
    #[msg("Invalid transaction status")]
    InvalidTransactionStatus,

    /// Sandwich attack detected
    #[msg("Sandwich attack detected")]
    SandwichAttackDetected,

    /// Price impact too high
    #[msg("Price impact too high")]
    PriceImpactTooHigh,

    /// Invalid time delay
    #[msg("Invalid time delay")]
    InvalidTimeDelay,

    /// Slippage protection too high
    #[msg("Slippage protection too high")]
    SlippageProtectionTooHigh,

    /// MEV risk too high
    #[msg("MEV risk too high")]
    MevRiskTooHigh,

    /// Frontrun attack detected
    #[msg("Frontrun attack detected")]
    FrontrunAttackDetected,

    /// Insufficient protection level
    #[msg("Insufficient protection level")]
    InsufficientProtectionLevel,

    /// Transaction expired
    #[msg("Transaction expired")]
    TransactionExpired,

    /// Unauthorized access
    #[msg("Unauthorized access")]
    Unauthorized,

    /// Invalid configuration
    #[msg("Invalid configuration")]
    InvalidConfig,

    /// Arithmetic overflow
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
