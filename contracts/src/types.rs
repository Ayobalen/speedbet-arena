//! Core type definitions for SpeedBet Arena

use linera_sdk::linera_base_types::{AccountOwner, Amount, ChainId, Timestamp};
use serde::{Deserialize, Serialize};

/// Unique identifier for a duel
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct DuelId(pub u64);

impl std::fmt::Display for DuelId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u64> for DuelId {
    fn from(value: u64) -> Self {
        DuelId(value)
    }
}

/// Supported assets for betting
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Asset {
    /// Bitcoin
    BTC,
    /// Ethereum
    ETH,
}

impl Asset {
    /// Returns the symbol string for the asset
    pub fn symbol(&self) -> &'static str {
        match self {
            Asset::BTC => "BTC",
            Asset::ETH => "ETH",
        }
    }
}

impl Default for Asset {
    fn default() -> Self {
        Asset::BTC
    }
}

/// Player's prediction direction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Direction {
    /// Price will go up
    Up,
    /// Price will go down or stay same
    Down,
}

/// Status of a duel
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum DuelStatus {
    /// Waiting for second player
    #[default]
    WaitingForPlayers,
    /// Both players joined, waiting for predictions
    WaitingForPredictions,
    /// Predictions locked, timer running
    Active,
    /// Winner determined
    Resolved,
    /// Duel cancelled (timeout, etc.)
    Cancelled,
}

/// Player statistics
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct PlayerStats {
    /// Total wins
    pub wins: u64,
    /// Total losses
    pub losses: u64,
    /// Total amount wagered
    pub total_wagered: Amount,
    /// Total amount won
    pub total_won: Amount,
    /// Current win streak
    pub win_streak: u64,
    /// Best win streak ever
    pub best_streak: u64,
}

impl PlayerStats {
    /// Create new empty stats
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a win
    pub fn record_win(&mut self, amount: Amount) {
        self.wins += 1;
        self.total_won = self.total_won.saturating_add(amount);
        self.win_streak += 1;
        if self.win_streak > self.best_streak {
            self.best_streak = self.win_streak;
        }
    }

    /// Record a loss
    pub fn record_loss(&mut self) {
        self.losses += 1;
        self.win_streak = 0;
    }

    /// Record a bet
    pub fn record_bet(&mut self, amount: Amount) {
        self.total_wagered = self.total_wagered.saturating_add(amount);
    }

    /// Calculate win rate as percentage (0-100)
    pub fn win_rate(&self) -> u64 {
        let total = self.wins + self.losses;
        if total == 0 {
            return 0;
        }
        (self.wins * 100) / total
    }
}

/// Queue entry for matchmaking
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueEntry {
    /// Player's owner address
    pub player: AccountOwner,
    /// Asset to bet on
    pub asset: Asset,
    /// Bet amount in micro-units
    pub bet_amount: Amount,
    /// When player joined queue
    pub joined_at: Timestamp,
}

/// Information about a duel (stored in Lobby)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DuelInfo {
    /// Unique duel identifier
    pub id: DuelId,
    /// First player (joined first)
    pub player1: AccountOwner,
    /// Second player
    pub player2: AccountOwner,
    /// Asset being bet on
    pub asset: Asset,
    /// Bet amount per player
    pub bet_amount: Amount,
    /// Current status
    pub status: DuelStatus,
    /// When duel was created
    pub created_at: Timestamp,
    /// Winner (if resolved)
    pub winner: Option<AccountOwner>,
    /// Player 1's prediction
    pub p1_prediction: Option<Direction>,
    /// Player 2's prediction
    pub p2_prediction: Option<Direction>,
    /// Start price (when predictions locked)
    pub start_price: Option<u64>,
    /// End price (when resolved)
    pub end_price: Option<u64>,
    /// When duel started (predictions locked)
    pub started_at: Option<Timestamp>,
}

impl DuelInfo {
    /// Create a new duel
    pub fn new(
        id: DuelId,
        player1: AccountOwner,
        player2: AccountOwner,
        asset: Asset,
        bet_amount: Amount,
        created_at: Timestamp,
    ) -> Self {
        Self {
            id,
            player1,
            player2,
            asset,
            bet_amount,
            status: DuelStatus::WaitingForPredictions,
            created_at,
            winner: None,
            p1_prediction: None,
            p2_prediction: None,
            start_price: None,
            end_price: None,
            started_at: None,
        }
    }

    /// Check if both players have predicted
    pub fn both_predicted(&self) -> bool {
        self.p1_prediction.is_some() && self.p2_prediction.is_some()
    }

    /// Check if a player is a participant
    pub fn is_participant(&self, player: &AccountOwner) -> bool {
        self.player1 == *player || self.player2 == *player
    }

    /// Get the prediction for a player
    pub fn get_prediction(&self, player: &AccountOwner) -> Option<Direction> {
        if self.player1 == *player {
            self.p1_prediction
        } else if self.player2 == *player {
            self.p2_prediction
        } else {
            None
        }
    }

    /// Set prediction for a player
    pub fn set_prediction(&mut self, player: &AccountOwner, direction: Direction) -> bool {
        if self.player1 == *player && self.p1_prediction.is_none() {
            self.p1_prediction = Some(direction);
            true
        } else if self.player2 == *player && self.p2_prediction.is_none() {
            self.p2_prediction = Some(direction);
            true
        } else {
            false
        }
    }

    /// Determine the winner based on price movement
    pub fn determine_winner(&mut self) -> Option<AccountOwner> {
        let start = self.start_price?;
        let end = self.end_price?;
        let p1_pred = self.p1_prediction?;
        let p2_pred = self.p2_prediction?;

        let actual_direction = if end > start {
            Direction::Up
        } else {
            Direction::Down
        };

        let p1_correct = p1_pred == actual_direction;
        let p2_correct = p2_pred == actual_direction;

        self.winner = match (p1_correct, p2_correct) {
            (true, false) => Some(self.player1),
            (false, true) => Some(self.player2),
            // Both correct or both wrong - tie goes to player who predicted first
            // In this implementation, player1 always predicts first by design
            (true, true) => Some(self.player1),
            (false, false) => Some(self.player1),
        };

        self.status = DuelStatus::Resolved;
        self.winner
    }

    /// Calculate total pot (both bets combined)
    pub fn total_pot(&self) -> Amount {
        self.bet_amount.saturating_add(self.bet_amount)
    }
}

/// Leaderboard entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    /// Player address
    pub player: AccountOwner,
    /// Player statistics
    pub stats: PlayerStats,
    /// Current rank
    pub rank: u64,
}

/// Price data from oracle
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceData {
    /// Asset type
    pub asset: Asset,
    /// Price in micro-USD (1 USD = 1_000_000)
    pub price: u64,
    /// When price was fetched
    pub timestamp: Timestamp,
}

/// Duration of a duel in microseconds (60 seconds)
pub const DUEL_DURATION_MICROS: u64 = 60_000_000;

/// Queue timeout in microseconds (5 minutes)
pub const QUEUE_TIMEOUT_MICROS: u64 = 300_000_000;

/// Minimum bet amount in micro-units
pub const MIN_BET: u64 = 1_000_000; // 1 token

/// Maximum bet amount in micro-units
pub const MAX_BET: u64 = 100_000_000; // 100 tokens

/// Platform fee in basis points (200 = 2%)
pub const PLATFORM_FEE_BPS: u16 = 200;
