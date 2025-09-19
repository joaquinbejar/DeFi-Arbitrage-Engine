-- TimescaleDB Migration for DeFi Arbitrage Engine
-- This schema is optimized for time-series data and high-frequency trading operations

-- Enable TimescaleDB extension
CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Custom types for better data modeling
CREATE TYPE dex_type AS ENUM ('uniswap_v2', 'uniswap_v3', 'sushiswap', 'pancakeswap', 'raydium', 'orca', 'serum');
CREATE TYPE opportunity_status AS ENUM ('detected', 'executing', 'completed', 'failed', 'expired');
CREATE TYPE transaction_status AS ENUM ('pending', 'confirmed', 'failed', 'reverted');
CREATE TYPE arbitrage_type AS ENUM ('triangular', 'cross_dex', 'flash_loan');

-- Users table for authentication and tracking
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    wallet_address VARCHAR(44) UNIQUE NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    is_active BOOLEAN DEFAULT true,
    settings JSONB DEFAULT '{}'
);

-- Tokens table with comprehensive token information
CREATE TABLE tokens (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    symbol VARCHAR(20) NOT NULL,
    name VARCHAR(100) NOT NULL,
    mint_address VARCHAR(44) UNIQUE NOT NULL,
    decimals INTEGER NOT NULL,
    chain_id INTEGER NOT NULL,
    coingecko_id VARCHAR(100),
    logo_uri TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    is_active BOOLEAN DEFAULT true
);

-- DEX pools with time-series capabilities
CREATE TABLE dex_pools (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    dex_type dex_type NOT NULL,
    pool_address VARCHAR(44) UNIQUE NOT NULL,
    token_a_id UUID REFERENCES tokens(id),
    token_b_id UUID REFERENCES tokens(id),
    fee_tier INTEGER,
    liquidity NUMERIC(36, 18),
    sqrt_price_x96 NUMERIC(78, 0),
    tick INTEGER,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW(),
    is_active BOOLEAN DEFAULT true
);

-- Price data as hypertable for time-series optimization
CREATE TABLE price_data (
    time TIMESTAMPTZ NOT NULL,
    token_id UUID NOT NULL REFERENCES tokens(id),
    pool_id UUID NOT NULL REFERENCES dex_pools(id),
    price NUMERIC(36, 18) NOT NULL,
    volume_24h NUMERIC(36, 18),
    liquidity NUMERIC(36, 18),
    market_cap NUMERIC(36, 18),
    price_change_24h NUMERIC(10, 4)
);

-- Convert to hypertable
SELECT create_hypertable('price_data', 'time');

-- Arbitrage opportunities with time-series tracking
CREATE TABLE arbitrage_opportunities (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    arbitrage_type arbitrage_type NOT NULL,
    token_path UUID[] NOT NULL,
    dex_path dex_type[] NOT NULL,
    pool_path UUID[] NOT NULL,
    profit_estimate NUMERIC(36, 18) NOT NULL,
    profit_percentage NUMERIC(10, 4) NOT NULL,
    gas_cost NUMERIC(36, 18),
    net_profit NUMERIC(36, 18),
    status opportunity_status DEFAULT 'detected',
    execution_time TIMESTAMPTZ,
    block_number BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    expires_at TIMESTAMPTZ
);

-- Convert to hypertable
SELECT create_hypertable('arbitrage_opportunities', 'time');

-- Transaction tracking
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    opportunity_id UUID REFERENCES arbitrage_opportunities(id),
    user_id UUID REFERENCES users(id),
    transaction_hash VARCHAR(128) UNIQUE,
    status transaction_status DEFAULT 'pending',
    gas_used BIGINT,
    gas_price NUMERIC(36, 18),
    actual_profit NUMERIC(36, 18),
    slippage NUMERIC(10, 4),
    execution_time_ms INTEGER,
    block_number BIGINT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('transactions', 'time');

-- Performance metrics tracking
CREATE TABLE performance_metrics (
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    metric_name VARCHAR(100) NOT NULL,
    metric_value NUMERIC(36, 18) NOT NULL,
    tags JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Convert to hypertable
SELECT create_hypertable('performance_metrics', 'time');

-- System configuration
CREATE TABLE system_config (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    key VARCHAR(100) UNIQUE NOT NULL,
    value JSONB NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Indexes for optimal query performance
CREATE INDEX idx_tokens_symbol ON tokens(symbol);
CREATE INDEX idx_tokens_mint_address ON tokens(mint_address);
CREATE INDEX idx_dex_pools_tokens ON dex_pools(token_a_id, token_b_id);
CREATE INDEX idx_dex_pools_dex_type ON dex_pools(dex_type);
CREATE INDEX idx_price_data_token_time ON price_data(token_id, time DESC);
CREATE INDEX idx_arbitrage_opportunities_status ON arbitrage_opportunities(status);
CREATE INDEX idx_arbitrage_opportunities_profit ON arbitrage_opportunities(profit_percentage DESC);
CREATE INDEX idx_transactions_user ON transactions(user_id);
CREATE INDEX idx_transactions_opportunity ON transactions(opportunity_id);
CREATE INDEX idx_performance_metrics_name_time ON performance_metrics(metric_name, time DESC);

-- Materialized views for analytics
CREATE MATERIALIZED VIEW hourly_price_summary AS
SELECT 
    time_bucket('1 hour', time) AS hour,
    token_id,
    FIRST(price, time) AS open_price,
    MAX(price) AS high_price,
    MIN(price) AS low_price,
    LAST(price, time) AS close_price,
    AVG(price) AS avg_price,
    SUM(volume_24h) AS total_volume
FROM price_data
GROUP BY hour, token_id;

CREATE MATERIALIZED VIEW daily_arbitrage_summary AS
SELECT 
    time_bucket('1 day', time) AS day,
    arbitrage_type,
    COUNT(*) AS total_opportunities,
    COUNT(*) FILTER (WHERE status = 'completed') AS successful_opportunities,
    AVG(profit_percentage) AS avg_profit_percentage,
    SUM(net_profit) FILTER (WHERE status = 'completed') AS total_profit
FROM arbitrage_opportunities
GROUP BY day, arbitrage_type;

-- Functions for data management
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for automatic timestamp updates
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_tokens_updated_at BEFORE UPDATE ON tokens
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_dex_pools_updated_at BEFORE UPDATE ON dex_pools
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_system_config_updated_at BEFORE UPDATE ON system_config
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Function to calculate arbitrage profit
CREATE OR REPLACE FUNCTION calculate_arbitrage_profit(
    input_amount NUMERIC,
    token_path UUID[],
    pool_path UUID[]
) RETURNS NUMERIC AS $$
DECLARE
    current_amount NUMERIC := input_amount;
    i INTEGER;
    pool_liquidity NUMERIC;
BEGIN
    FOR i IN 1..array_length(pool_path, 1) LOOP
        SELECT liquidity INTO pool_liquidity
        FROM dex_pools
        WHERE id = pool_path[i];
        
        -- Simplified AMM calculation (would need more complex logic for real implementation)
        current_amount := current_amount * 0.997; -- 0.3% fee
    END LOOP;
    
    RETURN current_amount - input_amount;
END;
$$ LANGUAGE plpgsql;

-- Data retention policies
SELECT add_retention_policy('price_data', INTERVAL '30 days');
SELECT add_retention_policy('performance_metrics', INTERVAL '90 days');

-- Continuous aggregates for real-time analytics
CREATE MATERIALIZED VIEW price_data_1min
WITH (timescaledb.continuous) AS
SELECT 
    time_bucket('1 minute', time) AS bucket,
    token_id,
    pool_id,
    FIRST(price, time) AS open,
    MAX(price) AS high,
    MIN(price) AS low,
    LAST(price, time) AS close,
    AVG(price) AS avg_price
FROM price_data
GROUP BY bucket, token_id, pool_id;

-- Refresh policies for materialized views
SELECT add_continuous_aggregate_policy('price_data_1min',
    start_offset => INTERVAL '1 hour',
    end_offset => INTERVAL '1 minute',
    schedule_interval => INTERVAL '1 minute');

-- Insert default tokens
INSERT INTO tokens (symbol, name, mint_address, decimals, chain_id) VALUES
('SOL', 'Solana', 'So11111111111111111111111111111111111111112', 9, 101),
('USDC', 'USD Coin', 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 6, 101),
('USDT', 'Tether USD', 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB', 6, 101),
('RAY', 'Raydium', '4k3Dyjzvzp8eMZWUXbBCjEvwSkkk59S5iCNLY3QrkX6R', 6, 101),
('SRM', 'Serum', 'SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt', 6, 101);

-- Insert default system configuration
INSERT INTO system_config (key, value, description) VALUES
('max_slippage', '0.01', 'Maximum allowed slippage percentage'),
('min_profit_threshold', '0.005', 'Minimum profit threshold percentage'),
('gas_price_multiplier', '1.2', 'Gas price multiplier for faster execution'),
('max_position_size', '1000', 'Maximum position size in USD'),
('execution_timeout', '30', 'Execution timeout in seconds');

-- Grant permissions
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO anon;
GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO authenticated;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO anon;
GRANT ALL PRIVILEGES ON ALL SEQUENCES IN SCHEMA public TO authenticated;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO anon;
GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO authenticated;

-- Enable row level security
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE arbitrage_opportunities ENABLE ROW LEVEL SECURITY;
ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;

-- RLS policies
CREATE POLICY "Users can view own data" ON users
    FOR ALL USING (auth.uid()::text = id::text);

CREATE POLICY "Users can view own opportunities" ON arbitrage_opportunities
    FOR ALL USING (true); -- Allow all for now, can be restricted later

CREATE POLICY "Users can view own transactions" ON transactions
    FOR ALL USING (auth.uid()::text = user_id::text);

-- Comments for documentation
COMMENT ON TABLE price_data IS 'Time-series price data optimized with TimescaleDB hypertables';
COMMENT ON TABLE arbitrage_opportunities IS 'Detected arbitrage opportunities with profit calculations';
COMMENT ON TABLE transactions IS 'Transaction execution tracking and results';
COMMENT ON FUNCTION calculate_arbitrage_profit IS 'Calculates potential arbitrage profit for given token and pool path';