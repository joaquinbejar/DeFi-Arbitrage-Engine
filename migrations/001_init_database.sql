-- DeFi Arbitrage Engine Database Schema
-- TimescaleDB initialization script

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pg_stat_statements";

-- Create custom types
CREATE TYPE trade_status AS ENUM ('pending', 'executing', 'completed', 'failed', 'cancelled');
CREATE TYPE opportunity_status AS ENUM ('detected', 'analyzing', 'executing', 'completed', 'expired', 'failed');
CREATE TYPE dex_type AS ENUM ('raydium', 'orca', 'meteora', 'jupiter', 'serum', 'aldrin', 'saber', 'mercurial');
CREATE TYPE token_standard AS ENUM ('spl', 'spl2022');
CREATE TYPE risk_level AS ENUM ('low', 'medium', 'high', 'critical');

-- Tokens table
CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    mint_address VARCHAR(44) UNIQUE NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    decimals INTEGER NOT NULL CHECK (decimals >= 0 AND decimals <= 18),
    standard token_standard DEFAULT 'spl',
    logo_uri TEXT,
    coingecko_id VARCHAR(100),
    is_verified BOOLEAN DEFAULT false,
    is_active BOOLEAN DEFAULT true,
    daily_volume_usd DECIMAL(20, 6) DEFAULT 0,
    market_cap_usd DECIMAL(20, 6) DEFAULT 0,
    price_usd DECIMAL(20, 10) DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- DEX pools table
CREATE TABLE dex_pools (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pool_address VARCHAR(44) UNIQUE NOT NULL,
    dex dex_type NOT NULL,
    token_a_mint VARCHAR(44) NOT NULL,
    token_b_mint VARCHAR(44) NOT NULL,
    token_a_reserve DECIMAL(30, 6) NOT NULL DEFAULT 0,
    token_b_reserve DECIMAL(30, 6) NOT NULL DEFAULT 0,
    fee_rate DECIMAL(8, 6) NOT NULL DEFAULT 0,
    liquidity_usd DECIMAL(20, 6) DEFAULT 0,
    volume_24h_usd DECIMAL(20, 6) DEFAULT 0,
    price DECIMAL(20, 10) DEFAULT 0,
    is_active BOOLEAN DEFAULT true,
    last_updated TIMESTAMPTZ DEFAULT NOW(),
    created_at TIMESTAMPTZ DEFAULT NOW(),
    FOREIGN KEY (token_a_mint) REFERENCES tokens(mint_address),
    FOREIGN KEY (token_b_mint) REFERENCES tokens(mint_address)
);

-- Price feeds table (hypertable)
CREATE TABLE price_feeds (
    time TIMESTAMPTZ NOT NULL,
    token_mint VARCHAR(44) NOT NULL,
    dex dex_type NOT NULL,
    pool_address VARCHAR(44) NOT NULL,
    price DECIMAL(20, 10) NOT NULL,
    volume_24h DECIMAL(20, 6) DEFAULT 0,
    liquidity DECIMAL(20, 6) DEFAULT 0,
    bid_price DECIMAL(20, 10),
    ask_price DECIMAL(20, 10),
    spread DECIMAL(8, 6),
    source VARCHAR(50) DEFAULT 'websocket',
    FOREIGN KEY (token_mint) REFERENCES tokens(mint_address),
    FOREIGN KEY (pool_address) REFERENCES dex_pools(pool_address)
);

-- Convert to hypertable
SELECT create_hypertable('price_feeds', 'time');

-- Arbitrage opportunities table (hypertable)
CREATE TABLE arbitrage_opportunities (
    time TIMESTAMPTZ NOT NULL,
    id UUID DEFAULT uuid_generate_v4(),
    token_mint VARCHAR(44) NOT NULL,
    buy_dex dex_type NOT NULL,
    sell_dex dex_type NOT NULL,
    buy_pool VARCHAR(44) NOT NULL,
    sell_pool VARCHAR(44) NOT NULL,
    buy_price DECIMAL(20, 10) NOT NULL,
    sell_price DECIMAL(20, 10) NOT NULL,
    price_difference DECIMAL(8, 6) NOT NULL,
    profit_percentage DECIMAL(8, 6) NOT NULL,
    estimated_profit_usd DECIMAL(20, 6) NOT NULL,
    max_trade_size DECIMAL(30, 6) NOT NULL,
    gas_cost_estimate DECIMAL(20, 6) DEFAULT 0,
    net_profit_usd DECIMAL(20, 6) NOT NULL,
    confidence_score DECIMAL(3, 2) DEFAULT 0.5,
    risk_score DECIMAL(3, 2) DEFAULT 0.5,
    status opportunity_status DEFAULT 'detected',
    expires_at TIMESTAMPTZ,
    detected_at TIMESTAMPTZ DEFAULT NOW(),
    FOREIGN KEY (token_mint) REFERENCES tokens(mint_address),
    FOREIGN KEY (buy_pool) REFERENCES dex_pools(pool_address),
    FOREIGN KEY (sell_pool) REFERENCES dex_pools(pool_address)
);

-- Convert to hypertable
SELECT create_hypertable('arbitrage_opportunities', 'time');

-- Trades table (hypertable)
CREATE TABLE trades (
    time TIMESTAMPTZ NOT NULL,
    id UUID DEFAULT uuid_generate_v4(),
    opportunity_id UUID,
    transaction_signature VARCHAR(88) UNIQUE,
    token_mint VARCHAR(44) NOT NULL,
    buy_dex dex_type NOT NULL,
    sell_dex dex_type NOT NULL,
    buy_pool VARCHAR(44) NOT NULL,
    sell_pool VARCHAR(44) NOT NULL,
    trade_amount DECIMAL(30, 6) NOT NULL,
    buy_price DECIMAL(20, 10) NOT NULL,
    sell_price DECIMAL(20, 10) NOT NULL,
    expected_profit_usd DECIMAL(20, 6) NOT NULL,
    actual_profit_usd DECIMAL(20, 6),
    gas_cost_usd DECIMAL(20, 6) DEFAULT 0,
    slippage DECIMAL(8, 6) DEFAULT 0,
    execution_time_ms INTEGER,
    status trade_status DEFAULT 'pending',
    error_message TEXT,
    block_height BIGINT,
    slot BIGINT,
    started_at TIMESTAMPTZ DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    FOREIGN KEY (token_mint) REFERENCES tokens(mint_address),
    FOREIGN KEY (buy_pool) REFERENCES dex_pools(pool_address),
    FOREIGN KEY (sell_pool) REFERENCES dex_pools(pool_address)
);

-- Convert to hypertable
SELECT create_hypertable('trades', 'time');

-- Performance metrics table (hypertable)
CREATE TABLE performance_metrics (
    time TIMESTAMPTZ NOT NULL,
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL(20, 6) NOT NULL,
    metric_type VARCHAR(50) DEFAULT 'gauge',
    labels JSONB DEFAULT '{}',
    description TEXT
);

-- Convert to hypertable
SELECT create_hypertable('performance_metrics', 'time');

-- Risk events table (hypertable)
CREATE TABLE risk_events (
    time TIMESTAMPTZ NOT NULL,
    id UUID DEFAULT uuid_generate_v4(),
    event_type VARCHAR(100) NOT NULL,
    risk_level risk_level NOT NULL,
    token_mint VARCHAR(44),
    dex dex_type,
    pool_address VARCHAR(44),
    description TEXT NOT NULL,
    impact_score DECIMAL(3, 2) DEFAULT 0,
    mitigation_action TEXT,
    resolved_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}',
    FOREIGN KEY (token_mint) REFERENCES tokens(mint_address),
    FOREIGN KEY (pool_address) REFERENCES dex_pools(pool_address)
);

-- Convert to hypertable
SELECT create_hypertable('risk_events', 'time');

-- System logs table (hypertable)
CREATE TABLE system_logs (
    time TIMESTAMPTZ NOT NULL,
    level VARCHAR(10) NOT NULL,
    message TEXT NOT NULL,
    module VARCHAR(100),
    function VARCHAR(100),
    line_number INTEGER,
    thread_id VARCHAR(50),
    request_id UUID,
    metadata JSONB DEFAULT '{}'
);

-- Convert to hypertable
SELECT create_hypertable('system_logs', 'time');

-- Configuration table
CREATE TABLE configuration (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(200) UNIQUE NOT NULL,
    value TEXT NOT NULL,
    value_type VARCHAR(50) DEFAULT 'string',
    description TEXT,
    is_encrypted BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Wallet balances table (hypertable)
CREATE TABLE wallet_balances (
    time TIMESTAMPTZ NOT NULL,
    wallet_address VARCHAR(44) NOT NULL,
    token_mint VARCHAR(44) NOT NULL,
    balance DECIMAL(30, 6) NOT NULL,
    balance_usd DECIMAL(20, 6) DEFAULT 0,
    FOREIGN KEY (token_mint) REFERENCES tokens(mint_address)
);

-- Convert to hypertable
SELECT create_hypertable('wallet_balances', 'time');

-- Create indexes for better performance
CREATE INDEX idx_tokens_mint_address ON tokens(mint_address);
CREATE INDEX idx_tokens_symbol ON tokens(symbol);
CREATE INDEX idx_tokens_is_active ON tokens(is_active);

CREATE INDEX idx_dex_pools_address ON dex_pools(pool_address);
CREATE INDEX idx_dex_pools_dex ON dex_pools(dex);
CREATE INDEX idx_dex_pools_tokens ON dex_pools(token_a_mint, token_b_mint);
CREATE INDEX idx_dex_pools_active ON dex_pools(is_active);

CREATE INDEX idx_price_feeds_token_dex ON price_feeds(token_mint, dex, time DESC);
CREATE INDEX idx_price_feeds_pool ON price_feeds(pool_address, time DESC);

CREATE INDEX idx_opportunities_token ON arbitrage_opportunities(token_mint, time DESC);
CREATE INDEX idx_opportunities_status ON arbitrage_opportunities(status, time DESC);
CREATE INDEX idx_opportunities_profit ON arbitrage_opportunities(profit_percentage DESC, time DESC);

CREATE INDEX idx_trades_token ON trades(token_mint, time DESC);
CREATE INDEX idx_trades_status ON trades(status, time DESC);
CREATE INDEX idx_trades_signature ON trades(transaction_signature);
CREATE INDEX idx_trades_opportunity ON trades(opportunity_id);

CREATE INDEX idx_performance_metrics_name ON performance_metrics(metric_name, time DESC);
CREATE INDEX idx_risk_events_type ON risk_events(event_type, time DESC);
CREATE INDEX idx_risk_events_level ON risk_events(risk_level, time DESC);

CREATE INDEX idx_system_logs_level ON system_logs(level, time DESC);
CREATE INDEX idx_system_logs_module ON system_logs(module, time DESC);

CREATE INDEX idx_wallet_balances_wallet ON wallet_balances(wallet_address, time DESC);
CREATE INDEX idx_wallet_balances_token ON wallet_balances(token_mint, time DESC);

-- Create continuous aggregates for better query performance
CREATE MATERIALIZED VIEW price_feeds_1min
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 minute', time) AS bucket,
    token_mint,
    dex,
    pool_address,
    FIRST(price, time) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, time) AS close_price,
    AVG(price) AS avg_price,
    SUM(volume_24h) AS total_volume,
    AVG(liquidity) AS avg_liquidity,
    COUNT(*) AS tick_count
FROM price_feeds
GROUP BY bucket, token_mint, dex, pool_address;

CREATE MATERIALIZED VIEW trades_hourly
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', time) AS bucket,
    token_mint,
    buy_dex,
    sell_dex,
    COUNT(*) AS trade_count,
    SUM(trade_amount) AS total_volume,
    SUM(actual_profit_usd) AS total_profit,
    AVG(actual_profit_usd) AS avg_profit,
    AVG(execution_time_ms) AS avg_execution_time,
    COUNT(*) FILTER (WHERE status = 'completed') AS successful_trades,
    COUNT(*) FILTER (WHERE status = 'failed') AS failed_trades
FROM trades
GROUP BY bucket, token_mint, buy_dex, sell_dex;

CREATE MATERIALIZED VIEW opportunities_hourly
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 hour', time) AS bucket,
    token_mint,
    buy_dex,
    sell_dex,
    COUNT(*) AS opportunity_count,
    AVG(profit_percentage) AS avg_profit_percentage,
    MAX(profit_percentage) AS max_profit_percentage,
    AVG(estimated_profit_usd) AS avg_estimated_profit,
    SUM(estimated_profit_usd) AS total_estimated_profit,
    AVG(confidence_score) AS avg_confidence,
    AVG(risk_score) AS avg_risk
FROM arbitrage_opportunities
GROUP BY bucket, token_mint, buy_dex, sell_dex;

-- Set up refresh policies for continuous aggregates
SELECT add_continuous_aggregate_policy('price_feeds_1min',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');

SELECT add_continuous_aggregate_policy('trades_hourly',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

SELECT add_continuous_aggregate_policy('opportunities_hourly',
    start_offset => INTERVAL '1 day',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour');

-- Set up data retention policies
SELECT add_retention_policy('price_feeds', INTERVAL '30 days');
SELECT add_retention_policy('system_logs', INTERVAL '7 days');
SELECT add_retention_policy('performance_metrics', INTERVAL '90 days');

-- Insert initial configuration
INSERT INTO configuration (key, value, value_type, description) VALUES
('engine.version', '1.0.0', 'string', 'Engine version'),
('engine.max_concurrent_trades', '10', 'integer', 'Maximum concurrent trades'),
('engine.min_profit_threshold', '0.5', 'decimal', 'Minimum profit threshold percentage'),
('engine.max_slippage', '2.0', 'decimal', 'Maximum allowed slippage percentage'),
('risk.max_position_size', '1000', 'decimal', 'Maximum position size in USD'),
('risk.daily_loss_limit', '5000', 'decimal', 'Daily loss limit in USD'),
('monitoring.metrics_interval', '5', 'integer', 'Metrics collection interval in seconds'),
('dex.raydium.enabled', 'true', 'boolean', 'Enable Raydium DEX'),
('dex.orca.enabled', 'true', 'boolean', 'Enable Orca DEX'),
('dex.meteora.enabled', 'true', 'boolean', 'Enable Meteora DEX'),
('dex.jupiter.enabled', 'true', 'boolean', 'Enable Jupiter DEX');

-- Insert some popular Solana tokens
INSERT INTO tokens (mint_address, symbol, name, decimals, is_verified) VALUES
('So11111111111111111111111111111111111111112', 'SOL', 'Solana', 9, true),
('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 'USDC', 'USD Coin', 6, true),
('Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB', 'USDT', 'Tether USD', 6, true),
('mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So', 'mSOL', 'Marinade staked SOL', 9, true),
('7dHbWXmci3dT8UFYWYZweBLXgycu7Y3iL6trKn1Y7ARj', 'stSOL', 'Lido Staked SOL', 9, true),
('DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263', 'BONK', 'Bonk', 5, true),
('JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN', 'JUP', 'Jupiter', 6, true),
('WENWENvqqNya429ubCdR81ZmD69brwQaaBYY6p3LCpk', 'WEN', 'Wen', 5, true);

-- Create functions for common queries
CREATE OR REPLACE FUNCTION get_latest_price(token_mint_param VARCHAR(44), dex_param dex_type DEFAULT NULL)
RETURNS TABLE(price DECIMAL(20,10), time TIMESTAMPTZ) AS $$
BEGIN
    RETURN QUERY
    SELECT pf.price, pf.time
    FROM price_feeds pf
    WHERE pf.token_mint = token_mint_param
    AND (dex_param IS NULL OR pf.dex = dex_param)
    ORDER BY pf.time DESC
    LIMIT 1;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION get_arbitrage_stats(time_interval INTERVAL DEFAULT INTERVAL '24 hours')
RETURNS TABLE(
    total_opportunities BIGINT,
    total_trades BIGINT,
    success_rate DECIMAL(5,2),
    total_profit DECIMAL(20,6),
    avg_profit DECIMAL(20,6)
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        COUNT(DISTINCT ao.id) as total_opportunities,
        COUNT(DISTINCT t.id) as total_trades,
        ROUND(
            (COUNT(DISTINCT t.id) FILTER (WHERE t.status = 'completed')::DECIMAL / 
             NULLIF(COUNT(DISTINCT t.id), 0) * 100), 2
        ) as success_rate,
        COALESCE(SUM(t.actual_profit_usd), 0) as total_profit,
        COALESCE(AVG(t.actual_profit_usd), 0) as avg_profit
    FROM arbitrage_opportunities ao
    LEFT JOIN trades t ON ao.id = t.opportunity_id
    WHERE ao.time >= NOW() - time_interval;
END;
$$ LANGUAGE plpgsql;

-- Create triggers for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_tokens_updated_at BEFORE UPDATE ON tokens
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_configuration_updated_at BEFORE UPDATE ON configuration
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Grant permissions
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO arbitrage_user;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO arbitrage_user;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO arbitrage_user;

-- Create read-only user for monitoring
CREATE USER arbitrage_readonly WITH PASSWORD 'readonly_password';
GRANT CONNECT ON DATABASE arbitrage_db TO arbitrage_readonly;
GRANT USAGE ON SCHEMA public TO arbitrage_readonly;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO arbitrage_readonly;
GRANT SELECT ON ALL SEQUENCES IN SCHEMA public TO arbitrage_readonly;

COMMIT;