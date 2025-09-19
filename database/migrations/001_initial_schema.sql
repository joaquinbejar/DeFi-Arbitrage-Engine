-- Initial schema for Solana DeFi Arbitrage Engine
-- Based on technical architecture document

-- Create users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    public_key VARCHAR(44) UNIQUE NOT NULL,
    role VARCHAR(20) DEFAULT 'observer' CHECK (role IN ('operator', 'analyst', 'observer')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_login TIMESTAMP WITH TIME ZONE,
    is_active BOOLEAN DEFAULT true
);

-- Create opportunities table
CREATE TABLE opportunities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_dex VARCHAR(20) NOT NULL,
    target_dex VARCHAR(20) NOT NULL,
    token_pair_input VARCHAR(44) NOT NULL,
    token_pair_output VARCHAR(44) NOT NULL,
    price_discrepancy DECIMAL(10,6) NOT NULL,
    expected_profit DECIMAL(18,9) NOT NULL,
    required_capital DECIMAL(18,9) NOT NULL,
    confidence INTEGER CHECK (confidence >= 0 AND confidence <= 100),
    deadline TIMESTAMP WITH TIME ZONE NOT NULL,
    route_data JSONB NOT NULL,
    detected_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'executed', 'expired', 'cancelled'))
);

-- Create executions table
CREATE TABLE executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    opportunity_id UUID REFERENCES opportunities(id),
    user_id UUID REFERENCES users(id),
    status VARCHAR(20) DEFAULT 'pending' CHECK (status IN ('pending', 'success', 'failed', 'cancelled')),
    actual_profit DECIMAL(18,9),
    gas_used INTEGER,
    execution_time_ms INTEGER,
    tx_signatures JSONB,
    error_message TEXT,
    started_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    completed_at TIMESTAMP WITH TIME ZONE
);

-- Create system_metrics table
CREATE TABLE system_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    active_opportunities INTEGER NOT NULL,
    success_rate DECIMAL(5,4) NOT NULL,
    total_profit DECIMAL(18,9) NOT NULL,
    avg_execution_time DECIMAL(8,2) NOT NULL,
    system_uptime DECIMAL(5,4) NOT NULL,
    data_latency DECIMAL(8,2) NOT NULL,
    recorded_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create dex_pools table
CREATE TABLE dex_pools (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dex_name VARCHAR(20) NOT NULL,
    pool_address VARCHAR(44) NOT NULL,
    token_a_mint VARCHAR(44) NOT NULL,
    token_b_mint VARCHAR(44) NOT NULL,
    liquidity DECIMAL(18,9),
    fee_rate DECIMAL(6,6) NOT NULL,
    last_updated TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_users_public_key ON users(public_key);
CREATE INDEX idx_users_role ON users(role);
CREATE INDEX idx_opportunities_detected_at ON opportunities(detected_at DESC);
CREATE INDEX idx_opportunities_status ON opportunities(status);
CREATE INDEX idx_opportunities_profit ON opportunities(expected_profit DESC);
CREATE INDEX idx_opportunities_deadline ON opportunities(deadline);
CREATE INDEX idx_executions_opportunity_id ON executions(opportunity_id);
CREATE INDEX idx_executions_user_id ON executions(user_id);
CREATE INDEX idx_executions_started_at ON executions(started_at DESC);
CREATE INDEX idx_executions_status ON executions(status);
CREATE INDEX idx_system_metrics_recorded_at ON system_metrics(recorded_at DESC);

-- Insert default operator user
INSERT INTO users (public_key, role) VALUES 
('9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM', 'operator');

-- Insert sample DEX pools
INSERT INTO dex_pools (dex_name, pool_address, token_a_mint, token_b_mint, fee_rate) VALUES 
('raydium', '58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2', 'So11111111111111111111111111111111111111112', 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 0.0025),
('orca', '2p7nYbtPBgtmY69NsE8DAW6szpRJn7tQvDnqvoEWQvjY', 'So11111111111111111111111111111111111111112', 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v', 0.003);