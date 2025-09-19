//! Helper functions for the Cross-DEX Router Program

use crate::models::*;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use std::str::FromStr;

/// Find optimal route across DEXes
pub fn find_optimal_route(
    input_token: Pubkey,
    output_token: Pubkey,
    input_amount: u64,
    max_hops: u8,
    preferred_dexes: &[String],
    _accounts: &[AccountInfo],
) -> Result<OptimalRoute> {
    // Implement Dijkstra's algorithm or similar pathfinding
    // This is a simplified implementation

    let mut best_route = OptimalRoute {
        hops: Vec::new(),
        expected_output: 0,
        total_fees: 0,
        price_impact: 0,
    };

    // Direct route (1 hop)
    if let Some(direct_hop) =
        find_direct_route(input_token, output_token, input_amount, preferred_dexes)?
    {
        best_route.hops.push(direct_hop.clone());
        best_route.expected_output = direct_hop.expected_output;
        best_route.total_fees = direct_hop.fees;
        best_route.price_impact = direct_hop.price_impact;
    }

    // Multi-hop routes (up to max_hops)
    if max_hops > 1 {
        let multi_hop_route = find_multi_hop_route(
            input_token,
            output_token,
            input_amount,
            max_hops,
            preferred_dexes,
        )?;

        if multi_hop_route.expected_output > best_route.expected_output {
            best_route = multi_hop_route;
        }
    }

    require!(!best_route.hops.is_empty(), CrossDexError::NoRouteFound);

    Ok(best_route)
}

/// Find direct route between two tokens
pub fn find_direct_route(
    input_token: Pubkey,
    output_token: Pubkey,
    input_amount: u64,
    preferred_dexes: &[String],
) -> Result<Option<RouteHop>> {
    // Check each DEX for direct trading pair
    let dexes = if preferred_dexes.is_empty() {
        vec![
            "raydium".to_string(),
            "orca".to_string(),
            "meteora".to_string(),
        ]
    } else {
        preferred_dexes.to_vec()
    };

    let mut best_hop: Option<RouteHop> = None;

    for dex in dexes {
        if let Some(hop) = simulate_dex_swap(&dex, input_token, output_token, input_amount)? {
            if best_hop.is_none()
                || hop.expected_output > best_hop.as_ref().unwrap().expected_output
            {
                best_hop = Some(hop);
            }
        }
    }

    Ok(best_hop)
}

/// Find multi-hop route through intermediate tokens
pub fn find_multi_hop_route(
    input_token: Pubkey,
    output_token: Pubkey,
    input_amount: u64,
    _max_hops: u8,
    preferred_dexes: &[String],
) -> Result<OptimalRoute> {
    // Simplified multi-hop pathfinding
    // In a real implementation, this would use graph algorithms

    let intermediate_tokens = get_popular_intermediate_tokens();
    let mut best_route = OptimalRoute {
        hops: Vec::new(),
        expected_output: 0,
        total_fees: 0,
        price_impact: 0,
    };

    // Try 2-hop routes through popular intermediate tokens
    for intermediate in intermediate_tokens {
        if intermediate == input_token || intermediate == output_token {
            continue;
        }

        // First hop: input_token -> intermediate
        if let Some(hop1) =
            find_direct_route(input_token, intermediate, input_amount, preferred_dexes)?
        {
            // Second hop: intermediate -> output_token
            if let Some(hop2) = find_direct_route(
                intermediate,
                output_token,
                hop1.expected_output,
                preferred_dexes,
            )? {
                let total_output = hop2.expected_output;
                if total_output > best_route.expected_output {
                    best_route.hops = vec![hop1, hop2];
                    best_route.expected_output = total_output;
                    best_route.total_fees = best_route.hops.iter().map(|h| h.fees).sum();
                    best_route.price_impact = calculate_combined_price_impact(&best_route.hops);
                }
            }
        }
    }

    Ok(best_route)
}

/// Simulate swap on specific DEX
pub fn simulate_dex_swap(
    dex: &str,
    input_token: Pubkey,
    output_token: Pubkey,
    input_amount: u64,
) -> Result<Option<RouteHop>> {
    // Simulate swap on specific DEX
    // In a real implementation, this would query actual pool data

    let (fee_rate, base_slippage) = match dex {
        "raydium" => (25, 50), // 0.25% fee, 0.5% base slippage
        "orca" => (30, 30),    // 0.30% fee, 0.3% base slippage
        "meteora" => (20, 40), // 0.20% fee, 0.4% base slippage
        "jupiter" => (15, 20), // 0.15% fee, 0.2% base slippage
        _ => return Ok(None),
    };

    // Simulate price calculation
    let fees = input_amount * fee_rate / 10000;
    let amount_after_fees = input_amount - fees;
    let slippage_amount = amount_after_fees * base_slippage / 10000;
    let expected_output = amount_after_fees - slippage_amount;

    // Check if pair exists (simplified)
    if input_token == output_token {
        return Ok(None);
    }

    Ok(Some(RouteHop {
        dex: dex.to_string(),
        input_token,
        output_token,
        input_amount,
        expected_output,
        fees,
        price_impact: base_slippage as u16,
        pool_address: Pubkey::default(), // Would be actual pool address
    }))
}

/// Execute route hops
#[allow(clippy::too_many_arguments)]
pub fn execute_route_hops(
    route: &OptimalRoute,
    input_amount: u64,
    max_slippage: u16,
    _user: &Signer,
    _input_account: &Account<TokenAccount>,
    _output_account: &Account<TokenAccount>,
    _token_program: &Program<Token>,
    _remaining_accounts: &[AccountInfo],
) -> Result<RouteExecutionResult> {
    let mut current_amount = input_amount;
    let mut total_fees = 0u64;

    for (i, hop) in route.hops.iter().enumerate() {
        let min_output = calculate_min_output_with_slippage(hop.expected_output, max_slippage);

        // Execute swap on specific DEX
        let swap_result = execute_dex_swap(&hop.dex, current_amount, min_output, hop.pool_address)?;

        current_amount = swap_result.output_amount;
        total_fees += swap_result.fees;

        emit!(HopExecuted {
            hop_index: i as u8,
            dex: hop.dex.clone(),
            input_amount: swap_result.input_amount,
            output_amount: swap_result.output_amount,
            fees: swap_result.fees,
        });
    }

    Ok(RouteExecutionResult {
        output_amount: current_amount,
        total_fees,
        hops_executed: route.hops.len() as u8,
    })
}

/// Execute swap on specific DEX
pub fn execute_dex_swap(
    dex: &str,
    input_amount: u64,
    min_output: u64,
    _pool_address: Pubkey,
) -> Result<SwapResult> {
    // Execute actual swap on DEX
    // This would call the specific DEX's swap instruction

    let (fee_rate, slippage) = match dex {
        "raydium" => (25, 50),
        "orca" => (30, 30),
        "meteora" => (20, 40),
        "jupiter" => (15, 20),
        _ => return Err(CrossDexError::UnsupportedDex.into()),
    };

    let fees = input_amount * fee_rate / 10000;
    let amount_after_fees = input_amount - fees;
    let slippage_amount = amount_after_fees * slippage / 10000;
    let output_amount = amount_after_fees - slippage_amount;

    require!(output_amount >= min_output, CrossDexError::SlippageTooHigh);

    Ok(SwapResult {
        input_amount,
        output_amount,
        fees,
    })
}

/// Get popular intermediate tokens for multi-hop routing
pub fn get_popular_intermediate_tokens() -> Vec<Pubkey> {
    // Return list of popular intermediate tokens (USDC, SOL, etc.)
    vec![
        // USDC
        Pubkey::from_str("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v").unwrap(),
        // WSOL
        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
        // USDT
        Pubkey::from_str("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB").unwrap(),
    ]
}

/// Calculate routing fee
pub fn calculate_routing_fee(amount: u64, fee_rate: u16) -> u64 {
    amount * fee_rate as u64 / 10000
}

/// Calculate total slippage
pub fn calculate_total_slippage(expected: u64, actual: u64) -> u16 {
    if actual >= expected {
        0
    } else {
        ((expected - actual) * 10000 / expected) as u16
    }
}

/// Calculate total route fees
pub fn calculate_total_route_fees(route: &OptimalRoute) -> u64 {
    route.hops.iter().map(|hop| hop.fees).sum()
}

/// Calculate total price impact
pub fn calculate_total_price_impact(route: &OptimalRoute) -> u16 {
    // Simplified price impact calculation
    route
        .hops
        .iter()
        .map(|hop| hop.price_impact as u32)
        .sum::<u32>() as u16
}

/// Calculate combined price impact for multi-hop route
pub fn calculate_combined_price_impact(hops: &[RouteHop]) -> u16 {
    // Calculate combined price impact for multi-hop route
    hops.iter().map(|hop| hop.price_impact as u32).sum::<u32>() as u16
}

/// Calculate minimum output with slippage protection
pub fn calculate_min_output_with_slippage(expected: u64, slippage_bps: u16) -> u64 {
    expected * (10000 - slippage_bps as u64) / 10000
}

// Handler functions for lib.rs

/// Initialize the cross-DEX router program
pub fn initialize_router(
    ctx: Context<Initialize>,
    max_hops: u8,
    default_slippage: u16,
    routing_fee: u16,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.authority = ctx.accounts.authority.key();
    config.max_hops = max_hops;
    config.default_slippage = default_slippage;
    config.routing_fee = routing_fee;
    config.total_routes_executed = 0;
    config.total_volume = 0;
    config.total_fees_collected = 0;
    config.is_active = true;
    config.bump = ctx.bumps.config;

    emit!(RouterInitialized {
        authority: config.authority,
        max_hops,
        default_slippage,
        routing_fee,
    });

    Ok(())
}

/// Execute optimal route handler
pub fn execute_optimal_route_handler(
    ctx: Context<ExecuteOptimalRoute>,
    input_amount: u64,
    min_output_amount: u64,
    max_slippage: Option<u16>,
    preferred_dexes: Vec<String>,
) -> Result<()> {
    use anchor_spl::token::{self, Transfer};

    let config = &ctx.accounts.config;

    // Check if router is active
    require!(config.is_active, CrossDexError::RouterInactive);

    // Validate input parameters
    require!(input_amount > 0, CrossDexError::InvalidAmount);
    require!(min_output_amount > 0, CrossDexError::InvalidAmount);

    let slippage = max_slippage.unwrap_or(config.default_slippage);
    require!(slippage <= 5000, CrossDexError::SlippageTooHigh); // Max 50%

    let route_state = &mut ctx.accounts.route_state;
    route_state.user = ctx.accounts.user.key();
    route_state.input_token = ctx.accounts.input_token_account.mint;
    route_state.output_token = ctx.accounts.output_token_account.mint;
    route_state.input_amount = input_amount;
    route_state.min_output_amount = min_output_amount;
    route_state.max_slippage = slippage;
    route_state.status = RouteStatus::Finding;
    route_state.start_time = Clock::get()?.unix_timestamp;

    // Find optimal route using pathfinding algorithm
    let optimal_route = find_optimal_route(
        route_state.input_token,
        route_state.output_token,
        input_amount,
        config.max_hops,
        &preferred_dexes,
        ctx.remaining_accounts,
    )?;

    // Validate route profitability
    require!(
        optimal_route.expected_output >= min_output_amount,
        CrossDexError::RouteNotProfitable
    );

    route_state.route = optimal_route.clone();
    route_state.status = RouteStatus::Executing;

    // Execute the route
    let execution_result = execute_route_hops(
        &optimal_route,
        input_amount,
        slippage,
        &ctx.accounts.user,
        &ctx.accounts.input_token_account,
        &ctx.accounts.output_token_account,
        &ctx.accounts.token_program,
        ctx.remaining_accounts,
    )?;

    // Calculate and collect routing fee
    let routing_fee = calculate_routing_fee(execution_result.output_amount, config.routing_fee);

    if routing_fee > 0 {
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.output_token_account.to_account_info(),
                    to: ctx.accounts.fee_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            routing_fee,
        )?;
    }

    let final_output = execution_result.output_amount - routing_fee;

    // Update route state
    route_state.status = RouteStatus::Completed;
    route_state.end_time = Clock::get()?.unix_timestamp;
    route_state.actual_output = final_output;
    route_state.total_fees = execution_result.total_fees + routing_fee;
    route_state.actual_slippage =
        calculate_total_slippage(optimal_route.expected_output, final_output);

    // Update global statistics
    let config = &mut ctx.accounts.config;
    config.total_routes_executed += 1;
    config.total_volume += input_amount;
    config.total_fees_collected += routing_fee;

    emit!(RouteExecuted {
        user: ctx.accounts.user.key(),
        input_token: route_state.input_token,
        output_token: route_state.output_token,
        input_amount,
        output_amount: final_output,
        hops_count: optimal_route.hops.len() as u8,
        total_fees: route_state.total_fees,
        execution_time: route_state.end_time - route_state.start_time,
    });

    Ok(())
}

/// Get route quote handler
pub fn get_route_quote_handler(
    ctx: Context<GetRouteQuote>,
    input_amount: u64,
    preferred_dexes: Vec<String>,
) -> Result<()> {
    let config = &ctx.accounts.config;

    require!(config.is_active, CrossDexError::RouterInactive);
    require!(input_amount > 0, CrossDexError::InvalidAmount);

    let quote_state = &mut ctx.accounts.quote_state;
    quote_state.input_token = ctx.accounts.input_token_mint.key();
    quote_state.output_token = ctx.accounts.output_token_mint.key();
    quote_state.input_amount = input_amount;

    // Find optimal route for quote
    let optimal_route = find_optimal_route(
        quote_state.input_token,
        quote_state.output_token,
        input_amount,
        config.max_hops,
        &preferred_dexes,
        ctx.remaining_accounts,
    )?;

    quote_state.route = optimal_route.clone();
    quote_state.expected_output = optimal_route.expected_output;
    quote_state.estimated_fees = calculate_total_route_fees(&optimal_route);
    quote_state.price_impact = calculate_total_price_impact(&optimal_route);
    quote_state.timestamp = Clock::get()?.unix_timestamp;

    emit!(QuoteGenerated {
        input_token: quote_state.input_token,
        output_token: quote_state.output_token,
        input_amount,
        expected_output: quote_state.expected_output,
        hops_count: optimal_route.hops.len() as u8,
        estimated_fees: quote_state.estimated_fees,
        price_impact: quote_state.price_impact,
    });

    Ok(())
}

/// Update DEX metrics handler
pub fn update_dex_metrics_handler(
    ctx: Context<UpdateDexMetrics>,
    volume: u64,
    swap_count: u32,
    success_rate: u16,
    average_slippage: u16,
) -> Result<()> {
    let dex_registry = &mut ctx.accounts.dex_registry;

    dex_registry.total_volume += volume;
    dex_registry.total_swaps += swap_count;
    dex_registry.success_rate = success_rate;
    dex_registry.average_slippage = average_slippage;
    dex_registry.last_updated = Clock::get()?.unix_timestamp;

    emit!(DexMetricsUpdated {
        dex_name: dex_registry.dex_info.name.clone(),
        total_volume: dex_registry.total_volume,
        total_swaps: dex_registry.total_swaps,
        success_rate,
        average_slippage,
    });

    Ok(())
}

/// Update config handler
pub fn update_config_handler(
    ctx: Context<UpdateConfig>,
    max_hops: Option<u8>,
    default_slippage: Option<u16>,
    routing_fee: Option<u16>,
    is_active: Option<bool>,
) -> Result<()> {
    let config = &mut ctx.accounts.config;

    if let Some(hops) = max_hops {
        require!(hops <= 10, CrossDexError::TooManyHops);
        config.max_hops = hops;
    }

    if let Some(slippage) = default_slippage {
        require!(slippage <= 5000, CrossDexError::SlippageTooHigh);
        config.default_slippage = slippage;
    }

    if let Some(fee) = routing_fee {
        require!(fee <= 1000, CrossDexError::FeeTooHigh); // Max 10%
        config.routing_fee = fee;
    }

    if let Some(active) = is_active {
        config.is_active = active;
    }

    emit!(ConfigUpdated {
        authority: ctx.accounts.authority.key(),
        max_hops: config.max_hops,
        default_slippage: config.default_slippage,
        routing_fee: config.routing_fee,
        is_active: config.is_active,
    });

    Ok(())
}

/// Register a new DEX for routing
pub fn register_dex_handler(ctx: Context<RegisterDex>, dex_info: DexInfo) -> Result<()> {
    let dex_registry = &mut ctx.accounts.dex_registry;

    // Validate DEX info
    require!(!dex_info.name.is_empty(), CrossDexError::InvalidDexName);
    require!(dex_info.fee_rate <= 10000, CrossDexError::FeeTooHigh); // Max 100%
    require!(dex_info.is_active, CrossDexError::DexNotActive);

    dex_registry.dex_info = dex_info.clone();
    dex_registry.total_volume = 0;
    dex_registry.total_swaps = 0;
    dex_registry.success_rate = 10000; // 100% initially
    dex_registry.average_slippage = 0;
    dex_registry.last_updated = Clock::get()?.unix_timestamp;

    emit!(DexRegistered {
        dex_name: dex_info.name,
        program_id: dex_info.program_id,
        fee_rate: dex_info.fee_rate,
    });

    Ok(())
}
