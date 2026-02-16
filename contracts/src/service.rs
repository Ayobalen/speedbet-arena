//! SpeedBet Arena GraphQL Service
//!
//! This service provides read-only access to application state via GraphQL.

#![cfg_attr(target_arch = "wasm32", no_main)]

use std::sync::Arc;
use async_graphql::{EmptySubscription, Object, Schema, SimpleObject, Request, Response};
use linera_sdk::{
    abi::WithServiceAbi,
    linera_base_types::Amount,
    views::View,
    Service, ServiceRuntime,
};
use speedbet_arena::{Operation, SpeedBetAbi, SpeedBetState, types::*};

linera_sdk::service!(SpeedBetService);

/// The SpeedBet Arena service
pub struct SpeedBetService {
    /// Application state (read-only)
    state: SpeedBetState,
    /// Service runtime for chain operations
    runtime: Arc<ServiceRuntime<Self>>,
}

impl WithServiceAbi for SpeedBetService {
    type Abi = SpeedBetAbi;
}

/// Implements the Linera Service trait for SpeedBetService.
///
/// This implementation provides the read-only interface for the SpeedBet Arena application.
/// The service exposes application state via GraphQL queries, allowing clients to fetch
/// platform statistics, duel information, player stats, and leaderboard data.
///
/// # Methods
///
/// - `new()` - Initializes the service by loading state from storage and setting up the runtime
/// - `handle_query()` - Processes incoming GraphQL requests and returns responses
impl Service for SpeedBetService {
    /// Application parameters
    type Parameters = ();

    /// Load the service from storage.
    ///
    /// Initializes the service state from persistent storage and wraps the runtime
    /// in an Arc for shared access during query handling.
    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = SpeedBetState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        Self {
            state,
            runtime: Arc::new(runtime),
        }
    }

    /// Handle GraphQL queries.
    ///
    /// Processes incoming GraphQL requests by building a schema with the current state
    /// and executing the request. Returns platform statistics and duel information
    /// to clients.
    async fn handle_query(&self, request: Request) -> Response {
        // Convert queue entries to GraphQL format
        let queue_entries: Vec<QueueEntryGQL> = self.state.queue.elements().await
            .unwrap_or_default()
            .into_iter()
            .map(|entry| QueueEntryGQL {
                player: format!("{:?}", entry.player),
                asset: format!("{:?}", entry.asset),
                bet_amount: entry.bet_amount.to_string(),
                joined_at: entry.joined_at.micros().to_string(),
            })
            .collect();

        let query_root = QueryRoot {
            chain_id: self.runtime.chain_id().to_string(),
            fee_bps: *self.state.fee_bps.get(),
            min_bet: self.state.min_bet.get().to_string(),
            max_bet: self.state.max_bet.get().to_string(),
            paused: *self.state.paused.get(),
            queue_length: self.state.queue.count() as u64,
            queue: queue_entries,
            total_duels: *self.state.total_duels.get(),
            total_volume: self.state.total_volume.get().to_string(),
            total_fees: self.state.total_fees.get().to_string(),
        };

        let mutation_root = MutationRoot {
            runtime: self.runtime.clone(),
        };

        let schema = Schema::build(query_root, mutation_root, EmptySubscription).finish();
        schema.execute(request).await
    }
}

// ============ GRAPHQL TYPES ============

/// GraphQL representation of a duel for client queries
#[derive(SimpleObject)]
struct DuelInfoGQL {
    /// Unique duel identifier
    id: String,
    /// First player address
    player1: String,
    /// Second player address
    player2: String,
    /// Asset being bet on (BTC or ETH)
    asset: String,
    /// Bet amount per player
    bet_amount: String,
    /// Current duel status
    status: String,
    /// When duel was created
    created_at: String,
    /// Winner address (if resolved)
    winner: Option<String>,
    /// Player 1's prediction (Up or Down)
    p1_prediction: Option<String>,
    /// Player 2's prediction (Up or Down)
    p2_prediction: Option<String>,
    /// Start price when predictions were locked
    start_price: Option<String>,
    /// End price when duel was resolved
    end_price: Option<String>,
    /// When duel started (predictions locked)
    started_at: Option<String>,
}

/// GraphQL representation of player statistics for client queries
#[derive(SimpleObject)]
struct PlayerStatsGQL {
    /// Total number of wins
    wins: String,
    /// Total number of losses
    losses: String,
    /// Total amount wagered (in micro-units)
    total_wagered: String,
    /// Total amount won (in micro-units)
    total_won: String,
    /// Current win streak
    win_streak: String,
    /// Best win streak ever
    best_streak: String,
    /// Win rate percentage (0-100)
    win_rate: String,
}

impl From<&PlayerStats> for PlayerStatsGQL {
    fn from(stats: &PlayerStats) -> Self {
        Self {
            wins: stats.wins.to_string(),
            losses: stats.losses.to_string(),
            total_wagered: stats.total_wagered.to_string(),
            total_won: stats.total_won.to_string(),
            win_streak: stats.win_streak.to_string(),
            best_streak: stats.best_streak.to_string(),
            win_rate: stats.win_rate().to_string(),
        }
    }
}

/// GraphQL representation of a queue entry for client queries
#[derive(SimpleObject)]
struct QueueEntryGQL {
    /// Player address
    player: String,
    /// Asset to bet on (BTC or ETH)
    asset: String,
    /// Bet amount (in micro-units)
    bet_amount: String,
    /// When the player joined the queue
    joined_at: String,
}

/// GraphQL representation of a leaderboard entry for client queries
#[derive(SimpleObject)]
struct LeaderboardEntryGQL {
    /// Player address
    player: String,
    /// Total number of wins
    wins: String,
    /// Total number of losses
    losses: String,
    /// Total amount wagered (in micro-units)
    total_wagered: String,
    /// Total amount won (in micro-units)
    total_won: String,
    /// Current win streak
    win_streak: String,
    /// Best win streak ever
    best_streak: String,
    /// Win rate percentage (0-100)
    win_rate: String,
    /// Current rank on leaderboard
    rank: String,
}

/// GraphQL representation of price data for client queries
#[derive(SimpleObject)]
struct PriceDataGQL {
    /// Asset type (BTC or ETH)
    asset: String,
    /// Price in micro-USD (1 USD = 1_000_000)
    price: String,
    /// When price was fetched
    timestamp: String,
}

/// GraphQL representation of platform info for client queries
#[derive(SimpleObject)]
struct PlatformInfoGQL {
    /// Chain ID of this application
    chain_id: String,
    /// Platform fee in basis points (100 = 1%)
    fee_bps: String,
    /// Minimum bet amount (in micro-units)
    min_bet: String,
    /// Maximum bet amount (in micro-units)
    max_bet: String,
    /// Whether the platform is paused
    paused: bool,
    /// Number of players currently in queue
    queue_length: String,
    /// Total number of completed duels
    total_duels: String,
    /// Total volume traded (in micro-units)
    total_volume: String,
    /// Total fees collected (in micro-units)
    total_fees: String,
}

/// GraphQL Query Root - returns platform statistics
#[derive(SimpleObject)]
struct QueryRoot {
    /// Chain ID of this application
    chain_id: String,
    /// Platform fee in basis points
    fee_bps: u16,
    /// Minimum bet amount
    min_bet: String,
    /// Maximum bet amount
    max_bet: String,
    /// Is platform paused
    paused: bool,
    /// Number of players in queue
    queue_length: u64,
    /// Matchmaking queue entries
    queue: Vec<QueueEntryGQL>,
    /// Total duels completed
    total_duels: u64,
    /// Total volume traded
    total_volume: String,
    /// Total fees collected
    total_fees: String,
}

// ============ MUTATION ROOT ============

/// GraphQL Mutation Root - schedules operations on the contract
struct MutationRoot {
    /// Service runtime for scheduling operations
    runtime: Arc<ServiceRuntime<SpeedBetService>>,
}

#[Object]
impl MutationRoot {
    /// Join matchmaking queue
    ///
    /// Schedules a JoinQueue operation to be executed by the contract.
    async fn join_queue(&self, asset: String, bet_amount: String) -> Vec<u8> {
        let asset_enum = match asset.to_uppercase().as_str() {
            "BTC" => Asset::BTC,
            "ETH" => Asset::ETH,
            _ => panic!("Unsupported asset"),
        };

        let amount: u128 = bet_amount.parse().expect("Invalid bet amount");

        let operation = Operation::JoinQueue {
            asset: asset_enum,
            bet_amount: Amount::from_attos(amount),
        };

        self.runtime.schedule_operation(&operation);
        vec![]
    }

    /// Leave matchmaking queue
    ///
    /// Schedules a LeaveQueue operation to be executed by the contract.
    async fn leave_queue(&self) -> Vec<u8> {
        self.runtime.schedule_operation(&Operation::LeaveQueue);
        vec![]
    }

    /// Submit prediction for a duel
    ///
    /// Schedules a SubmitPrediction operation to be executed by the contract.
    async fn submit_prediction(&self, duel_id: String, direction: String) -> Vec<u8> {
        let id: u64 = duel_id.parse().expect("Invalid duel ID");
        let dir = match direction.to_uppercase().as_str() {
            "UP" => Direction::Up,
            "DOWN" => Direction::Down,
            _ => panic!("Direction must be UP or DOWN"),
        };

        let operation = Operation::SubmitPrediction {
            duel_id: DuelId(id),
            direction: dir,
        };

        self.runtime.schedule_operation(&operation);
        vec![]
    }

    /// Deposit funds
    ///
    /// Schedules a Deposit operation to be executed by the contract.
    async fn deposit(&self, amount: String) -> Vec<u8> {
        let amt: u128 = amount.parse().expect("Invalid amount");

        self.runtime.schedule_operation(&Operation::Deposit {
            amount: Amount::from_attos(amt),
        });
        vec![]
    }

    /// Withdraw funds
    ///
    /// Schedules a Withdraw operation to be executed by the contract.
    async fn withdraw(&self, amount: String) -> Vec<u8> {
        let amt: u128 = amount.parse().expect("Invalid amount");

        self.runtime.schedule_operation(&Operation::Withdraw {
            amount: Amount::from_attos(amt),
        });
        vec![]
    }

    /// Update price (oracle)
    ///
    /// Schedules an UpdatePrice operation to be executed by the contract.
    async fn update_price(&self, asset: String, price: String) -> Vec<u8> {
        let asset_enum = match asset.to_uppercase().as_str() {
            "BTC" => Asset::BTC,
            "ETH" => Asset::ETH,
            _ => panic!("Unsupported asset"),
        };

        let price_val: u64 = price.parse().expect("Invalid price");

        self.runtime.schedule_operation(&Operation::UpdatePrice {
            asset: asset_enum,
            price: price_val,
        });
        vec![]
    }
}
