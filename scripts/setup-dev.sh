#!/bin/bash

# DeFi Arbitrage Engine Development Environment Setup Script
# This script sets up the complete development environment

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOLANA_VERSION="1.18.8"
ANCHOR_VERSION="0.29.0"
RUST_VERSION="1.89.0"
NODE_VERSION="20"

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

error_exit() {
    log_error "$1"
    exit 1
}

# Check if running on supported OS
check_os() {
    log_info "Checking operating system..."
    
    case "$(uname -s)" in
        Darwin*)
            OS="macos"
            log_success "macOS detected"
            ;;
        Linux*)
            OS="linux"
            log_success "Linux detected"
            ;;
        *)
            error_exit "Unsupported operating system: $(uname -s)"
            ;;
    esac
}

# Check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Install Homebrew (macOS only)
install_homebrew() {
    if [ "$OS" = "macos" ] && ! command_exists brew; then
        log_info "Installing Homebrew..."
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
        log_success "Homebrew installed"
    fi
}

# Install system dependencies
install_system_deps() {
    log_info "Installing system dependencies..."
    
    if [ "$OS" = "macos" ]; then
        # macOS dependencies
        brew update
        brew install curl git pkg-config openssl libuv cmake llvm timescaledb redis
        
        # Install Docker Desktop if not present
        if ! command_exists docker; then
            log_warning "Docker Desktop not found. Please install it manually from https://docker.com/products/docker-desktop"
        fi
        
    elif [ "$OS" = "linux" ]; then
        # Linux dependencies
        if command_exists apt-get; then
            sudo apt-get update
            sudo apt-get install -y curl git build-essential pkg-config libssl-dev libuv1-dev cmake llvm-dev libclang-dev timescaledb-client redis-tools docker.io docker-compose
        elif command_exists yum; then
            sudo yum update -y
            sudo yum install -y curl git gcc gcc-c++ pkgconfig openssl-devel libuv-devel cmake llvm-devel clang-devel timescaledb redis docker docker-compose
        else
            error_exit "Unsupported Linux distribution. Please install dependencies manually."
        fi
        
        # Add user to docker group
        sudo usermod -aG docker "$USER" || log_warning "Could not add user to docker group"
    fi
    
    log_success "System dependencies installed"
}

# Install Rust
install_rust() {
    if ! command_exists rustc; then
        log_info "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain "$RUST_VERSION"
        source "$HOME/.cargo/env"
        log_success "Rust $RUST_VERSION installed"
    else
        log_info "Rust already installed: $(rustc --version)"
    fi
    
    # Install additional Rust components
    log_info "Installing Rust components..."
    rustup component add rustfmt clippy
    rustup target add wasm32-unknown-unknown
    
    # Install cargo tools
    log_info "Installing cargo tools..."
    cargo install --force cargo-watch cargo-edit cargo-audit sqlx-cli --features postgres
}

# Install Node.js
install_nodejs() {
    if ! command_exists node; then
        log_info "Installing Node.js..."
        
        if [ "$OS" = "macos" ]; then
            brew install node@$NODE_VERSION
        elif [ "$OS" = "linux" ]; then
            curl -fsSL https://deb.nodesource.com/setup_${NODE_VERSION}.x | sudo -E bash -
            sudo apt-get install -y nodejs
        fi
        
        log_success "Node.js installed: $(node --version)"
    else
        log_info "Node.js already installed: $(node --version)"
    fi
    
    # Install global npm packages
    log_info "Installing global npm packages..."
    npm install -g yarn pnpm typescript ts-node
}

# Install Solana CLI
install_solana() {
    if ! command_exists solana; then
        log_info "Installing Solana CLI..."
        sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_VERSION}/install)"
        
        # Add to PATH
        export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"
        echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> "$HOME/.bashrc"
        echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> "$HOME/.zshrc"
        
        log_success "Solana CLI $SOLANA_VERSION installed"
    else
        log_info "Solana CLI already installed: $(solana --version)"
    fi
    
    # Configure Solana for development
    log_info "Configuring Solana for development..."
    solana config set --url localhost
    solana-keygen new --no-bip39-passphrase --silent --outfile "$HOME/.config/solana/id.json" || true
}

# Install Anchor
install_anchor() {
    if ! command_exists anchor; then
        log_info "Installing Anchor CLI..."
        
        # Install avm (Anchor Version Manager)
        cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
        
        # Install specific Anchor version
        avm install "$ANCHOR_VERSION"
        avm use "$ANCHOR_VERSION"
        
        log_success "Anchor CLI $ANCHOR_VERSION installed"
    else
        log_info "Anchor CLI already installed: $(anchor --version)"
    fi
}

# Setup project environment
setup_project() {
    log_info "Setting up project environment..."
    
    cd "$PROJECT_ROOT"
    
    # Create necessary directories
    mkdir -p logs backups temp
    
    # Copy environment template if it doesn't exist
    if [ ! -f ".env" ]; then
        log_info "Creating .env file from template..."
        cat > .env << EOF
# Database Configuration
DATABASE_URL=timescaledb://arbitrage_user:arbitrage_pass@localhost:5432/arbitrage_db
REDIS_URL=redis://localhost:6379

# Solana Configuration
SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
SOLANA_WS_URL=wss://api.mainnet-beta.solana.com
SOLANA_PRIVATE_KEY_PATH=/path/to/your/keypair.json

# DEX Configuration
JUPITER_API_URL=https://quote-api.jup.ag/v6
RAYDIUM_API_URL=https://api.raydium.io/v2
ORCA_API_URL=https://api.orca.so

# Trading Configuration
MAX_SLIPPAGE=0.01
MIN_PROFIT_THRESHOLD=0.001
MAX_POSITION_SIZE=1000

# Monitoring
PROMETHEUS_PORT=9090
GRAFANA_PORT=3000

# Development
RUST_LOG=info
RUST_BACKTRACE=1
EOF
        log_success ".env file created"
    fi
    
    # Make scripts executable
    chmod +x scripts/*.sh
    
    # Build Rust project
    log_info "Building Rust project..."
    cargo build
    
    # Build Anchor programs
    if [ -d "programs" ]; then
        log_info "Building Anchor programs..."
        anchor build
    fi
    
    log_success "Project setup completed"
}

# Start development services
start_services() {
    log_info "Starting development services..."
    
    # Check if Docker is running
    if ! docker info >/dev/null 2>&1; then
        log_warning "Docker is not running. Please start Docker and run this script again."
        return 1
    fi
    
    # Start services with Docker Compose
    docker-compose up -d timescaledb redis prometheus grafana
    
    # Wait for services to be ready
    log_info "Waiting for services to be ready..."
    sleep 10
    
    # Run database migrations
    log_info "Running database migrations..."
    if [ -f "migrations/001_init_database.sql" ]; then
        PGPASSWORD=arbitrage_pass psql -h localhost -p 5432 -U arbitrage_user -d arbitrage_db -f migrations/001_init_database.sql || \
            log_warning "Could not run database migrations. Please run them manually."
    fi
    
    log_success "Development services started"
}

# Verify installation
verify_installation() {
    log_info "Verifying installation..."
    
    local errors=0
    
    # Check Rust
    if command_exists rustc; then
        log_success "Rust: $(rustc --version)"
    else
        log_error "Rust not found"
        ((errors++))
    fi
    
    # Check Cargo
    if command_exists cargo; then
        log_success "Cargo: $(cargo --version)"
    else
        log_error "Cargo not found"
        ((errors++))
    fi
    
    # Check Node.js
    if command_exists node; then
        log_success "Node.js: $(node --version)"
    else
        log_error "Node.js not found"
        ((errors++))
    fi
    
    # Check Solana
    if command_exists solana; then
        log_success "Solana CLI: $(solana --version)"
    else
        log_error "Solana CLI not found"
        ((errors++))
    fi
    
    # Check Anchor
    if command_exists anchor; then
        log_success "Anchor CLI: $(anchor --version)"
    else
        log_error "Anchor CLI not found"
        ((errors++))
    fi
    
    # Check Docker
    if command_exists docker; then
        log_success "Docker: $(docker --version)"
    else
        log_error "Docker not found"
        ((errors++))
    fi
    
    if [ $errors -eq 0 ]; then
        log_success "All tools installed successfully!"
        return 0
    else
        log_error "$errors tools are missing or not properly installed"
        return 1
    fi
}

# Print usage information
usage() {
    echo "Usage: $0 [OPTIONS]"
    echo "Options:"
    echo "  -h, --help              Show this help message"
    echo "  -s, --skip-system       Skip system dependencies installation"
    echo "  -n, --no-services       Don't start development services"
    echo "  -v, --verify-only       Only verify existing installation"
    echo "  --solana-version VER    Solana version to install (default: $SOLANA_VERSION)"
    echo "  --anchor-version VER    Anchor version to install (default: $ANCHOR_VERSION)"
    echo "  --rust-version VER      Rust version to install (default: $RUST_VERSION)"
}

# Parse command line arguments
SKIP_SYSTEM=false
NO_SERVICES=false
VERIFY_ONLY=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            usage
            exit 0
            ;;
        -s|--skip-system)
            SKIP_SYSTEM=true
            shift
            ;;
        -n|--no-services)
            NO_SERVICES=true
            shift
            ;;
        -v|--verify-only)
            VERIFY_ONLY=true
            shift
            ;;
        --solana-version)
            SOLANA_VERSION="$2"
            shift 2
            ;;
        --anchor-version)
            ANCHOR_VERSION="$2"
            shift 2
            ;;
        --rust-version)
            RUST_VERSION="$2"
            shift 2
            ;;
        *)
            error_exit "Unknown option: $1"
            ;;
    esac
done

# Main execution
main() {
    log_info "Starting DeFi Arbitrage Engine development environment setup..."
    
    if [ "$VERIFY_ONLY" = true ]; then
        verify_installation
        exit $?
    fi
    
    check_os
    
    if [ "$SKIP_SYSTEM" != true ]; then
        install_homebrew
        install_system_deps
    fi
    
    install_rust
    install_nodejs
    install_solana
    install_anchor
    setup_project
    
    if [ "$NO_SERVICES" != true ]; then
        start_services
    fi
    
    verify_installation
    
    log_success "Development environment setup completed!"
    log_info "Next steps:"
    log_info "1. Review and update the .env file with your configuration"
    log_info "2. Run 'cargo test' to verify the setup"
    log_info "3. Run 'cargo run' to start the arbitrage engine"
    log_info "4. Access Grafana at http://localhost:3000 (admin/admin)"
    log_info "5. Access Prometheus at http://localhost:9090"
}

# Run main function
main "$@"