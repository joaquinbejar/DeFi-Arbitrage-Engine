# Solana DeFi Arbitrage Engine

A high-performance, real-time arbitrage engine for Solana DeFi protocols. This engine detects and executes arbitrage opportunities across multiple DEXs including Raydium, Orca, Meteora, and Jupiter.

## Features

### Core Functionality
- **Real-time Market Data**: Streams live price data via Geyser gRPC
- **Multi-DEX Support**: Integrates with Raydium, Orca, Meteora, and Jupiter
- **Opportunity Detection**: Advanced algorithms for triangular and cross-DEX arbitrage
- **Atomic Execution**: Flash loan integration for capital-efficient trades
- **Risk Management**: Comprehensive position sizing and loss prevention
- **MEV Protection**: Jito integration for sandwich attack prevention

### Infrastructure
- **WebSocket API**: Real-time data streaming to clients
- **REST API**: Management and monitoring endpoints
- **TimescaleDB**: Time-series database optimized for financial data
- **Redis Cache**: High-performance caching layer
- **Prometheus Metrics**: Comprehensive monitoring and alerting
- **Distributed Architecture**: Scalable and fault-tolerant design

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Geyser gRPC   │    │   DEX APIs      │    │   Price Feeds   │
│   (Real-time)   │    │   (Quotes)      │    │   (Backup)      │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────▼─────────────┐
                    │    Arbitrage Engine      │
                    │  ┌─────────────────────┐ │
                    │  │ Opportunity Detector│ │
                    │  └─────────────────────┘ │
                    │  ┌─────────────────────┐ │
                    │  │  Execution Engine   │ │
                    │  └─────────────────────┘ │
                    │  ┌─────────────────────┐ │
                    │  │   Risk Manager      │ │
                    │  └─────────────────────┘ │
                    └─────────────┬─────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
┌───────▼───────┐    ┌───────────▼────────────┐    ┌───────▼───────┐
│  TimescaleDB  │    │      Redis Cache       │    │   WebSocket   │
│  (Analytics)  │    │    (Performance)       │    │   (Real-time) │
└───────────────┘    └────────────────────────┘    └───────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+ with TimescaleDB extension
- Redis 6+
- Solana CLI tools
- Access to Geyser gRPC endpoint

### Installation

1. **Clone the repository**:
   ```bash
   git clone https://github.com/your-org/defi-arbitrage-engine.git
   cd defi-arbitrage-engine/engine
   ```

2. **Install dependencies**:
   ```bash
   cargo build --release
   ```

3. **Set up the database**:
   ```bash
   # Create TimescaleDB database
   createdb arbitrage_db
   
   # Install TimescaleDB extension
   psql arbitrage_db -c "CREATE EXTENSION IF NOT EXISTS timescaledb;"
   ```

4. **Configure the engine**:
   ```bash
   cp config.example.toml config.toml
   # Edit config.toml with your settings
   ```

5. **Run the engine**:
   ```bash
   cargo run --release
   ```

### Configuration

The engine uses a TOML configuration file. Key sections include:

- **Database**: PostgreSQL/TimescaleDB connection settings
- **Redis**: Cache configuration and connection pooling
- **Solana**: RPC endpoints and Geyser gRPC settings
- **DEX**: Individual DEX configurations and API endpoints
- **Trading**: Position sizing, slippage, and execution parameters
- **Risk**: Loss limits, circuit breakers, and exposure controls
- **API**: REST and WebSocket server settings
- **Metrics**: Prometheus monitoring configuration

## Usage

### Basic Example

```rust
use arbitrage_engine::{
    ArbitrageEngine, Config, Database, RedisCache, MetricsCollector
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Arc::new(Config::load("config.toml").await?);
    
    // Initialize components
    let database = Arc::new(Database::new(&config).await?);
    let redis = Arc::new(RedisCache::new(&config).await?);
    let metrics = Arc::new(MetricsCollector::new());
    
    // Create and start the arbitrage engine
    let engine = ArbitrageEngine::new(config, database, redis, metrics).await?;
    engine.start().await?;
    
    // Keep running until shutdown signal
    tokio::signal::ctrl_c().await?;
    engine.stop().await?;
    
    Ok(())
}
```

### API Endpoints

#### REST API (Port 8080)

- `GET /health` - Health check
- `GET /opportunities` - Current arbitrage opportunities
- `GET /opportunities/history` - Historical opportunities
- `POST /trading/execute` - Execute specific opportunity
- `GET /trading/positions` - Current positions
- `GET /market/prices` - Current token prices
- `GET /portfolio/balance` - Wallet balances
- `GET /metrics` - Performance metrics
- `POST /admin/stop` - Emergency stop
- `POST /admin/resume` - Resume operations

#### WebSocket API (Port 8081)

```javascript
const ws = new WebSocket('ws://localhost:8081');

// Subscribe to opportunities
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'opportunities'
}));

// Subscribe to trades
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'trades'
}));

// Subscribe to market data
ws.send(JSON.stringify({
  type: 'subscribe',
  channel: 'market_data',
  tokens: ['SOL', 'USDC', 'RAY']
}));
```

## Development

### Project Structure

```
engine/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Configuration management
│   ├── types.rs             # Core data structures
│   ├── error.rs             # Error handling
│   ├── database.rs          # TimescaleDB integration
│   ├── redis.rs             # Redis cache layer
│   ├── metrics.rs           # Prometheus metrics
│   ├── websocket.rs         # WebSocket server
│   ├── api.rs               # REST API server
│   ├── arbitrage_engine.rs  # Main engine orchestrator
│   ├── opportunity_detector.rs # Opportunity detection
│   ├── execution_engine.rs  # Trade execution
│   ├── risk_manager.rs      # Risk management
│   ├── dex_integration.rs   # DEX protocol integrations
│   └── geyser_client.rs     # Geyser gRPC client
├── proto/                   # Protocol Buffer definitions
├── migrations/              # Database migrations
├── config.example.toml      # Example configuration
├── Cargo.toml              # Rust dependencies
└── build.rs                # Build script for gRPC
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint code
cargo clippy
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test opportunity_detector

# Run integration tests
cargo test --test integration

# Run with output
cargo test -- --nocapture
```

## Monitoring

### Metrics

The engine exposes Prometheus metrics on `/metrics` endpoint:

- **System Metrics**: CPU, memory, disk usage
- **Trading Metrics**: Opportunities detected, trades executed, PnL
- **DEX Metrics**: API latency, success rates, pool liquidity
- **Database Metrics**: Query performance, connection pool status
- **Cache Metrics**: Hit rates, memory usage, evictions
- **WebSocket Metrics**: Active connections, message rates
- **Risk Metrics**: Position sizes, exposure levels, circuit breaker status

### Logging

Structured JSON logging with configurable levels:

```bash
# Set log level
export RUST_LOG=arbitrage_engine=debug

# Log to file
export RUST_LOG=arbitrage_engine=info
cargo run 2>&1 | tee arbitrage.log
```

### Health Checks

```bash
# Check engine health
curl http://localhost:8080/health

# Check component status
curl http://localhost:8080/health/detailed
```

## Performance Tuning

### Configuration Optimization

```toml
[performance]
max_threads = 16              # Match CPU cores
worker_threads = 8             # Async task threads
blocking_threads = 4           # Blocking I/O threads

[database]
max_connections = 50           # Scale with load
min_connections = 10

[redis]
max_connections = 20           # Scale with throughput
default_ttl = 60              # Optimize cache TTL

[trading]
max_concurrent_trades = 10     # Balance speed vs risk
```

### System Optimization

```bash
# Increase file descriptor limits
ulimit -n 65536

# Optimize TCP settings
echo 'net.core.rmem_max = 134217728' >> /etc/sysctl.conf
echo 'net.core.wmem_max = 134217728' >> /etc/sysctl.conf

# Use performance CPU governor
echo performance > /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

## Security

### Wallet Security

- Store private keys in secure hardware wallets when possible
- Use environment variables for sensitive configuration
- Implement proper key rotation procedures
- Monitor wallet balances and transactions

### Network Security

- Use TLS for all external communications
- Implement rate limiting and DDoS protection
- Whitelist trusted IP addresses
- Regular security audits and updates

### Operational Security

- Run with minimal privileges
- Use containerization for isolation
- Implement proper logging and monitoring
- Regular backups of configuration and data

## Troubleshooting

### Common Issues

1. **Connection Timeouts**:
   ```bash
   # Check network connectivity
   curl -I https://api.mainnet-beta.solana.com
   
   # Verify Geyser endpoint
   grpcurl -plaintext localhost:10000 list
   ```

2. **Database Issues**:
   ```bash
   # Check PostgreSQL status
   pg_isready -h localhost -p 5432
   
   # Verify TimescaleDB extension
   psql -d arbitrage_db -c "SELECT * FROM pg_extension WHERE extname='timescaledb';"
   ```

3. **Redis Issues**:
   ```bash
   # Check Redis connectivity
   redis-cli ping
   
   # Monitor Redis performance
   redis-cli --latency-history
   ```

4. **Performance Issues**:
   ```bash
   # Monitor system resources
   htop
   
   # Check network latency
   ping api.mainnet-beta.solana.com
   
   # Profile the application
   cargo flamegraph --bin arbitrage-engine
   ```

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Enable development mode
echo 'debug_mode = true' >> config.toml

# Dry run mode (no actual trades)
echo 'dry_run = true' >> config.toml
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass
6. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Use meaningful variable and function names
- Add comprehensive documentation
- Include unit tests for new features
- Follow error handling best practices

## License

MIT License - see LICENSE file for details.

## Support

For support and questions:

- GitHub Issues: [Report bugs and feature requests](https://github.com/your-org/defi-arbitrage-engine/issues)
- Documentation: [Full documentation](https://docs.your-org.com/arbitrage-engine)
- Discord: [Community chat](https://discord.gg/your-server)

## Disclaimer

This software is for educational and research purposes. Trading cryptocurrencies involves substantial risk of loss. Use at your own risk and ensure compliance with applicable regulations.