//! Helper functions for the Flash Arbitrage Program

use crate::models::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

/// Execute flash loan from provider
pub fn execute_flash_loan(
    _provider: &AccountInfo,
    _token_account: &Account<TokenAccount>,
    amount: u64,
) -> Result<FlashLoanResult> {
    // In a real implementation, this would interact with a lending protocol
    // like Solend, Mango, or a custom flash loan provider

    Ok(FlashLoanResult {
        amount_borrowed: amount,
        fee: calculate_flash_loan_fee(amount, 30), // 0.3% fee
        success: true,
    })
}

/// Repay flash loan to provider
pub fn repay_flash_loan(
    _provider: &AccountInfo,
    _token_account: &Account<TokenAccount>,
    _amount: u64,
) -> Result<()> {
    // In a real implementation, this would repay the flash loan
    Ok(())
}

/// Execute arbitrage route on specified DEX
pub fn execute_route(
    route: &ArbitrageRoute,
    input_amount: u64,
    max_slippage: u16,
    accounts: &[AccountInfo],
) -> Result<RouteResult> {
    // Validate slippage
    let min_output = calculate_min_output(input_amount, route.expected_output, max_slippage);

    // Execute trade based on DEX type
    let output_amount = match route.dex.as_str() {
        "raydium" => execute_raydium_swap(accounts, input_amount, min_output)?,
        "orca" => execute_orca_swap(accounts, input_amount, min_output)?,
        "meteora" => execute_meteora_swap(accounts, input_amount, min_output)?,
        "jupiter" => execute_jupiter_swap(accounts, input_amount, min_output)?,
        _ => return Err(FlashArbitrageError::UnsupportedDex.into()),
    };

    let fees_paid = calculate_dex_fees(input_amount, &route.dex);

    Ok(RouteResult {
        input_amount,
        output_amount,
        fees_paid,
        slippage: calculate_slippage(route.expected_output, output_amount),
    })
}

/// Execute swap on Raydium DEX
pub fn execute_raydium_swap(
    _accounts: &[AccountInfo],
    input_amount: u64,
    min_output: u64,
) -> Result<u64> {
    // Simulate Raydium swap
    // In a real implementation, this would call Raydium's swap instruction
    let output = input_amount * 99 / 100; // Simulate 1% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

/// Execute swap on Orca DEX
pub fn execute_orca_swap(
    _accounts: &[AccountInfo],
    input_amount: u64,
    min_output: u64,
) -> Result<u64> {
    // Simulate Orca swap
    let output = input_amount * 995 / 1000; // Simulate 0.5% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

/// Execute swap on Meteora DEX
pub fn execute_meteora_swap(
    _accounts: &[AccountInfo],
    input_amount: u64,
    min_output: u64,
) -> Result<u64> {
    // Simulate Meteora swap
    let output = input_amount * 997 / 1000; // Simulate 0.3% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

/// Execute swap on Jupiter DEX
pub fn execute_jupiter_swap(
    _accounts: &[AccountInfo],
    input_amount: u64,
    min_output: u64,
) -> Result<u64> {
    // Simulate Jupiter swap
    let output = input_amount * 998 / 1000; // Simulate 0.2% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

/// Calculate flash loan fee based on amount and fee rate in basis points
pub fn calculate_flash_loan_fee(amount: u64, fee_bps: u16) -> u64 {
    amount * fee_bps as u64 / 10000
}

/// Calculate program fee based on profit and fee rate
pub fn calculate_program_fee(profit: u64, fee_rate: u16) -> u64 {
    profit * fee_rate as u64 / 10000
}

/// Calculate minimum output amount considering slippage
pub fn calculate_min_output(_input: u64, expected: u64, max_slippage_bps: u16) -> u64 {
    expected * (10000 - max_slippage_bps as u64) / 10000
}

/// Calculate actual slippage between expected and actual output
pub fn calculate_slippage(expected: u64, actual: u64) -> u16 {
    if actual >= expected {
        0
    } else {
        ((expected - actual) * 10000 / expected) as u16
    }
}

/// Calculate DEX-specific trading fees
pub fn calculate_dex_fees(amount: u64, dex: &str) -> u64 {
    match dex {
        "raydium" => amount * 25 / 10000, // 0.25%
        "orca" => amount * 30 / 10000,    // 0.30%
        "meteora" => amount * 20 / 10000, // 0.20%
        "jupiter" => amount * 15 / 10000, // 0.15%
        _ => amount * 30 / 10000,         // Default 0.30%
    }
}

/// Initialize handler
pub fn initialize_handler(
    ctx: Context<Initialize>,
    fee_rate: u16,
    max_slippage: u16,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    config.fee_rate = fee_rate;
    config.max_slippage = max_slippage;
    config.total_volume = 0;
    config.total_fees_collected = 0;
    config.is_paused = false;
    config.bump = ctx.bumps.config;

    emit!(ProgramInitialized {
        authority: config.authority,
        fee_rate,
        max_slippage,
    });

    Ok(())
}

/// Execute flash arbitrage handler
pub fn execute_flash_arbitrage_handler(
    ctx: Context<ExecuteFlashArbitrage>,
    flash_loan_amount: u64,
    min_profit: u64,
    routes: Vec<ArbitrageRoute>,
) -> Result<()> {
    use anchor_spl::token::{self, Transfer};

    let config = &ctx.accounts.config;

    // Check if program is paused
    require!(!config.is_paused, FlashArbitrageError::ProgramPaused);

    // Validate routes
    require!(!routes.is_empty(), FlashArbitrageError::EmptyRoutes);
    require!(routes.len() <= 5, FlashArbitrageError::TooManyRoutes);

    // Validate flash loan amount
    require!(flash_loan_amount > 0, FlashArbitrageError::InvalidAmount);
    require!(
        flash_loan_amount <= 1_000_000_000_000,
        FlashArbitrageError::AmountTooLarge
    );

    let arbitrage_state = &mut ctx.accounts.arbitrage_state;
    arbitrage_state.flash_loan_amount = flash_loan_amount;
    arbitrage_state.min_profit = min_profit;
    arbitrage_state.routes = routes.clone();
    arbitrage_state.status = ArbitrageStatus::InProgress;
    arbitrage_state.start_time = Clock::get()?.unix_timestamp;

    // Step 1: Take flash loan
    let _initial_balance = ctx.accounts.user_token_account.amount;

    // Simulate flash loan by temporarily increasing balance
    // In a real implementation, this would interact with a lending protocol
    let _flash_loan_result = execute_flash_loan(
        &ctx.accounts.flash_loan_provider,
        &ctx.accounts.user_token_account,
        flash_loan_amount,
    )?;

    // Step 2: Execute arbitrage routes
    let mut current_amount = flash_loan_amount;
    let mut total_fees = 0u64;

    for (i, route) in routes.iter().enumerate() {
        let route_result = execute_route(
            route,
            current_amount,
            config.max_slippage,
            &ctx.remaining_accounts[i * 4..(i + 1) * 4],
        )?;

        current_amount = route_result.output_amount;
        total_fees += route_result.fees_paid;

        emit!(RouteExecuted {
            route_index: i as u8,
            dex: route.dex.clone(),
            input_amount: route_result.input_amount,
            output_amount: route_result.output_amount,
            fees_paid: route_result.fees_paid,
        });
    }

    // Step 3: Calculate profit and repay flash loan
    let final_balance = current_amount;
    let flash_loan_fee = calculate_flash_loan_fee(flash_loan_amount, 30); // 0.3% fee
    let repay_amount = flash_loan_amount + flash_loan_fee;

    require!(
        final_balance >= repay_amount,
        FlashArbitrageError::InsufficientFunds
    );

    let gross_profit = final_balance - repay_amount;
    require!(
        gross_profit >= min_profit,
        FlashArbitrageError::ProfitTooLow
    );

    // Calculate program fee
    let program_fee = calculate_program_fee(gross_profit, config.fee_rate);
    let net_profit = gross_profit - program_fee;

    // Step 4: Repay flash loan
    repay_flash_loan(
        &ctx.accounts.flash_loan_provider,
        &ctx.accounts.user_token_account,
        repay_amount,
    )?;

    // Step 5: Collect program fee
    if program_fee > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.fee_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            program_fee,
        )?;
    }

    // Update state
    arbitrage_state.status = ArbitrageStatus::Completed;
    arbitrage_state.end_time = Clock::get()?.unix_timestamp;
    arbitrage_state.gross_profit = gross_profit;
    arbitrage_state.net_profit = net_profit;
    arbitrage_state.fees_paid = total_fees + flash_loan_fee + program_fee;

    // Update global stats
    let config = &mut ctx.accounts.config;
    config.total_volume += flash_loan_amount;
    config.total_fees_collected += program_fee;

    emit!(ArbitrageExecuted {
        user: ctx.accounts.user.key(),
        flash_loan_amount,
        gross_profit,
        net_profit,
        total_fees: total_fees + flash_loan_fee + program_fee,
        routes_count: routes.len() as u8,
    });

    Ok(())
}

/// Update config handler
pub fn update_config_handler(
    ctx: Context<UpdateConfig>,
    fee_rate: Option<u16>,
    max_slippage: Option<u16>,
    is_paused: Option<bool>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    if let Some(rate) = fee_rate {
        require!(rate <= 1000, FlashArbitrageError::FeeTooHigh); // Max 10%
        config.fee_rate = rate;
    }

    if let Some(slippage) = max_slippage {
        require!(slippage <= 5000, FlashArbitrageError::SlippageTooHigh); // Max 50%
        config.max_slippage = slippage;
    }

    if let Some(paused) = is_paused {
        config.is_paused = paused;
    }

    emit!(ConfigUpdated {
        authority: ctx.accounts.authority.key(),
        fee_rate: config.fee_rate,
        max_slippage: config.max_slippage,
        is_paused: config.is_paused,
    });

    Ok(())
}

/// Withdraw fees handler
pub fn withdraw_fees_handler(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
    use anchor_spl::token::{self, Transfer};

    let config = &ctx.accounts.config;
    let available_balance = ctx.accounts.fee_account.amount;

    require!(
        amount <= available_balance,
        FlashArbitrageError::InsufficientFunds
    );

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.fee_account.to_account_info(),
                to: ctx.accounts.destination_account.to_account_info(),
                authority: ctx.accounts.config.to_account_info(),
            },
            &[&[b"config", &[config.bump]]],
        ),
        amount,
    )?;

    emit!(FeesWithdrawn {
        authority: ctx.accounts.authority.key(),
        amount,
        destination: ctx.accounts.destination_account.key(),
    });

    Ok(())
}

/// Emergency pause handler
pub fn emergency_pause_handler(ctx: Context<EmergencyPause>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.is_paused = true;

    emit!(EmergencyPauseActivated {
        authority: ctx.accounts.authority.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
