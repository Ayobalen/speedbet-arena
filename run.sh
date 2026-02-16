#!/bin/bash
# SpeedBet Arena - Local Development Runner
# This script builds and runs the application locally

set -eu

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  SpeedBet Arena - Local Development${NC}"
echo -e "${BLUE}========================================${NC}"

# Check dependencies
echo -e "\n${YELLOW}Checking dependencies...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not installed${NC}"
    echo "Install from https://rustup.rs/"
    exit 1
fi
echo -e "${GREEN}✓ Cargo found${NC}"

if ! command -v linera &> /dev/null; then
    echo -e "${RED}Error: Linera CLI not installed${NC}"
    echo "Install from https://linera.dev/getting-started/installation.html"
    exit 1
fi
echo -e "${GREEN}✓ Linera CLI found${NC}"

if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo -e "${YELLOW}Adding WASM target...${NC}"
    rustup target add wasm32-unknown-unknown
fi
echo -e "${GREEN}✓ WASM target available${NC}"

# Define paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTRACTS_DIR="${SCRIPT_DIR}/contracts"
FRONTEND_DIR="${SCRIPT_DIR}/frontend"

# Build contracts
echo -e "\n${YELLOW}Building contracts...${NC}"
cd "${CONTRACTS_DIR}"
cargo build --release --target wasm32-unknown-unknown

# Check if build succeeded
if [ ! -f "target/wasm32-unknown-unknown/release/speedbet_arena.wasm" ]; then
    echo -e "${RED}Error: Contract build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Contracts built successfully${NC}"

# Initialize wallet if needed
echo -e "\n${YELLOW}Setting up Linera wallet...${NC}"
if ! linera wallet show 2>/dev/null; then
    echo "Initializing new wallet..."
    linera wallet init --with-new-chain
fi
echo -e "${GREEN}✓ Wallet ready${NC}"

# Get the chain ID
CHAIN_ID=$(linera wallet show | grep -o 'e[0-9a-f]\{64\}' | head -1)
echo -e "${GREEN}Chain ID: ${CHAIN_ID}${NC}"

# Deploy the application
echo -e "\n${YELLOW}Deploying application...${NC}"
cd "${SCRIPT_DIR}"

CONTRACT_PATH="${CONTRACTS_DIR}/target/wasm32-unknown-unknown/release/speedbet_arena_contract.wasm"
SERVICE_PATH="${CONTRACTS_DIR}/target/wasm32-unknown-unknown/release/speedbet_arena_service.wasm"

# Deploy with instantiation arguments
DEPLOY_OUTPUT=$(linera deploy "${CONTRACT_PATH}" "${SERVICE_PATH}" \
    --json-argument '{"fee_bps": 200, "min_bet": "1000000", "max_bet": "100000000"}' \
    2>&1)

# Extract application ID
APP_ID=$(echo "${DEPLOY_OUTPUT}" | grep -o 'e[0-9a-f]\{64\}' | tail -1)
echo -e "${GREEN}Application ID: ${APP_ID}${NC}"

# Start the Linera service
echo -e "\n${YELLOW}Starting Linera service...${NC}"
linera service --port 8080 &
LINERA_PID=$!

# Cleanup function
cleanup() {
    echo -e "\n${YELLOW}Cleaning up...${NC}"
    kill ${LINERA_PID} 2>/dev/null || true
    if [ -n "${FRONTEND_PID:-}" ]; then
        kill ${FRONTEND_PID} 2>/dev/null || true
    fi
    echo -e "${GREEN}Cleanup complete${NC}"
}
trap cleanup EXIT

# Wait for service to start
sleep 3

# Write environment file for frontend
echo -e "\n${YELLOW}Configuring frontend...${NC}"
cat > "${FRONTEND_DIR}/.env" <<EOF
VITE_LINERA_RPC_URL=http://localhost:8080
VITE_APP_ID=${APP_ID}
VITE_CHAIN_ID=${CHAIN_ID}
VITE_FAUCET_URL=
EOF
echo -e "${GREEN}✓ Frontend environment configured${NC}"

# Setup and start frontend
echo -e "\n${YELLOW}Setting up frontend...${NC}"
cd "${FRONTEND_DIR}"

if [ ! -d "node_modules" ]; then
    npm install
fi

npm run dev &
FRONTEND_PID=$!

# Wait for frontend to start
sleep 3

# Print URLs
echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}  SpeedBet Arena is running!${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "\n${BLUE}URLs:${NC}"
echo -e "  Frontend:    http://localhost:5173"
echo -e "  GraphQL:     http://localhost:8080/chains/${CHAIN_ID}/applications/${APP_ID}"
echo -e "\n${BLUE}Chain ID:${NC}    ${CHAIN_ID}"
echo -e "${BLUE}App ID:${NC}      ${APP_ID}"
echo -e "\n${YELLOW}Press Ctrl+C to stop${NC}"

# Wait for user interrupt
wait
