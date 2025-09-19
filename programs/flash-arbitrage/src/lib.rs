//! Flash Arbitrage Program
//!
//! This Anchor program enables atomic arbitrage execution with flash loans,
//! allowing traders to exploit price differences across DEXes without
//! requiring upfront capital.

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod flash_arbitrage {
    use super::*;

    /// Initialize the flash arbitrage program
    pub fn initialize(
        ctx: Context<Initialize>,
        fee_rate: u16,     // Fee rate in basis points (e.g., 30 = 0.3%)
        max_slippage: u16, // Max slippage in basis points
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

    /// Execute flash arbitrage across multiple DEXes
    pub fn execute_flash_arbitrage(
        ctx: Context<ExecuteFlashArbitrage>,
        flash_loan_amount: u64,
        min_profit: u64,
        routes: Vec<ArbitrageRoute>,
    ) -> Result<()> {
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

    /// Update program configuration (admin only)
    pub fn update_config(
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

    /// Withdraw collected fees (admin only)
    pub fn withdraw_fees(ctx: Context<WithdrawFees>, amount: u64) -> Result<()> {
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

    /// Emergency pause (admin only)
    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.is_paused = true;

        emit!(EmergencyPauseActivated {
            authority: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

// Helper functions
fn execute_flash_loan(
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

fn repay_flash_loan(
    _provider: &AccountInfo,
    _token_account: &Account<TokenAccount>,
    _amount: u64,
) -> Result<()> {
    // In a real implementation, this would repay the flash loan
    Ok(())
}

fn execute_route(
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

fn execute_raydium_swap(
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

fn execute_orca_swap(_accounts: &[AccountInfo], input_amount: u64, min_output: u64) -> Result<u64> {
    // Simulate Orca swap
    let output = input_amount * 995 / 1000; // Simulate 0.5% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

fn execute_meteora_swap(
    _accounts: &[AccountInfo],
    input_amount: u64,
    min_output: u64,
) -> Result<u64> {
    // Simulate Meteora swap
    let output = input_amount * 997 / 1000; // Simulate 0.3% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

fn execute_jupiter_swap(
    _accounts: &[AccountInfo],
    input_amount: u64,
    min_output: u64,
) -> Result<u64> {
    // Simulate Jupiter swap
    let output = input_amount * 998 / 1000; // Simulate 0.2% slippage
    require!(output >= min_output, FlashArbitrageError::SlippageTooHigh);
    Ok(output)
}

fn calculate_flash_loan_fee(amount: u64, fee_bps: u16) -> u64 {
    amount * fee_bps as u64 / 10000
}

fn calculate_program_fee(profit: u64, fee_rate: u16) -> u64 {
    profit * fee_rate as u64 / 10000
}

fn calculate_min_output(_input: u64, expected: u64, max_slippage_bps: u16) -> u64 {
    expected * (10000 - max_slippage_bps as u64) / 10000
}

fn calculate_slippage(expected: u64, actual: u64) -> u16 {
    if actual >= expected {
        0
    } else {
        ((expected - actual) * 10000 / expected) as u16
    }
}

fn calculate_dex_fees(amount: u64, dex: &str) -> u64 {
    match dex {
        "raydium" => amount * 25 / 10000, // 0.25%
        "orca" => amount * 30 / 10000,    // 0.30%
        "meteora" => amount * 20 / 10000, // 0.20%
        "jupiter" => amount * 15 / 10000, // 0.15%
        _ => amount * 30 / 10000,         // Default 0.30%
    }
}

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

#[derive(Accounts)]
pub struct WithdrawFees<'info> {
    #[account(
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,

    #[account(mut)]
    pub fee_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub destination_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
    #[account(
        mut,
        seeds = [b"config"],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

// Data structs
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

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct ArbitrageRoute {
    #[max_len(20)]
    pub dex: String, // DEX name (e.g., "raydium", "orca")
    pub token_in: Pubkey,     // Input token mint
    pub token_out: Pubkey,    // Output token mint
    pub expected_output: u64, // Expected output amount
    pub pool_address: Pubkey, // Pool/market address
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum ArbitrageStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RouteResult {
    pub input_amount: u64,
    pub output_amount: u64,
    pub fees_paid: u64,
    pub slippage: u16,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct FlashLoanResult {
    pub amount_borrowed: u64,
    pub fee: u64,
    pub success: bool,
}

// Events
#[event]
pub struct ProgramInitialized {
    pub authority: Pubkey,
    pub fee_rate: u16,
    pub max_slippage: u16,
}

#[event]
pub struct ArbitrageExecuted {
    pub user: Pubkey,
    pub flash_loan_amount: u64,
    pub gross_profit: u64,
    pub net_profit: u64,
    pub total_fees: u64,
    pub routes_count: u8,
}

#[event]
pub struct RouteExecuted {
    pub route_index: u8,
    pub dex: String,
    pub input_amount: u64,
    pub output_amount: u64,
    pub fees_paid: u64,
}

#[event]
pub struct ConfigUpdated {
    pub authority: Pubkey,
    pub fee_rate: u16,
    pub max_slippage: u16,
    pub is_paused: bool,
}

#[event]
pub struct FeesWithdrawn {
    pub authority: Pubkey,
    pub amount: u64,
    pub destination: Pubkey,
}

#[event]
pub struct EmergencyPauseActivated {
    pub authority: Pubkey,
    pub timestamp: i64,
}

// Errors
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
