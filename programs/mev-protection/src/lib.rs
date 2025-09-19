//! MEV Protection Program
//!
//! This Anchor program provides protection against MEV attacks,
//! particularly sandwich attacks, by implementing various defensive mechanisms.

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

declare_id!("MevProtect111111111111111111111111111111111");

#[program]
pub mod mev_protection {
    use super::*;

    /// Initialize the MEV protection program
    pub fn initialize(
        ctx: Context<Initialize>,
        max_price_impact: u16,
        min_time_delay: i64,
        max_slippage_protection: u16,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.max_price_impact = max_price_impact;
        config.min_time_delay = min_time_delay;
        config.max_slippage_protection = max_slippage_protection;
        config.total_transactions_protected = 0;
        config.total_mev_attacks_prevented = 0;
        config.is_active = true;
        config.bump = ctx.bumps.config;

        emit!(MevProtectionInitialized {
            authority: config.authority,
            max_price_impact,
            min_time_delay,
            max_slippage_protection,
        });

        Ok(())
    }

    /// Create a protected transaction with MEV safeguards
    pub fn create_protected_transaction(
        ctx: Context<CreateProtectedTransaction>,
        transaction_params: TransactionParams,
        protection_level: ProtectionLevel,
    ) -> Result<()> {
        let config = &ctx.accounts.config;

        // Check if protection is active
        require!(config.is_active, MevProtectionError::ProtectionInactive);

        // Validate transaction parameters
        validate_transaction_params(&transaction_params, config)?;

        let protected_tx = &mut ctx.accounts.protected_transaction;
        protected_tx.user = ctx.accounts.user.key();
        protected_tx.params = transaction_params.clone();
        protected_tx.protection_level = protection_level;
        protected_tx.status = TransactionStatus::Pending;
        protected_tx.created_at = Clock::get()?.unix_timestamp;
        protected_tx.execution_deadline = protected_tx.created_at + config.min_time_delay;
        protected_tx.nonce = generate_nonce()?;
        protected_tx.protection_mechanisms = ProtectionMechanisms::default();
        protected_tx.executed_at = 0;
        protected_tx.cancelled_at = 0;
        protected_tx.execution_result = None;

        // Apply protection mechanisms based on level
        match protection_level {
            ProtectionLevel::Basic => {
                apply_basic_protection(protected_tx, config)?;
            }
            ProtectionLevel::Advanced => {
                apply_advanced_protection(protected_tx, config)?;
            }
            ProtectionLevel::Maximum => {
                apply_maximum_protection(protected_tx, config)?;
            }
        }

        emit!(ProtectedTransactionCreated {
            user: ctx.accounts.user.key(),
            transaction_id: protected_tx.nonce,
            protection_level,
            execution_deadline: protected_tx.execution_deadline,
        });

        Ok(())
    }

    /// Execute a protected transaction with MEV checks
    pub fn execute_protected_transaction(ctx: Context<ExecuteProtectedTransaction>) -> Result<()> {
        let config = &ctx.accounts.config;
        let protected_tx = &mut ctx.accounts.protected_transaction;

        // Check if transaction is ready for execution
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time >= protected_tx.execution_deadline,
            MevProtectionError::ExecutionTooEarly
        );

        require!(
            protected_tx.status == TransactionStatus::Pending,
            MevProtectionError::InvalidTransactionStatus
        );

        // Perform MEV detection checks
        let mev_analysis = analyze_mev_risk(&protected_tx.params, ctx.remaining_accounts)?;

        if mev_analysis.risk_level > RiskLevel::Low {
            // Apply additional protections or delay execution
            handle_high_mev_risk(protected_tx, &mev_analysis, config)?;
        }

        // Check for sandwich attack patterns
        let sandwich_risk =
            detect_sandwich_attack(&protected_tx.params, current_time, ctx.remaining_accounts)?;

        if sandwich_risk.is_detected {
            protected_tx.status = TransactionStatus::Blocked;

            emit!(SandwichAttackDetected {
                transaction_id: protected_tx.nonce,
                user: protected_tx.user,
                risk_score: sandwich_risk.risk_score,
                attack_type: sandwich_risk.attack_type,
            });

            return Err(MevProtectionError::SandwichAttackDetected.into());
        }

        // Execute the transaction with protection
        let execution_result = execute_with_protection(
            &protected_tx.params,
            &protected_tx.protection_level,
            &ctx.accounts.user,
            &ctx.accounts.input_token_account,
            &ctx.accounts.output_token_account,
            &ctx.accounts.token_program,
            ctx.remaining_accounts,
        )?;

        // Update transaction status
        protected_tx.status = TransactionStatus::Executed;
        protected_tx.executed_at = current_time;
        protected_tx.execution_result = Some(execution_result.clone());

        // Update global statistics
        let config = &mut ctx.accounts.config;
        config.total_transactions_protected += 1;
        if mev_analysis.risk_level > RiskLevel::Low || sandwich_risk.is_detected {
            config.total_mev_attacks_prevented += 1;
        }

        emit!(ProtectedTransactionExecuted {
            transaction_id: protected_tx.nonce,
            user: protected_tx.user,
            input_amount: execution_result.input_amount,
            output_amount: execution_result.output_amount,
            protection_fee: execution_result.protection_fee,
            mev_risk_level: mev_analysis.risk_level,
        });

        Ok(())
    }

    /// Cancel a protected transaction
    pub fn cancel_protected_transaction(ctx: Context<CancelProtectedTransaction>) -> Result<()> {
        let protected_tx = &mut ctx.accounts.protected_transaction;

        require!(
            protected_tx.status == TransactionStatus::Pending,
            MevProtectionError::InvalidTransactionStatus
        );

        protected_tx.status = TransactionStatus::Cancelled;
        protected_tx.cancelled_at = Clock::get()?.unix_timestamp;

        emit!(ProtectedTransactionCancelled {
            transaction_id: protected_tx.nonce,
            user: protected_tx.user,
            cancelled_at: protected_tx.cancelled_at,
        });

        Ok(())
    }

    /// Update MEV protection configuration (admin only)
    pub fn update_protection_config(
        ctx: Context<UpdateProtectionConfig>,
        max_price_impact: Option<u16>,
        min_time_delay: Option<i64>,
        max_slippage_protection: Option<u16>,
        is_active: Option<bool>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;

        if let Some(impact) = max_price_impact {
            require!(impact <= 5000, MevProtectionError::PriceImpactTooHigh); // Max 50%
            config.max_price_impact = impact;
        }

        if let Some(delay) = min_time_delay {
            require!(
                (0..=300).contains(&delay),
                MevProtectionError::InvalidTimeDelay
            ); // Max 5 minutes
            config.min_time_delay = delay;
        }

        if let Some(slippage) = max_slippage_protection {
            require!(
                slippage <= 1000,
                MevProtectionError::SlippageProtectionTooHigh
            ); // Max 10%
            config.max_slippage_protection = slippage;
        }

        if let Some(active) = is_active {
            config.is_active = active;
        }

        emit!(ProtectionConfigUpdated {
            authority: ctx.accounts.authority.key(),
            max_price_impact: config.max_price_impact,
            min_time_delay: config.min_time_delay,
            max_slippage_protection: config.max_slippage_protection,
            is_active: config.is_active,
        });

        Ok(())
    }

    /// Report MEV attack attempt
    pub fn report_mev_attack(
        ctx: Context<ReportMevAttack>,
        attack_details: AttackDetails,
    ) -> Result<()> {
        let report = &mut ctx.accounts.attack_report;
        report.reporter = ctx.accounts.reporter.key();
        report.attack_details = attack_details.clone();
        report.reported_at = Clock::get()?.unix_timestamp;
        report.status = ReportStatus::Pending;

        emit!(MevAttackReported {
            reporter: report.reporter,
            attack_type: attack_details.attack_type,
            victim_transaction: attack_details.victim_transaction,
            estimated_damage: attack_details.estimated_damage,
        });

        Ok(())
    }
}

// Helper functions
fn validate_transaction_params(
    params: &TransactionParams,
    config: &ProtectionConfig,
) -> Result<()> {
    require!(params.input_amount > 0, MevProtectionError::InvalidAmount);
    require!(
        params.min_output_amount > 0,
        MevProtectionError::InvalidAmount
    );
    require!(
        params.max_slippage <= config.max_slippage_protection,
        MevProtectionError::SlippageTooHigh
    );

    Ok(())
}

fn apply_basic_protection(
    protected_tx: &mut ProtectedTransaction,
    _config: &ProtectionConfig,
) -> Result<()> {
    // Basic protection: time delay and slippage protection
    protected_tx.protection_mechanisms.time_delay = true;
    protected_tx.protection_mechanisms.slippage_protection = true;
    protected_tx.protection_mechanisms.price_impact_check = false;
    protected_tx.protection_mechanisms.frontrun_detection = false;

    Ok(())
}

fn apply_advanced_protection(
    protected_tx: &mut ProtectedTransaction,
    config: &ProtectionConfig,
) -> Result<()> {
    // Advanced protection: all basic + price impact and frontrun detection
    protected_tx.protection_mechanisms.time_delay = true;
    protected_tx.protection_mechanisms.slippage_protection = true;
    protected_tx.protection_mechanisms.price_impact_check = true;
    protected_tx.protection_mechanisms.frontrun_detection = true;

    // Increase time delay for advanced protection
    protected_tx.execution_deadline += config.min_time_delay / 2;

    Ok(())
}

fn apply_maximum_protection(
    protected_tx: &mut ProtectedTransaction,
    config: &ProtectionConfig,
) -> Result<()> {
    // Maximum protection: all mechanisms enabled
    protected_tx.protection_mechanisms.time_delay = true;
    protected_tx.protection_mechanisms.slippage_protection = true;
    protected_tx.protection_mechanisms.price_impact_check = true;
    protected_tx.protection_mechanisms.frontrun_detection = true;
    protected_tx.protection_mechanisms.commit_reveal = true;
    protected_tx.protection_mechanisms.private_mempool = true;

    // Maximum time delay
    protected_tx.execution_deadline += config.min_time_delay;

    Ok(())
}

fn analyze_mev_risk(params: &TransactionParams, _accounts: &[AccountInfo]) -> Result<MevAnalysis> {
    // Analyze various MEV risk factors
    let mut risk_score = 0u16;

    // Check transaction size (larger transactions = higher MEV risk)
    if params.input_amount > 1_000_000_000 {
        // > 1000 tokens
        risk_score += 300;
    } else if params.input_amount > 100_000_000 {
        // > 100 tokens
        risk_score += 150;
    }

    // Check slippage tolerance (higher slippage = higher MEV risk)
    if params.max_slippage > 500 {
        // > 5%
        risk_score += 400;
    } else if params.max_slippage > 100 {
        // > 1%
        risk_score += 200;
    }

    // Check price impact
    let estimated_price_impact = estimate_price_impact(params)?;
    if estimated_price_impact > 300 {
        // > 3%
        risk_score += 500;
    } else if estimated_price_impact > 100 {
        // > 1%
        risk_score += 250;
    }

    let risk_level = match risk_score {
        0..=200 => RiskLevel::Low,
        201..=500 => RiskLevel::Medium,
        501..=800 => RiskLevel::High,
        _ => RiskLevel::Critical,
    };

    Ok(MevAnalysis {
        risk_level,
        risk_score,
        price_impact: estimated_price_impact,
        liquidity_risk: calculate_liquidity_risk(params)?,
        timing_risk: calculate_timing_risk()?,
    })
}

fn detect_sandwich_attack(
    params: &TransactionParams,
    current_time: i64,
    _accounts: &[AccountInfo],
) -> Result<SandwichDetection> {
    // Simplified sandwich attack detection
    // In a real implementation, this would analyze mempool and recent transactions

    #[allow(unused_assignments)]
    let mut is_detected = false;
    let mut risk_score = 0u16;
    let mut attack_type = AttackType::None;

    // Check for suspicious patterns
    // 1. Large transaction with high slippage tolerance
    if params.input_amount > 500_000_000 && params.max_slippage > 300 {
        risk_score += 400;
        attack_type = AttackType::Sandwich;
    }

    // 2. Check timing patterns (simplified)
    let time_factor = (current_time % 60) as u16; // Use timestamp modulo for demo
    if time_factor < 5 {
        // Suspicious timing
        risk_score += 200;
    }

    // 3. Check for frontrunning patterns
    if params.max_slippage > 500 {
        // > 5% slippage tolerance
        risk_score += 300;
        if attack_type == AttackType::None {
            attack_type = AttackType::Frontrun;
        }
    }

    is_detected = risk_score > 600;

    Ok(SandwichDetection {
        is_detected,
        risk_score,
        attack_type,
        confidence: calculate_detection_confidence(risk_score),
    })
}

fn handle_high_mev_risk(
    protected_tx: &mut ProtectedTransaction,
    mev_analysis: &MevAnalysis,
    config: &ProtectionConfig,
) -> Result<()> {
    match mev_analysis.risk_level {
        RiskLevel::High => {
            // Add extra delay for high risk
            protected_tx.execution_deadline += config.min_time_delay / 2;
        }
        RiskLevel::Critical => {
            // Maximum delay for critical risk
            protected_tx.execution_deadline += config.min_time_delay;
            // Enable all protection mechanisms
            protected_tx.protection_mechanisms.commit_reveal = true;
            protected_tx.protection_mechanisms.private_mempool = true;
        }
        _ => {}
    }

    Ok(())
}

fn execute_with_protection(
    params: &TransactionParams,
    protection_level: &ProtectionLevel,
    _user: &Signer,
    _input_account: &Account<TokenAccount>,
    _output_account: &Account<TokenAccount>,
    _token_program: &Program<Token>,
    _remaining_accounts: &[AccountInfo],
) -> Result<ExecutionResult> {
    // Execute the actual swap with protection mechanisms

    let protection_fee = calculate_protection_fee(params.input_amount, protection_level);

    // Simulate swap execution (in real implementation, this would call DEX)
    let swap_result = simulate_protected_swap(params, protection_level)?;

    // Collect protection fee if applicable
    if protection_fee > 0 {
        // In a real implementation, transfer protection fee to fee account
    }

    Ok(ExecutionResult {
        input_amount: params.input_amount,
        output_amount: swap_result.output_amount,
        protection_fee,
        gas_used: swap_result.gas_used,
        execution_time: swap_result.execution_time,
    })
}

fn simulate_protected_swap(
    params: &TransactionParams,
    _protection_level: &ProtectionLevel,
) -> Result<SwapResult> {
    // Simulate swap with protection
    let output_amount = params.input_amount * 99 / 100; // 1% slippage simulation

    Ok(SwapResult {
        output_amount,
        gas_used: 50000,      // Estimated gas
        execution_time: 2000, // 2 seconds
    })
}

fn estimate_price_impact(params: &TransactionParams) -> Result<u16> {
    // Simplified price impact estimation
    // In reality, this would query pool liquidity
    let impact = if params.input_amount > 1_000_000_000 {
        500 // 5%
    } else if params.input_amount > 100_000_000 {
        200 // 2%
    } else {
        50 // 0.5%
    };

    Ok(impact)
}

fn calculate_liquidity_risk(params: &TransactionParams) -> Result<u16> {
    // Calculate liquidity risk based on transaction size
    let risk = if params.input_amount > 1_000_000_000 {
        800 // High risk
    } else if params.input_amount > 100_000_000 {
        400 // Medium risk
    } else {
        100 // Low risk
    };

    Ok(risk)
}

fn calculate_timing_risk() -> Result<u16> {
    // Calculate timing-based risk
    let current_slot = Clock::get()?.slot;
    let risk = (current_slot % 1000) as u16; // Simplified timing risk

    Ok(risk)
}

fn calculate_detection_confidence(risk_score: u16) -> u16 {
    // Calculate confidence level for attack detection
    match risk_score {
        0..=300 => 2000,   // 20% confidence
        301..=600 => 5000, // 50% confidence
        601..=900 => 8000, // 80% confidence
        _ => 9500,         // 95% confidence
    }
}

fn calculate_protection_fee(amount: u64, protection_level: &ProtectionLevel) -> u64 {
    let fee_rate = match protection_level {
        ProtectionLevel::Basic => 10,    // 0.1%
        ProtectionLevel::Advanced => 25, // 0.25%
        ProtectionLevel::Maximum => 50,  // 0.5%
    };

    amount * fee_rate / 10000
}

fn generate_nonce() -> Result<u64> {
    let clock = Clock::get()?;
    Ok(clock.unix_timestamp as u64 + clock.slot)
}

#[allow(dead_code)]
fn simulate_swap(params: &TransactionParams, input_amount: u64) -> Result<u64> {
    // Simplified swap simulation
    // In reality, this would interact with actual DEX
    let base_output = input_amount * 95 / 100; // Assume 5% fee/slippage
    let slippage_factor = 10000 - params.max_slippage;
    let output_amount = base_output * slippage_factor as u64 / 10000;

    Ok(output_amount)
}

// Account structs
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

#[derive(Accounts)]
pub struct CancelProtectedTransaction<'info> {
    #[account(
        mut,
        seeds = [b"protected_tx", user.key().as_ref()],
        bump,
        has_one = user
    )]
    pub protected_transaction: Account<'info, ProtectedTransaction>,

    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateProtectionConfig<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, ProtectionConfig>,

    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReportMevAttack<'info> {
    #[account(
        init,
        payer = reporter,
        space = 8 + AttackReport::INIT_SPACE,
        seeds = [b"attack_report", reporter.key().as_ref()],
        bump
    )]
    pub attack_report: Account<'info, AttackReport>,

    #[account(mut)]
    pub reporter: Signer<'info>,

    pub system_program: Program<'info, System>,
}

// Data structs
#[account]
#[derive(InitSpace)]
pub struct ProtectionConfig {
    pub authority: Pubkey,
    pub max_price_impact: u16,
    pub min_time_delay: i64,
    pub max_slippage_protection: u16,
    pub total_transactions_protected: u64,
    pub total_mev_attacks_prevented: u64,
    pub is_active: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ProtectedTransaction {
    pub user: Pubkey,
    pub params: TransactionParams,
    pub protection_level: ProtectionLevel,
    pub protection_mechanisms: ProtectionMechanisms,
    pub status: TransactionStatus,
    pub nonce: u64,
    pub created_at: i64,
    pub execution_deadline: i64,
    pub executed_at: i64,
    pub cancelled_at: i64,
    pub execution_result: Option<ExecutionResult>,
}

#[account]
#[derive(InitSpace)]
pub struct AttackReport {
    pub reporter: Pubkey,
    pub attack_details: AttackDetails,
    pub reported_at: i64,
    pub status: ReportStatus,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct TransactionParams {
    pub input_token: Pubkey,
    pub output_token: Pubkey,
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub max_slippage: u16,
    #[max_len(50)]
    pub dex: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum ProtectionLevel {
    Basic,
    Advanced,
    Maximum,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Copy, InitSpace, Default)]
pub struct ProtectionMechanisms {
    pub time_delay: bool,
    pub slippage_protection: bool,
    pub price_impact_check: bool,
    pub frontrun_detection: bool,
    pub commit_reveal: bool,
    pub private_mempool: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum TransactionStatus {
    Pending,
    Executed,
    Cancelled,
    Blocked,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ExecutionResult {
    pub input_amount: u64,
    pub output_amount: u64,
    pub protection_fee: u64,
    pub gas_used: u64,
    pub execution_time: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct AttackDetails {
    pub attack_type: AttackType,
    pub victim_transaction: Pubkey,
    pub attacker_address: Option<Pubkey>,
    pub estimated_damage: u64,
    #[max_len(200)]
    pub description: String,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum AttackType {
    None,
    Sandwich,
    Frontrun,
    Backrun,
    JustInTime,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Copy, InitSpace)]
pub enum ReportStatus {
    Pending,
    Verified,
    Rejected,
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, PartialOrd, Copy, InitSpace,
)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MevAnalysis {
    pub risk_level: RiskLevel,
    pub risk_score: u16,
    pub price_impact: u16,
    pub liquidity_risk: u16,
    pub timing_risk: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SandwichDetection {
    pub is_detected: bool,
    pub risk_score: u16,
    pub attack_type: AttackType,
    pub confidence: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SwapResult {
    pub output_amount: u64,
    pub gas_used: u64,
    pub execution_time: u64,
}

// Events
#[event]
pub struct MevProtectionInitialized {
    pub authority: Pubkey,
    pub max_price_impact: u16,
    pub min_time_delay: i64,
    pub max_slippage_protection: u16,
}

#[event]
pub struct ProtectedTransactionCreated {
    pub user: Pubkey,
    pub transaction_id: u64,
    pub protection_level: ProtectionLevel,
    pub execution_deadline: i64,
}

#[event]
pub struct ProtectedTransactionExecuted {
    pub transaction_id: u64,
    pub user: Pubkey,
    pub input_amount: u64,
    pub output_amount: u64,
    pub protection_fee: u64,
    pub mev_risk_level: RiskLevel,
}

#[event]
pub struct ProtectedTransactionCancelled {
    pub transaction_id: u64,
    pub user: Pubkey,
    pub cancelled_at: i64,
}

#[event]
pub struct SandwichAttackDetected {
    pub transaction_id: u64,
    pub user: Pubkey,
    pub risk_score: u16,
    pub attack_type: AttackType,
}

#[event]
pub struct ProtectionConfigUpdated {
    pub authority: Pubkey,
    pub max_price_impact: u16,
    pub min_time_delay: i64,
    pub max_slippage_protection: u16,
    pub is_active: bool,
}

#[event]
pub struct MevAttackReported {
    pub reporter: Pubkey,
    pub attack_type: AttackType,
    pub victim_transaction: Pubkey,
    pub estimated_damage: u64,
}

// Errors
#[error_code]
pub enum MevProtectionError {
    #[msg("MEV protection is inactive")]
    ProtectionInactive,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Slippage too high")]
    SlippageTooHigh,

    #[msg("Execution too early")]
    ExecutionTooEarly,

    #[msg("Invalid transaction status")]
    InvalidTransactionStatus,

    #[msg("Sandwich attack detected")]
    SandwichAttackDetected,

    #[msg("Price impact too high")]
    PriceImpactTooHigh,

    #[msg("Invalid time delay")]
    InvalidTimeDelay,

    #[msg("Slippage protection too high")]
    SlippageProtectionTooHigh,

    #[msg("MEV risk too high")]
    MevRiskTooHigh,

    #[msg("Frontrun attack detected")]
    FrontrunAttackDetected,

    #[msg("Insufficient protection level")]
    InsufficientProtectionLevel,

    #[msg("Transaction expired")]
    TransactionExpired,

    #[msg("Unauthorized access")]
    Unauthorized,

    #[msg("Invalid configuration")]
    InvalidConfig,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}
