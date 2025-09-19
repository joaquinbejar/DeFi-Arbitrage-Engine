//! Helper functions for the MEV Protection Program

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::models::*;

// Helper functions
pub fn validate_transaction_params(
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

pub fn apply_basic_protection(_config: &ProtectionConfig) -> Result<ProtectionMechanisms> {
    // Basic protection: time delay and slippage protection
    Ok(ProtectionMechanisms {
        time_delay: true,
        slippage_protection: true,
        price_impact_check: false,
        frontrun_detection: false,
        commit_reveal: false,
        private_mempool: false,
    })
}

pub fn apply_advanced_protection(_config: &ProtectionConfig) -> Result<ProtectionMechanisms> {
    // Advanced protection: all basic + price impact and frontrun detection
    Ok(ProtectionMechanisms {
        time_delay: true,
        slippage_protection: true,
        price_impact_check: true,
        frontrun_detection: true,
        commit_reveal: false,
        private_mempool: false,
    })
}

pub fn apply_maximum_protection(_config: &ProtectionConfig) -> Result<ProtectionMechanisms> {
    // Maximum protection: all mechanisms enabled
    Ok(ProtectionMechanisms {
        time_delay: true,
        slippage_protection: true,
        price_impact_check: true,
        frontrun_detection: true,
        commit_reveal: true,
        private_mempool: true,
    })
}

pub fn analyze_mev_risk(
    params: &TransactionParams,
    _accounts: &[AccountInfo],
) -> Result<MevAnalysis> {
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

pub fn detect_sandwich_attack(
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

pub fn handle_high_mev_risk(
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

pub fn execute_with_protection(
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

pub fn simulate_protected_swap(
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

pub fn estimate_price_impact(params: &TransactionParams) -> Result<u16> {
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

pub fn calculate_liquidity_risk(params: &TransactionParams) -> Result<u16> {
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

pub fn calculate_timing_risk() -> Result<u16> {
    // Calculate timing-based risk
    let current_slot = Clock::get()?.slot;
    let risk = (current_slot % 1000) as u16; // Simplified timing risk

    Ok(risk)
}

pub fn calculate_detection_confidence(risk_score: u16) -> u16 {
    // Calculate confidence level for attack detection
    match risk_score {
        0..=300 => 2000,   // 20% confidence
        301..=600 => 5000, // 50% confidence
        601..=900 => 8000, // 80% confidence
        _ => 9500,         // 95% confidence
    }
}

pub fn calculate_protection_fee(amount: u64, protection_level: &ProtectionLevel) -> u64 {
    let fee_rate = match protection_level {
        ProtectionLevel::Basic => 10,    // 0.1%
        ProtectionLevel::Advanced => 25, // 0.25%
        ProtectionLevel::Maximum => 50,  // 0.5%
    };

    amount * fee_rate / 10000
}

pub fn generate_nonce() -> Result<u64> {
    let clock = Clock::get()?;
    Ok(clock.unix_timestamp as u64 + clock.slot)
}

#[allow(dead_code)]
pub fn simulate_swap(params: &TransactionParams, input_amount: u64) -> Result<u64> {
    // Simplified swap simulation
    // In reality, this would interact with actual DEX
    let base_output = input_amount * 95 / 100; // Assume 5% fee/slippage
    let slippage_factor = 10000 - params.max_slippage;
    let output_amount = base_output * slippage_factor as u64 / 10000;

    Ok(output_amount)
}

/// Handler for initializing MEV protection program
pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    config.max_price_impact = 500; // 5% default
    config.min_time_delay = 10; // 10 seconds default
    config.max_slippage_protection = 200; // 2% default
    config.is_active = true;
    config.total_transactions_protected = 0;
    config.total_mev_attacks_prevented = 0;

    emit!(MevProtectionInitialized {
        authority: config.authority,
        max_price_impact: config.max_price_impact,
        min_time_delay: config.min_time_delay,
        max_slippage_protection: config.max_slippage_protection,
    });

    Ok(())
}

/// Handler for creating protected transaction
pub fn create_protected_transaction_handler(
    ctx: Context<CreateProtectedTransaction>,
    transaction_data: TransactionParams,
    protection_level: ProtectionLevel,
) -> Result<()> {
    let protected_tx = &mut ctx.accounts.protected_transaction;
    let config = &ctx.accounts.config;

    require!(config.is_active, MevProtectionError::ProtectionInactive);

    // Validate transaction parameters
    validate_transaction_params(&transaction_data, config)?;

    // Apply protection based on level
    let protection_mechanisms = match protection_level {
        ProtectionLevel::Basic => apply_basic_protection(config)?,
        ProtectionLevel::Advanced => apply_advanced_protection(config)?,
        ProtectionLevel::Maximum => apply_maximum_protection(config)?,
    };

    protected_tx.user = ctx.accounts.user.key();
    protected_tx.params = transaction_data.clone();
    protected_tx.protection_level = protection_level;
    protected_tx.protection_mechanisms = protection_mechanisms;
    protected_tx.status = TransactionStatus::Pending;
    protected_tx.created_at = Clock::get()?.unix_timestamp;
    protected_tx.execution_deadline = protected_tx.created_at + config.min_time_delay;
    protected_tx.nonce = 0;
    protected_tx.executed_at = 0;
    protected_tx.cancelled_at = 0;
    protected_tx.execution_result = None;

    emit!(ProtectedTransactionCreated {
        user: protected_tx.user,
        transaction_id: 0,
        protection_level,
        execution_deadline: protected_tx.execution_deadline,
    });

    Ok(())
}

/// Handler for executing protected transaction
pub fn execute_protected_transaction_handler(
    ctx: Context<ExecuteProtectedTransaction>,
) -> Result<()> {
    let protected_tx = &mut ctx.accounts.protected_transaction;
    let config = &mut ctx.accounts.config;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        protected_tx.status == TransactionStatus::Pending,
        MevProtectionError::InvalidTransactionStatus
    );
    require!(
        current_time >= protected_tx.execution_deadline,
        MevProtectionError::ExecutionTooEarly
    );

    // Analyze MEV risk
    let mev_risk = analyze_mev_risk(&protected_tx.params, ctx.remaining_accounts)?;

    // Handle high MEV risk scenarios
    if matches!(mev_risk.risk_level, RiskLevel::High | RiskLevel::Critical) {
        handle_high_mev_risk(protected_tx, &mev_risk, config)?;
    }

    // Execute with protection
    let execution_result = execute_with_protection(
        &protected_tx.params,
        &protected_tx.protection_level,
        &ctx.accounts.user,
        &ctx.accounts.input_token_account,
        &ctx.accounts.output_token_account,
        &ctx.accounts.token_program,
        ctx.remaining_accounts,
    )?;

    // Update transaction status and statistics
    protected_tx.status = TransactionStatus::Executed;
    protected_tx.executed_at = current_time;
    protected_tx.execution_result = Some(execution_result.clone());

    config.total_transactions_protected = config
        .total_transactions_protected
        .checked_add(1)
        .ok_or(MevProtectionError::ArithmeticOverflow)?;

    if matches!(mev_risk.risk_level, RiskLevel::High | RiskLevel::Critical) {
        config.total_mev_attacks_prevented = config
            .total_mev_attacks_prevented
            .checked_add(1)
            .ok_or(MevProtectionError::ArithmeticOverflow)?;
    }

    emit!(ProtectedTransactionExecuted {
        transaction_id: 0, // TODO: Generate proper transaction ID
        user: protected_tx.user,
        input_amount: execution_result.input_amount,
        output_amount: execution_result.output_amount,
        protection_fee: execution_result.protection_fee,
        mev_risk_level: mev_risk.risk_level,
    });

    Ok(())
}

/// Handler for canceling protected transaction
pub fn cancel_protected_transaction_handler(
    ctx: Context<CancelProtectedTransaction>,
) -> Result<()> {
    let protected_tx = &mut ctx.accounts.protected_transaction;

    require!(
        protected_tx.status == TransactionStatus::Pending,
        MevProtectionError::InvalidTransactionStatus
    );
    require!(
        protected_tx.user == ctx.accounts.user.key(),
        MevProtectionError::Unauthorized
    );

    protected_tx.status = TransactionStatus::Cancelled;
    protected_tx.cancelled_at = Clock::get()?.unix_timestamp;

    emit!(ProtectedTransactionCancelled {
        transaction_id: 0, // TODO: Generate proper transaction ID
        user: protected_tx.user,
        cancelled_at: protected_tx.cancelled_at,
    });

    Ok(())
}

/// Handler for updating protection configuration
pub fn update_protection_config_handler(
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

/// Handler for reporting MEV attack
pub fn report_mev_attack_handler(
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
