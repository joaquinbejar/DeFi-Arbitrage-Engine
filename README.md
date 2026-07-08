# DeFi Arbitrage Engine

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=for-the-badge&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)
[![Solana](https://img.shields.io/badge/Solana-9945FF?style=for-the-badge&logo=solana&logoColor=white)](https://solana.com/)
[![React](https://img.shields.io/badge/react-%2320232a.svg?style=for-the-badge&logo=react&logoColor=%2361DAFB)](https://reactjs.org/)

A high-performance, real-time DeFi arbitrage engine built on Solana blockchain, designed to identify and execute profitable arbitrage opportunities across multiple decentralized exchanges (DEXs).

## 🚀 Overview

The DeFi Arbitrage Engine is a comprehensive solution that combines real-time market monitoring, advanced arbitrage strategies, and automated execution capabilities. It leverages Solana's high throughput and low latency to capture arbitrage opportunities with minimal slippage and maximum profitability.

### Key Features

- **Real-time Market Monitoring**: Continuous monitoring of multiple DEXs for price discrepancies
- **Advanced Arbitrage Strategies**: Multiple strategies including cross-DEX arbitrage and flash loans
- **MEV Protection**: Built-in protection against Maximum Extractable Value attacks
- **High-Performance Engine**: Rust-based engine optimized for speed and efficiency
- **Interactive Dashboard**: React-based web interface for monitoring and management
- **TimescaleDB Integration**: Time-series database for historical data and analytics
- **Comprehensive Monitoring**: Grafana dashboards and Prometheus metrics

## 🏗️ Architecture

The project consists of several interconnected components:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Dashboard     │    │     Engine      │    │    Programs     │
│   (React/TS)    │◄──►│    (Rust)       │◄──►│   (Anchor)      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   TimescaleDB   │    │   Monitoring    │    │   Solana RPC    │
│   (Database)    │    │ (Grafana/Prom)  │    │   (Blockchain)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Components

#### 🔧 Engine (`/engine`)
The core Rust-based arbitrage engine that:
- Monitors multiple DEXs in real-time using Geyser plugin
- Identifies arbitrage opportunities
- Executes trades automatically
- Manages risk and position sizing
- Provides metrics and logging

#### 🎛️ Dashboard (`/dashboard`)
A modern React/TypeScript web interface that provides:
- Real-time monitoring of arbitrage opportunities
- Performance analytics and metrics
- Configuration management
- Historical data visualization
- System health monitoring

#### 📜 Programs (`/programs`)
Solana programs (smart contracts) written in Anchor framework:
- **Cross-DEX Router**: Routes trades across multiple DEXs
- **Flash Arbitrage**: Executes flash loan arbitrage strategies
- **MEV Protection**: Protects against front-running and sandwich attacks

#### 🗄️ Database
TimescaleDB for storing:
- Historical price data
- Trade execution records
- Performance metrics
- System logs and events

## 🛠️ Technologies Used

- **Backend**: Rust, Tokio, SQLx, TimescaleDB
- **Frontend**: React, TypeScript, Vite, Tailwind CSS
- **Blockchain**: Solana, Anchor Framework
- **Database**: TimescaleDB (PostgreSQL extension)
- **Monitoring**: Grafana, Prometheus
- **Infrastructure**: Docker, Docker Compose
- **Build Tools**: Cargo, npm/yarn, Make

## 📋 Prerequisites

Before running the project, ensure you have:

- **Rust** (latest stable version)
- **Node.js** (v18 or higher)
- **Solana CLI** (v1.16 or higher)
- **Anchor CLI** (v0.28 or higher)
- **Docker** and **Docker Compose**
- **PostgreSQL** with TimescaleDB extension

## 🚀 Installation

### 1. Clone the Repository

```bash
git clone https://github.com/joaquinbejar/DeFi-Arbitrage-Engine.git
cd DeFi-Arbitrage-Engine
```

### 2. Environment Setup

```bash
# Copy environment template
cp .env.example .env

# Edit configuration
vim .env
```

### 3. Database Setup

```bash
# Start TimescaleDB with Docker (compose file lives in Docker/)
docker compose -f Docker/docker-compose.yml up -d timescaledb

# Run migrations
make migrate
```

### 4. Build the Project

```bash
# Build all components
make build

# Or build individually
make build-engine    # Build Rust engine
make build-programs  # Build Anchor programs
make build-dashboard # Build React dashboard
```

### 5. Deploy Programs (Devnet)

```bash
# Deploy Anchor programs to devnet
anchor build
anchor deploy --provider.cluster devnet
```

## 🎯 Usage

### Development Mode

```bash
# Start all services in development mode
make dev

# Or start individually
make dev-engine     # Start engine in development
make dev-dashboard  # Start dashboard dev server
make dev-monitoring # Start monitoring stack
```

### Production Mode

```bash
# Start all services in production mode
docker compose -f Docker/docker-compose.yml up -d
```

### Accessing Services

- **Dashboard**: http://localhost:3000
- **Engine API**: http://localhost:8080
- **Grafana**: http://localhost:3001 (admin/admin)
- **Prometheus**: http://localhost:9090

## 📊 Monitoring

The project includes comprehensive monitoring:

### Metrics
- Trade execution latency
- Profit/loss tracking
- System resource usage
- DEX connection status
- Error rates and alerts

### Dashboards
- **Arbitrage Performance**: P&L, success rates, volume
- **System Health**: CPU, memory, network usage
- **Market Data**: Price feeds, spreads, opportunities

## 🔧 Configuration

### Engine Configuration (`config.toml`)

```toml
[database]
url = "postgresql://user:pass@localhost/arbitrage"

[solana]
rpc_url = "https://api.devnet.solana.com"
ws_url = "wss://api.devnet.solana.com"

[strategy]
min_profit_threshold = 0.001  # 0.1%
max_position_size = 1000      # SOL
slippage_tolerance = 0.005    # 0.5%

[dexes]
raydium = { enabled = true, priority = 1 }
orca = { enabled = true, priority = 2 }
serum = { enabled = true, priority = 3 }
```

## 🧪 Testing

```bash
# Run all tests
make test

# Run specific test suites
make test-engine     # Rust tests
make test-programs   # Anchor tests
make test-dashboard  # Frontend tests

# Run integration tests
make test-integration
```

## 📁 Project Structure

```
DeFi-Arbitrage-Engine/
├── engine/                 # Core arbitrage engine (Rust)
│   ├── src/
│   │   ├── config.rs      # Configuration management
│   │   ├── database.rs    # Database operations
│   │   ├── dex.rs         # DEX integrations
│   │   ├── engine.rs      # Main engine logic
│   │   ├── geyser.rs      # Solana Geyser plugin
│   │   ├── strategy.rs    # Arbitrage strategies
│   │   └── models.rs      # Data models
│   └── migrations/        # Database migrations
├── dashboard/             # Web dashboard (React/TS)
│   ├── src/
│   │   ├── components/    # React components
│   │   ├── pages/         # Page components
│   │   ├── stores/        # State management
│   │   └── types/         # TypeScript types
│   └── public/            # Static assets
├── programs/              # Solana programs (Anchor)
│   ├── cross-dex-router/  # Cross-DEX routing
│   ├── flash-arbitrage/   # Flash loan arbitrage
│   └── mev-protection/    # MEV protection
├── monitoring/            # Monitoring configuration
│   ├── grafana/           # Grafana dashboards
│   └── prometheus.yml     # Prometheus config
├── scripts/               # Utility scripts
├── Docker/                # Docker configurations
└── migrations/            # Database migrations
```

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Guidelines

- Follow Rust best practices and use `cargo fmt` and `cargo clippy`
- Write comprehensive tests for new features
- Update documentation for API changes
- Ensure all CI checks pass

### Code Style

- **Rust**: Follow standard Rust formatting (`cargo fmt`)
- **TypeScript**: Use Prettier and ESLint configurations
- **Commit Messages**: Use conventional commit format

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## ⚠️ Disclaimer

This software is provided for educational and research purposes only. Trading cryptocurrencies involves substantial risk of loss and is not suitable for all investors. The authors are not responsible for any financial losses incurred through the use of this software.

**Important Notes:**
- Always test on devnet before using on mainnet
- Start with small position sizes
- Monitor performance and adjust strategies accordingly
- Be aware of market risks and regulatory requirements

## 📈 Roadmap

- [ ] **v1.1**: Additional DEX integrations (Jupiter, Meteora)
- [ ] **v1.2**: Advanced ML-based opportunity detection
- [ ] **v1.3**: Cross-chain arbitrage support
- [ ] **v1.4**: Mobile dashboard application
- [ ] **v2.0**: Institutional-grade features and APIs

## 🆘 Support

If you encounter any issues or have questions:

1. Check the [Issues](https://github.com/joaquinbejar/DeFi-Arbitrage-Engine/issues) page
2. Review the documentation and configuration
3. Join our community discussions
4. Contact the maintainer directly

---

### **Contact Information**
- **Author**: Joaquín Béjar García
- **Email**: jb@taunais.com
- **Telegram**: https://t.me/joaquin_bejar
- **GitHub**: https://github.com/joaquinbejar/DeFi-Arbitrage-Engine

---

**Built with ❤️ for the DeFi community**