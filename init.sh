#!/bin/bash
# SpeedBet Arena - Initialization Script
# Checks prerequisites and builds the project

set -eu

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  SpeedBet Arena - Project Setup${NC}"
echo -e "${BLUE}========================================${NC}"

# Check Rust installation
echo -e "\n${YELLOW}Checking Rust installation...${NC}"
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Rust is not installed!${NC}"
    echo -e "Install Rust from: https://rustup.rs/"
    echo -e "Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi
RUST_VERSION=$(rustc --version)
echo -e "${GREEN}✓ Rust installed: ${RUST_VERSION}${NC}"

# Check Cargo
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Cargo is not installed!${NC}"
    exit 1
fi
CARGO_VERSION=$(cargo --version)
echo -e "${GREEN}✓ Cargo installed: ${CARGO_VERSION}${NC}"

# Check WASM target
echo -e "\n${YELLOW}Checking WASM target...${NC}"
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}Installing wasm32-unknown-unknown target...${NC}"
    rustup target add wasm32-unknown-unknown
fi
echo -e "${GREEN}✓ WASM target (wasm32-unknown-unknown) available${NC}"

# Check Linera CLI
echo -e "\n${YELLOW}Checking Linera CLI...${NC}"
if ! command -v linera &> /dev/null; then
    echo -e "${RED}Linera CLI is not installed!${NC}"
    echo -e ""
    echo -e "Install Linera CLI from source:"
    echo -e "  cargo install linera-service"
    echo -e ""
    echo -e "Or see: https://linera.dev/getting-started/installation.html"
    exit 1
fi
LINERA_VERSION=$(linera --version)
echo -e "${GREEN}✓ Linera CLI installed: ${LINERA_VERSION}${NC}"

# Check Node.js (for frontend)
echo -e "\n${YELLOW}Checking Node.js...${NC}"
if ! command -v node &> /dev/null; then
    echo -e "${YELLOW}Node.js not found. Frontend development requires Node.js.${NC}"
    echo -e "Install from: https://nodejs.org/"
else
    NODE_VERSION=$(node --version)
    echo -e "${GREEN}✓ Node.js installed: ${NODE_VERSION}${NC}"
fi

# Check npm
if command -v npm &> /dev/null; then
    NPM_VERSION=$(npm --version)
    echo -e "${GREEN}✓ npm installed: ${NPM_VERSION}${NC}"
fi

# Build contracts
echo -e "\n${YELLOW}Building contracts...${NC}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${SCRIPT_DIR}/contracts"

echo "Running cargo build --release --target wasm32-unknown-unknown..."
cargo build --release --target wasm32-unknown-unknown

# Verify build outputs
CONTRACT_WASM="target/wasm32-unknown-unknown/release/speedbet_arena_contract.wasm"
SERVICE_WASM="target/wasm32-unknown-unknown/release/speedbet_arena_service.wasm"

if [ -f "${CONTRACT_WASM}" ]; then
    CONTRACT_SIZE=$(ls -lh "${CONTRACT_WASM}" | awk '{print $5}')
    echo -e "${GREEN}✓ Contract WASM built: ${CONTRACT_SIZE}${NC}"
else
    echo -e "${RED}✗ Contract WASM not found${NC}"
    exit 1
fi

if [ -f "${SERVICE_WASM}" ]; then
    SERVICE_SIZE=$(ls -lh "${SERVICE_WASM}" | awk '{print $5}')
    echo -e "${GREEN}✓ Service WASM built: ${SERVICE_SIZE}${NC}"
else
    echo -e "${RED}✗ Service WASM not found${NC}"
    exit 1
fi

# Print usage instructions
echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}  Setup Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "\n${BLUE}Next steps:${NC}"
echo -e ""
echo -e "  ${YELLOW}Local development:${NC}"
echo -e "    ./run.sh"
echo -e ""
echo -e "  ${YELLOW}Deploy to Conway testnet:${NC}"
echo -e "    ./conway_deploy.sh"
echo -e ""
echo -e "  ${YELLOW}Run tests:${NC}"
echo -e "    cd contracts && cargo test"
echo -e ""
echo -e "${BLUE}Documentation:${NC}"
echo -e "  See README.md for full documentation"
echo -e ""
