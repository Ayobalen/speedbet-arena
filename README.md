# SpeedBet Arena

Real-time price prediction duels on Linera. Two players bet on whether BTC or ETH goes up or down in the next 60 seconds. Winner takes the pot minus a 2.5% platform fee.

**Live demo:** [speedbet-arena.vercel.app](https://speedbet-arena.vercel.app)

## How it works

1. Pick an asset (BTC or ETH) and a bet amount
2. Join the matchmaking queue -- the contract pairs you with someone betting the same amount
3. Both players predict UP or DOWN
4. After 60 seconds, the price oracle resolves the duel
5. Correct prediction wins the combined pot

The entire game loop runs on-chain: escrow, matchmaking, resolution, and payouts are all handled by the Linera smart contract.

## Linera features used

- **linera-sdk 0.15.8** -- contract and service development
- **linera-views** -- persistent state with `MapView`, `RegisterView`, and `QueueView`
- **Cross-chain messaging** -- duel coordination across player chains (match notifications, payouts, refunds)
- **GraphQL service layer** -- real-time state queries for the frontend
- **Conway Testnet** -- live deployment on Linera's test network

## Quick start (Docker)

```bash
git clone https://github.com/Ayobalen/speedbet-arena
cd speedbet-arena
docker-compose up --build
```

Open http://localhost:80. This spins up a Linera node, deploys the contract, and serves the frontend.

## Local development

### Prerequisites

- Rust 1.86+ with `wasm32-unknown-unknown` target
- Node.js 20+
- Linera CLI 0.15.8

### Setup

```bash
# Build the contract
cd contracts
cargo build --release --target wasm32-unknown-unknown

# Start the Linera service
export LINERA_WALLET=.linera/wallet.json
export LINERA_STORAGE=rocksdb:.linera/storage.db
linera service --port 8081

# In another terminal, start the frontend
cd frontend
npm install
npm run dev
```

The frontend runs on http://localhost:8082.

## Project structure

```
speedbet-arena/
├── contracts/              # Rust smart contracts (Linera SDK)
│   └── src/
│       ├── lib.rs          # ABI definitions
│       ├── contract.rs     # Game logic (escrow, matchmaking, resolution)
│       ├── service.rs      # GraphQL query/mutation handlers
│       ├── state.rs        # On-chain state (linera-views)
│       ├── types.rs        # Core types (DuelInfo, PlayerStats, etc.)
│       └── operations.rs   # Operations and cross-chain messages
├── frontend/               # Vue 3 + Vite
│   └── src/
│       ├── components/     # UI components (DuelArena, Leaderboard, etc.)
│       ├── composables/    # useLinera, useDuel, useToast
│       └── utils/          # GraphQL client, price feeds, helpers
├── docker-compose.yml      # One-command deployment
├── Dockerfile              # Multi-stage build (Rust + Node + Nginx)
├── conway_deploy.sh        # Testnet deployment script
└── .env.example            # Configuration template
```

## Architecture

### Multi-chain design

- **Lobby chain** -- matchmaking queue, global state, fee collection
- **Duel chains** -- individual game instances with isolated state
- **Player chains** -- per-user balances and match history

### On-chain state

The contract uses `linera-views` for persistent storage:

```rust
MapView<DuelId, DuelInfo>       // active duels
QueueView<QueueEntry>           // matchmaking queue
MapView<Owner, PlayerStats>     // player win/loss records
RegisterView<Amount>            // platform fee totals
```

### Cross-chain messages

```rust
enum Message {
    MatchFound { duel_id, opponent, asset, bet_amount, duel_chain },
    Payout { duel_id, amount },
    Refund { duel_id, amount },
    DuelCompleted { duel_id, winner, loser, pot },
}
```

## Smart contract

The contract (`contracts/src/contract.rs`) handles:

- **Escrow** -- funds are locked when joining the queue, released on resolution
- **Matchmaking** -- players are paired by asset and bet amount
- **Resolution** -- price oracle determines the winner after the duel window
- **Fee collection** -- 2.5% (250 basis points) taken from the pot
- **Admin controls** -- pause/unpause, fee adjustment

## GraphQL API

### Queries

```graphql
query {
  platformInfo {
    chainId
    feeBps
    totalDuels
    queueLength
  }
}

query {
  activeDuels {
    id
    player1
    player2
    asset
    status
  }
}
```

### Mutations

```graphql
mutation {
  joinQueue(asset: "BTC", betAmount: "1000000")
}

mutation {
  submitPrediction(duelId: "1", direction: "UP")
}
```

## Environment variables

Copy `.env.example` to `.env.local` and configure:

```bash
VITE_APP_ID=<application_id>           # from contract deployment
VITE_CHAIN_ID=<chain_id>              # from Linera faucet
VITE_GRAPHQL_ENDPOINT=http://localhost:8080/graphql
```

## License

MIT
