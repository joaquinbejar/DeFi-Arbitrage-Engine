# DeFi Arbitrage Engine

[![Rust](https://img.shields.io/badge/rust-1.89+-orange.svg)](https://www.rust-lang.org)
[![Solana](https://img.shields.io/badge/solana-1.18+-purple.svg)](https://solana.com)
[![Anchor](https://img.shields.io/badge/anchor-0.29+-blue.svg)](https://anchor-lang.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

A high-performance, production-ready arbitrage engine for Solana DeFi protocols. This engine automatically detects and executes profitable arbitrage opportunities across multiple DEXs including Raydium, Orca, Meteora, and Jupiter.

## 🚀 Features

### Core Engine
- **Real-time Opportunity Detection**: Advanced algorithms to identify arbitrage opportunities across multiple DEXs
- **Atomic Execution**: Flash loan-powered atomic transactions ensuring risk-free arbitrage
- **MEV Protection**: Built-in sandwich attack detection and prevention
- **Multi-DEX Support**: Integrated with Raydium, Orca, Meteora, and Jupiter
- **Risk Management**: Comprehensive position sizing and loss prevention systems
- **High Performance**: Sub-100ms execution latency with optimized Rust implementation

### Smart Contracts
- **Flash Arbitrage Program**: Atomic execution with flash loans
- **Cross-DEX Router**: Multi-hop optimization across different DEXs
- **MEV Protection**: Sandwich attack prevention and transaction ordering

### Infrastructure
- **Real-time Data Streaming**: Geyser gRPC integration for blockchain data
- **WebSocket API**: Real-time updates for connected clients
- **REST API**: Comprehensive endpoints for monitoring and control
- **TimescaleDB**: Time-series database for storing price data, trades, and analytics
- **Redis Caching**: Ultra-fast data access and session management
- **Prometheus Metrics**: Comprehensive monitoring and alerting
- **Grafana Dashboards**: Beautiful visualization and analytics

## 📋 Prerequisites

- **Rust**: 1.89.0 or later
- **Node.js**: 20.0.0 or later
- **Solana CLI**: 1.18.8 or later
- **Anchor CLI**: 0.29.0 or later
- **Docker & Docker Compose**: Latest version
- **TimescaleDB**: 15+ (or use Docker)
- **Redis**: 7+ (or use Docker)

## 🛠️ Quick Start

### Automated Setup

Run the automated setup script to install all dependencies and configure the development environment:

```bash
# Clone the repository
git clone https://github.com/joaquinbejar/DeFi-Arbitrage-Engine.git
cd defi-arbitrage-engine

# Run the setup script
./scripts/setup-dev.sh
```

### Manual Setup

1. **Install System Dependencies**

   ```bash
   # macOS
   brew install postgresql redis docker
   
   # Ubuntu/Debian
   sudo apt-get install postgresql-client redis-tools docker.io docker-compose
   ```

2. **Install Rust and Cargo Tools**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   cargo install sqlx-cli --features postgres
   cargo install cargo-watch cargo-audit
   ```

3. **Install Solana CLI**

   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/v1.18.8/install)"
   ```

4. **Install Anchor CLI**

   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install 0.29.0
   avm use 0.29.0
   ```

5. **Configure Environment**

   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

6. **Start Infrastructure Services**

   ```bash
   docker-compose up -d timescaledb redis prometheus grafana
   ```

7. **Run Database Migrations**

   ```bash
   PGPASSWORD=arbitrage_pass psql -h localhost -p 5432 -U arbitrage_user -d arbitrage_db -f migrations/001_init_database.sql
   ```

8. **Build and Run**

   ```bash
   # Build the project
   cargo build --release
   
   # Build Anchor programs
   anchor build
   
   # Run the engine
   cargo run --release
   ```

## 🏗️ Architecture

### System Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Geyser gRPC   │    │   Solana RPC    │    │   DEX APIs      │
│   (Real-time)   │    │   (State)       │    │   (Prices)      │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────┴─────────────┐
                    │    Arbitrage Engine       │
                    │                           │
                    │  ┌─────────────────────┐  │
                    │  │ Opportunity Detector│  │
                    │  └─────────────────────┘  │
                    │  ┌─────────────────────┐  │
                    │  │ Execution Engine    │  │
                    │  └─────────────────────┘  │
                    │  ┌─────────────────────┐  │
                    │  │ Risk Manager        │  │
                    │  └─────────────────────┘  │
                    └─────────────┬─────────────┘
                                  │
        ┌─────────────────────────┼─────────────────────────┐
        │                         │                         │
┌───────▼───────┐    ┌───────────▼────────────┐    ┌───────▼───────┐
│  TimescaleDB  │    │      Redis Cache       │    │   Monitoring  │
│  (Analytics)  │    │    (Fast Access)       │    │ (Prometheus)  │
└───────────────┘    └────────────────────────┘    └───────────────┘
```

### Core Components

1. **Opportunity Detection System** (`engine/src/opportunities/`)
   - Real-time price monitoring across multiple DEXs
   - Advanced arbitrage opportunity identification
   - Profit calculation and feasibility analysis

2. **Execution Engine** (`engine/src/execution/`)
   - Atomic transaction bundling
   - Flash loan integration
   - MEV protection and front-running prevention

3. **DEX Integration Layer** (`engine/src/dex/`)
   - Unified interface for multiple DEXs
   - Real-time price feeds and liquidity monitoring
   - Transaction routing and optimization

4. **Risk Management** (`engine/src/risk/`)
   - Position sizing algorithms
   - Loss prevention mechanisms
   - Market volatility analysis

5. **Smart Contracts** (`programs/`)
   - Flash arbitrage program
   - Cross-DEX router
   - MEV protection mechanisms

## 📊 Monitoring & Analytics

### Grafana Dashboards

Access the monitoring dashboard at `http://localhost:3000` (admin/admin):

- **Arbitrage Overview**: Real-time opportunities and execution metrics
- **Performance Metrics**: Latency, throughput, and success rates
- **Risk Analytics**: Position sizes, drawdowns, and risk exposure
- **System Health**: Infrastructure monitoring and alerts

### Prometheus Metrics

Key metrics available at `http://localhost:9090`:

- `arbitrage_opportunities_detected_total`: Total opportunities found
- `arbitrage_trades_executed_total`: Total trades executed
- `arbitrage_profit_usd`: Total profit in USD
- `arbitrage_execution_latency_seconds`: Execution latency distribution
- `arbitrage_success_rate`: Trade success rate percentage

## 🔧 Configuration

### Environment Variables

Key configuration options in `.env`:

```bash
# Database
DATABASE_URL=timescaledb://arbitrage_user:arbitrage_pass@localhost:5432/arbitrage_db
REDIS_URL=redis://localhost:6379

# Solana
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_WS_URL=wss://api.mainnet-beta.solana.com
SOLANA_PRIVATE_KEY_PATH=/path/to/keypair.json

# Trading
MAX_SLIPPAGE=0.01
MIN_PROFIT_THRESHOLD=0.001
MAX_POSITION_SIZE=1000

# Risk Management
MAX_DRAWDOWN=0.05
POSITION_SIZE_MULTIPLIER=0.1
STOP_LOSS_THRESHOLD=0.02
```

### Trading Configuration

Edit `config.toml` for detailed trading parameters:

```toml
[trading]
max_slippage = 0.01
min_profit_threshold = 0.001
max_position_size = 1000.0
gas_price_multiplier = 1.2

[risk]
max_drawdown = 0.05
position_size_multiplier = 0.1
stop_loss_threshold = 0.02
max_concurrent_trades = 5
```

## 🚀 Deployment

### Docker Deployment

```bash
# Build the Docker image
docker build -t defi-arbitrage-engine .

# Run with Docker Compose
docker-compose up -d
```

### Production Deployment

1. **Configure Production Environment**
   ```bash
   cp .env.production .env
   # Update with production values
   ```

2. **Build Release Binary**
   ```bash
   cargo build --release
   ```

3. **Deploy Smart Contracts**
   ```bash
   anchor deploy --provider.cluster mainnet
   ```

4. **Start Services**
   ```bash
   ./target/release/arbitrage-engine
   ```

## 🧪 Testing

### Unit Tests

```bash
# Run all tests
cargo test

# Run specific module tests
cargo test opportunities
cargo test execution
cargo test risk
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration

# Test with real DEX data (requires API keys)
cargo test --test integration --features live-testing
```

### Smart Contract Tests

```bash
# Test Anchor programs
anchor test

# Test specific program
anchor test --skip-local-validator programs/flash-arbitrage
```

## 📈 Performance

### Benchmarks

- **Opportunity Detection**: < 10ms average latency
- **Trade Execution**: < 100ms end-to-end
- **Throughput**: 1000+ opportunities/second analysis
- **Memory Usage**: < 512MB under normal load
- **CPU Usage**: < 50% on 4-core system

### Optimization Tips

1. **Database Tuning**
   - Configure TimescaleDB chunk intervals
   - Optimize indexes for query patterns
   - Use connection pooling

2. **Redis Configuration**
   - Enable persistence for critical data
   - Configure memory limits
   - Use Redis Cluster for scaling

3. **Network Optimization**
   - Use dedicated RPC endpoints
   - Configure connection pooling
   - Implement request batching

## 🔒 Security

### Best Practices

1. **Private Key Management**
   - Use hardware wallets for production
   - Implement key rotation policies
   - Never commit keys to version control

2. **Network Security**
   - Use VPN for production deployments
   - Implement rate limiting
   - Monitor for suspicious activity

3. **Smart Contract Security**
   - Regular security audits
   - Implement circuit breakers
   - Use multi-signature wallets

### Risk Mitigation

- **Position Limits**: Automatic position sizing based on account balance
- **Stop Losses**: Automatic trade termination on adverse price movements
- **Circuit Breakers**: System shutdown on excessive losses
- **MEV Protection**: Built-in sandwich attack prevention

## 🛠️ Development

### Project Structure

```
defi-arbitrage-engine/
├── engine/                 # Main Rust engine
│   ├── src/
│   │   ├── opportunities/  # Opportunity detection
│   │   ├── execution/      # Trade execution
│   │   ├── dex/           # DEX integrations
│   │   ├── risk/          # Risk management
│   │   └── api/           # REST/WebSocket APIs
│   └── Cargo.toml
├── programs/              # Anchor smart contracts
│   ├── flash-arbitrage/   # Flash loan arbitrage
│   ├── cross-dex-router/  # Multi-DEX routing
│   └── mev-protection/    # MEV protection
├── migrations/            # Database migrations
├── monitoring/            # Prometheus/Grafana config
├── scripts/              # Utility scripts
├── docker-compose.yml    # Development environment
└── Dockerfile           # Production container
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Run the test suite
6. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run Clippy for linting (`cargo clippy`)
- Maintain test coverage above 80%
- Document public APIs

## 📚 API Documentation

### REST API Endpoints

- `GET /api/v1/opportunities` - List current opportunities
- `GET /api/v1/trades` - Trade history
- `GET /api/v1/metrics` - System metrics
- `POST /api/v1/trades` - Execute manual trade
- `GET /api/v1/health` - Health check

### WebSocket API

- `/ws/opportunities` - Real-time opportunity updates
- `/ws/trades` - Real-time trade execution updates
- `/ws/metrics` - Real-time system metrics

## 🐛 Troubleshooting

### Common Issues

1. **Database Connection Errors**
   ```bash
   # Check if TimescaleDB is running
   docker-compose ps timescaledb
   
   # Check logs
   docker-compose logs timescaledb
   ```

2. **Solana RPC Issues**
   ```bash
   # Test RPC connection
   solana cluster-version
   
   # Check balance
   solana balance
   ```

3. **Build Errors**
   ```bash
   # Clean and rebuild
   cargo clean
   cargo build
   
   # Update dependencies
   cargo update
   ```

### Logging

Adjust log levels in `.env`:

```bash
# Debug level logging
RUST_LOG=debug

# Module-specific logging
RUST_LOG=arbitrage_engine=debug,sqlx=warn
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Support

- **Documentation**: [Wiki](https://github.com/joaquinbejar/DeFi-Arbitrage-Engine/wiki)
- **Issues**: [GitHub Issues](https://github.com/joaquinbejar/DeFi-Arbitrage-Engine/issues)
- **Discussions**: [GitHub Discussions](https://github.com/joaquinbejar/DeFi-Arbitrage-Engine/discussions)
- **Discord**: [Community Server](https://discord.gg/your-server)

### **Contact Information**
- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: [@joaquin_bejar](https://t.me/joaquin_bejar)
- **GitHub**: <https://github.com/joaquinbejar>


## ⚠️ Disclaimer

This software is provided for educational and research purposes only. Trading cryptocurrencies involves substantial risk of loss and is not suitable for all investors. The authors and contributors are not responsible for any financial losses incurred through the use of this software.

Always:
- Test thoroughly on devnet before mainnet deployment
- Start with small position sizes
- Monitor your trades closely
- Understand the risks involved
- Comply with local regulations

---

**Built with ❤️ for the Solana DeFi ecosystem**