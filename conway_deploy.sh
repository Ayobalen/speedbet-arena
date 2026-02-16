#!/bin/bash
# SpeedBet Arena - Conway Testnet Deployment
# Deploys the application to Linera Conway testnet

set -eu

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}  SpeedBet Arena - Conway Deployment${NC}"
echo -e "${BLUE}========================================${NC}"

# Conway testnet configuration
CONWAY_FAUCET_URL="https://faucet.conway-2.linera.net"
CONWAY_RPC_URL="https://rpc.conway-2.linera.net"

# Check dependencies
echo -e "\n${YELLOW}Checking dependencies...${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo not installed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Cargo found${NC}"

if ! command -v linera &> /dev/null; then
    echo -e "${RED}Error: Linera CLI not installed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Linera CLI found${NC}"

if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    rustup target add wasm32-unknown-unknown
fi
echo -e "${GREEN}✓ WASM target available${NC}"

# Define paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTRACTS_DIR="${SCRIPT_DIR}/contracts"

# Build contracts
echo -e "\n${YELLOW}Building contracts for release...${NC}"
cd "${CONTRACTS_DIR}"
cargo build --release --target wasm32-unknown-unknown

echo -e "${GREEN}✓ Contracts built successfully${NC}"

# Initialize wallet with Conway faucet
echo -e "\n${YELLOW}Setting up wallet with Conway faucet...${NC}"

# Check if wallet exists
if linera wallet show 2>/dev/null; then
    echo -e "${YELLOW}Existing wallet found. Using current wallet.${NC}"
else
    echo "Creating new wallet from Conway faucet..."
    linera wallet init --with-new-chain --faucet "${CONWAY_FAUCET_URL}"
fi

# Get chain ID
CHAIN_ID=$(linera wallet show | grep -o 'e[0-9a-f]\{64\}' | head -1)
echo -e "${GREEN}Chain ID: ${CHAIN_ID}${NC}"

# Request tokens from faucet if balance is low
echo -e "\n${YELLOW}Checking balance and requesting tokens if needed...${NC}"
linera faucet claim --faucet "${CONWAY_FAUCET_URL}" || echo "Faucet claim skipped (may already have tokens)"

# Deploy the application
echo -e "\n${YELLOW}Deploying to Conway testnet...${NC}"
cd "${SCRIPT_DIR}"

CONTRACT_PATH="${CONTRACTS_DIR}/target/wasm32-unknown-unknown/release/speedbet_arena_contract.wasm"
SERVICE_PATH="${CONTRACTS_DIR}/target/wasm32-unknown-unknown/release/speedbet_arena_service.wasm"

# Verify WASM files exist
if [ ! -f "${CONTRACT_PATH}" ]; then
    echo -e "${RED}Error: Contract WASM not found at ${CONTRACT_PATH}${NC}"
    exit 1
fi

if [ ! -f "${SERVICE_PATH}" ]; then
    echo -e "${RED}Error: Service WASM not found at ${SERVICE_PATH}${NC}"
    exit 1
fi

# Deploy with instantiation arguments
echo "Deploying contract and service..."
DEPLOY_OUTPUT=$(linera deploy "${CONTRACT_PATH}" "${SERVICE_PATH}" \
    --json-argument '{"fee_bps": 200, "min_bet": "1000000", "max_bet": "100000000"}' \
    2>&1)

echo "${DEPLOY_OUTPUT}"

# Extract application ID (last hex string in output)
APP_ID=$(echo "${DEPLOY_OUTPUT}" | grep -oE '[0-9a-f]{64}' | tail -1)

if [ -z "${APP_ID}" ]; then
    echo -e "${RED}Error: Failed to extract application ID from deployment output${NC}"
    exit 1
fi

echo -e "\n${GREEN}========================================${NC}"
echo -e "${GREEN}  Deployment Successful!${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "\n${BLUE}Chain ID:${NC}"
echo -e "  ${CHAIN_ID}"
echo -e "\n${BLUE}Application ID:${NC}"
echo -e "  ${APP_ID}"
echo -e "\n${BLUE}GraphQL Endpoint:${NC}"
echo -e "  ${CONWAY_RPC_URL}/chains/${CHAIN_ID}/applications/${APP_ID}"
echo -e "\n${BLUE}Frontend Configuration (.env):${NC}"
echo -e "  VITE_LINERA_RPC_URL=${CONWAY_RPC_URL}"
echo -e "  VITE_APP_ID=${APP_ID}"
echo -e "  VITE_CHAIN_ID=${CHAIN_ID}"
echo -e "  VITE_FAUCET_URL=${CONWAY_FAUCET_URL}"

# Save deployment info
cat > "${SCRIPT_DIR}/deployment.json" <<EOF
{
  "network": "conway",
  "chain_id": "${CHAIN_ID}",
  "app_id": "${APP_ID}",
  "rpc_url": "${CONWAY_RPC_URL}",
  "faucet_url": "${CONWAY_FAUCET_URL}",
  "deployed_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
}
EOF

echo -e "\n${GREEN}Deployment info saved to deployment.json${NC}"
