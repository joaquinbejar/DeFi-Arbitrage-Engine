-- Initial schema for Solana DeFi Arbitrage Engine
-- TimescaleDB migration

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb;

-- Create custom types
CREATE TYPE opportunity_type AS ENUM ('direct', 'triangular', 'cross_dex');
CREATE TYPE trade_action AS ENUM ('buy', 'sell', 'swap');
CREATE TYPE trade_status AS ENUM ('pending', 'executing', 'completed', 'failed', 'cancelled');
CREATE TYPE dex_type AS ENUM ('raydium', 'orca', 'meteora', 'jupiter');

-- Tokens table
CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,
    mint_address VARCHAR(44) UNIQUE NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    decimals INTEGER NOT NULL,
    logo_uri TEXT,
    coingecko_id VARCHAR(100),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for tokens
CREATE INDEX idx_tokens_mint_address ON tokens(mint_address);
CREATE INDEX idx_tokens_symbol ON tokens(symbol);
CREATE INDEX idx_tokens_active ON tokens(is_active);

-- DEX pools table
CREATE TABLE dex_pools (
    id SERIAL PRIMARY KEY,
    pool_address VARCHAR(44) UNIQUE NOT NULL,
    dex dex_type NOT NULL,
    token_a_mint VARCHAR(44) NOT NULL,
    token_b_mint VARCHAR(44) NOT NULL,
    token_a_reserve DECIMAL(20, 6) NOT NULL DEFAULT 0,
    token_b_reserve DECIMAL(20, 6) NOT NULL DEFAULT 0,
    fee_rate DECIMAL(8, 6) NOT NULL DEFAULT 0,
    liquidity DECIMAL(20, 6) NOT NULL DEFAULT 0,
    volume_24h DECIMAL(20, 6) DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for dex_pools
CREATE INDEX idx_dex_pools_address ON dex_pools(pool_address);
CREATE INDEX idx_dex_pools_dex ON dex_pools(dex);
CREATE INDEX idx_dex_pools_tokens ON dex_pools(token_a_mint, token_b_mint);
CREATE INDEX idx_dex_pools_active ON dex_pools(is_active);
CREATE INDEX idx_dex_pools_liquidity ON dex_pools(liquidity DESC);

-- Price feeds table (time-series)
CREATE TABLE price_feeds (
    time TIMESTAMPTZ NOT NULL,
    token_mint VARCHAR(44) NOT NULL,
    price DECIMAL(20, 8) NOT NULL,
    volume_24h DECIMAL(20, 6) DEFAULT 0,
    market_cap DECIMAL(20, 2) DEFAULT 0,
    source VARCHAR(50) NOT NULL,
    PRIMARY KEY (time, token_mint, source)
);

-- Convert price_feeds to hypertable
SELECT create_hypertable('price_feeds', 'time');

-- Create indexes for price_feeds
CREATE INDEX idx_price_feeds_token ON price_feeds(token_mint, time DESC);
CREATE INDEX idx_price_feeds_source ON price_feeds(source, time DESC);

-- Arbitrage opportunities table (time-series)
CREATE TABLE arbitrage_opportunities (
    time TIMESTAMPTZ NOT NULL,
    id UUID NOT NULL,
    opportunity_type opportunity_type NOT NULL,
    token_in VARCHAR(44) NOT NULL,
    token_out VARCHAR(44) NOT NULL,
    amount_in DECIMAL(20, 6) NOT NULL,
    expected_amount_out DECIMAL(20, 6) NOT NULL,
    profit_amount DECIMAL(20, 6) NOT NULL,
    profit_percentage DECIMAL(8, 4) NOT NULL,
    gas_cost DECIMAL(20, 6) NOT NULL,
    net_profit DECIMAL(20, 6) NOT NULL,
    dex_path TEXT[] NOT NULL,
    pool_addresses TEXT[] NOT NULL,
    price_impact DECIMAL(8, 4) NOT NULL,
    confidence_score DECIMAL(4, 2) NOT NULL,
    is_executed BOOLEAN DEFAULT false,
    execution_time TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (time, id)
);

-- Convert arbitrage_opportunities to hypertable
SELECT create_hypertable('arbitrage_opportunities', 'time');

-- Create indexes for arbitrage_opportunities
CREATE INDEX idx_opportunities_type ON arbitrage_opportunities(opportunity_type, time DESC);
CREATE INDEX idx_opportunities_profit ON arbitrage_opportunities(profit_percentage DESC, time DESC);
CREATE INDEX idx_opportunities_executed ON arbitrage_opportunities(is_executed, time DESC);
CREATE INDEX idx_opportunities_tokens ON arbitrage_opportunities(token_in, token_out, time DESC);

-- Trades table (time-series)
CREATE TABLE trades (
    time TIMESTAMPTZ NOT NULL,
    id UUID NOT NULL,
    opportunity_id UUID,
    signature VARCHAR(88) UNIQUE NOT NULL,
    status trade_status NOT NULL DEFAULT 'pending',
    dex dex_type NOT NULL,
    action trade_action NOT NULL,
    token_in VARCHAR(44) NOT NULL,
    token_out VARCHAR(44) NOT NULL,
    amount_in DECIMAL(20, 6) NOT NULL,
    amount_out DECIMAL(20, 6),
    expected_amount_out DECIMAL(20, 6) NOT NULL,
    slippage DECIMAL(8, 4),
    gas_used DECIMAL(20, 6),
    gas_price DECIMAL(20, 6),
    profit_loss DECIMAL(20, 6),
    execution_time_ms INTEGER,
    block_slot BIGINT,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    PRIMARY KEY (time, id)
);

-- Convert trades to hypertable
SELECT create_hypertable('trades', 'time');

-- Create indexes for trades
CREATE INDEX idx_trades_signature ON trades(signature);
CREATE INDEX idx_trades_status ON trades(status, time DESC);
CREATE INDEX idx_trades_opportunity ON trades(opportunity_id, time DESC);
CREATE INDEX idx_trades_dex ON trades(dex, time DESC);
CREATE INDEX idx_trades_tokens ON trades(token_in, token_out, time DESC);
CREATE INDEX idx_trades_profit ON trades(profit_loss DESC, time DESC) WHERE profit_loss IS NOT NULL;

-- Performance metrics table (time-series)
CREATE TABLE performance_metrics (
    time TIMESTAMPTZ NOT NULL,
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(20, 6) NOT NULL,
    tags JSONB DEFAULT '{}',
    PRIMARY KEY (time, metric_name)
);

-- Convert performance_metrics to hypertable
SELECT create_hypertable('performance_metrics', 'time');

-- Create indexes for performance_metrics
CREATE INDEX idx_performance_metrics_name ON performance_metrics(metric_name, time DESC);
CREATE INDEX idx_performance_metrics_tags ON performance_metrics USING GIN(tags);

-- Wallet balances table (time-series)
CREATE TABLE wallet_balances (
    time TIMESTAMPTZ NOT NULL,
    wallet_address VARCHAR(44) NOT NULL,
    token_mint VARCHAR(44) NOT NULL,
    balance DECIMAL(20, 6) NOT NULL,
    usd_value DECIMAL(20, 2),
    PRIMARY KEY (time, wallet_address, token_mint)
);

-- Convert wallet_balances to hypertable
SELECT create_hypertable('wallet_balances', 'time');

-- Create indexes for wallet_balances
CREATE INDEX idx_wallet_balances_address ON wallet_balances(wallet_address, time DESC);
CREATE INDEX idx_wallet_balances_token ON wallet_balances(token_mint, time DESC);

-- System logs table (time-series)
CREATE TABLE system_logs (
    time TIMESTAMPTZ NOT NULL,
    level VARCHAR(10) NOT NULL,
    component VARCHAR(50) NOT NULL,
    message TEXT NOT NULL,
    metadata JSONB DEFAULT '{}',
    PRIMARY KEY (time, component)
);

-- Convert system_logs to hypertable
SELECT create_hypertable('system_logs', 'time');

-- Create indexes for system_logs
CREATE INDEX idx_system_logs_level ON system_logs(level, time DESC);
CREATE INDEX idx_system_logs_component ON system_logs(component, time DESC);
CREATE INDEX idx_system_logs_metadata ON system_logs USING GIN(metadata);

-- Configuration table
CREATE TABLE configuration (
    id SERIAL PRIMARY KEY,
    key VARCHAR(100) UNIQUE NOT NULL,
    value JSONB NOT NULL,
    description TEXT,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create index for configuration
CREATE INDEX idx_configuration_key ON configuration(key);
CREATE INDEX idx_configuration_active ON configuration(is_active);

-- Insert default tokens
INSERT INTO tokens (mint_address, symbol, name, decimals, coingecko_id) VALUES
('So11111111111111111111111111111111111111112', 'SOL', 'Solana', 9, 'solana'),
('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 'USDC', 'USD Coin', 6, 'usd-coin'),
('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB', 'USDT', 'Tether USD', 6, 'tether'),
('4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R', 'RAY', 'Raydium', 6, 'raydium'),
('orcaEKTdK7LKz57vaAYr9QeNsVEPfiu6QeMU1kektZE', 'ORCA', 'Orca', 6, 'orca'),
('SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt', 'SRM', 'Serum', 6, 'serum'),
('mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So', 'mSOL', 'Marinade Staked SOL', 9, 'marinade-staked-sol'),
('7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj', 'stSOL', 'Lido Staked SOL', 9, 'lido-staked-sol');

-- Insert default configuration
INSERT INTO configuration (key, value, description) VALUES
('max_position_size', '10000.0', 'Maximum position size in USD'),
('min_profit_threshold', '0.005', 'Minimum profit threshold (0.5%)'),
('max_slippage', '0.01', 'Maximum allowed slippage (1%)'),
('gas_price_multiplier', '1.2', 'Gas price multiplier for priority'),
('max_concurrent_trades', '5', 'Maximum concurrent trades'),
('circuit_breaker_enabled', 'true', 'Enable circuit breaker'),
('max_daily_loss', '1000.0', 'Maximum daily loss in USD'),
('risk_score_threshold', '0.7', 'Minimum risk score for execution');

-- Create materialized views for analytics
CREATE MATERIALIZED VIEW hourly_trading_stats AS
SELECT 
    time_bucket('1 hour', time) AS hour,
    COUNT(*) AS total_trades,
    COUNT(*) FILTER (WHERE status = 'completed') AS successful_trades,
    COUNT(*) FILTER (WHERE status = 'failed') AS failed_trades,
    AVG(profit_loss) FILTER (WHERE profit_loss IS NOT NULL) AS avg_profit,
    SUM(profit_loss) FILTER (WHERE profit_loss IS NOT NULL) AS total_profit,
    AVG(execution_time_ms) FILTER (WHERE execution_time_ms IS NOT NULL) AS avg_execution_time
FROM trades
GROUP BY hour
ORDER BY hour DESC;

-- Create index for materialized view
CREATE INDEX idx_hourly_trading_stats_hour ON hourly_trading_stats(hour DESC);

-- Create materialized view for opportunity analytics
CREATE MATERIALIZED VIEW hourly_opportunity_stats AS
SELECT 
    time_bucket('1 hour', time) AS hour,
    opportunity_type,
    COUNT(*) AS total_opportunities,
    COUNT(*) FILTER (WHERE is_executed = true) AS executed_opportunities,
    AVG(profit_percentage) AS avg_profit_percentage,
    AVG(confidence_score) AS avg_confidence_score,
    AVG(price_impact) AS avg_price_impact
FROM arbitrage_opportunities
GROUP BY hour, opportunity_type
ORDER BY hour DESC, opportunity_type;

-- Create index for opportunity stats
CREATE INDEX idx_hourly_opportunity_stats_hour ON hourly_opportunity_stats(hour DESC, opportunity_type);

-- Create function to refresh materialized views
CREATE OR REPLACE FUNCTION refresh_analytics_views()
RETURNS void AS $$
BEGIN
    REFRESH MATERIALIZED VIEW CONCURRENTLY hourly_trading_stats;
    REFRESH MATERIALIZED VIEW CONCURRENTLY hourly_opportunity_stats;
END;
$$ LANGUAGE plpgsql;

-- Create function to clean old data
CREATE OR REPLACE FUNCTION cleanup_old_data(retention_days INTEGER DEFAULT 30)
RETURNS void AS $$
BEGIN
    -- Clean old price feeds (keep 30 days)
    DELETE FROM price_feeds WHERE time < NOW() - INTERVAL '1 day' * retention_days;
    
    -- Clean old system logs (keep 7 days)
    DELETE FROM system_logs WHERE time < NOW() - INTERVAL '7 days';
    
    -- Clean old performance metrics (keep 30 days)
    DELETE FROM performance_metrics WHERE time < NOW() - INTERVAL '1 day' * retention_days;
    
    -- Archive old opportunities (keep 90 days)
    DELETE FROM arbitrage_opportunities WHERE time < NOW() - INTERVAL '90 days';
    
    -- Archive old trades (keep 90 days)
    DELETE FROM trades WHERE time < NOW() - INTERVAL '90 days';
END;
$$ LANGUAGE plpgsql;

-- Create function to update token prices
CREATE OR REPLACE FUNCTION update_token_price(
    p_token_mint VARCHAR(44),
    p_price DECIMAL(20, 8),
    p_volume_24h DECIMAL(20, 6) DEFAULT 0,
    p_market_cap DECIMAL(20, 2) DEFAULT 0,
    p_source VARCHAR(50) DEFAULT 'api'
)
RETURNS void AS $$
BEGIN
    INSERT INTO price_feeds (time, token_mint, price, volume_24h, market_cap, source)
    VALUES (NOW(), p_token_mint, p_price, p_volume_24h, p_market_cap, p_source)
    ON CONFLICT (time, token_mint, source) DO UPDATE SET
        price = EXCLUDED.price,
        volume_24h = EXCLUDED.volume_24h,
        market_cap = EXCLUDED.market_cap;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply triggers to tables with updated_at column
CREATE TRIGGER update_tokens_updated_at BEFORE UPDATE ON tokens
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_dex_pools_updated_at BEFORE UPDATE ON dex_pools
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_configuration_updated_at BEFORE UPDATE ON configuration
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Grant permissions to application user (adjust username as needed)
-- GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO arbitrage_user;
-- GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO arbitrage_user;
-- GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO arbitrage_user;