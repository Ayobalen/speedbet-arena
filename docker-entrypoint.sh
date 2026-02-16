#!/bin/sh
# SpeedBet Arena - Docker Entrypoint Script
# This script initializes the container and starts nginx

set -e

echo "Starting SpeedBet Arena..."

# Log environment info
echo "LINERA_FAUCET_URL: ${LINERA_FAUCET_URL:-not set}"

# Start nginx
exec nginx -g "daemon off;"
