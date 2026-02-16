//! Operations and messages for SpeedBet Arena

use crate::types::*;
use linera_sdk::linera_base_types::{AccountOwner, Amount, ChainId};
use serde::{Deserialize, Serialize};

/// Arguments provided when instantiating the application
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InstantiationArgument {
    /// Platform fee in basis points (100 = 1%)
    pub fee_bps: u16,
    /// Minimum bet amount
    pub min_bet: Amount,
    /// Maximum bet amount
    pub max_bet: Amount,
}

impl Default for InstantiationArgument {
    fn default() -> Self {
        Self {
            fee_bps: PLATFORM_FEE_BPS,
            min_bet: Amount::from_tokens(MIN_BET as u128),
            max_bet: Amount::from_tokens(MAX_BET as u128),
        }
    }
}

/// Operations that can be performed on the application
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Operation {
    // ============ ADMIN OPERATIONS ============

    /// Update platform settings (admin only)
    UpdateSettings {
        /// New fee in basis points
        fee_bps: Option<u16>,
        /// New minimum bet
        min_bet: Option<Amount>,
        /// New maximum bet
        max_bet: Option<Amount>,
    },

    /// Pause/unpause platform (admin only)
    SetPaused {
        /// Whether to pause
        paused: bool,
    },

    // ============ PLAYER OPERATIONS ============

    /// Deposit funds to user account
    Deposit {
        /// Amount to deposit
        amount: Amount,
    },

    /// Withdraw funds from user account
    Withdraw {
        /// Amount to withdraw
        amount: Amount,
    },

    // ============ MATCHMAKING OPERATIONS ============

    /// Join the matchmaking queue
    JoinQueue {
        /// Asset to bet on
        asset: Asset,
        /// Bet amount
        bet_amount: Amount,
    },

    /// Leave the matchmaking queue
    LeaveQueue,

    // ============ DUEL OPERATIONS ============

    /// Submit prediction for a duel
    SubmitPrediction {
        /// Duel ID
        duel_id: DuelId,
        /// Prediction direction
        direction: Direction,
    },

    /// Start the duel (called after both predict)
    StartDuel {
        /// Duel ID
        duel_id: DuelId,
        /// Starting price
        start_price: u64,
    },

    /// Resolve the duel (called after 60 seconds)
    ResolveDuel {
        /// Duel ID
        duel_id: DuelId,
        /// Ending price
        end_price: u64,
    },

    /// Cancel a duel (timeout or dispute)
    CancelDuel {
        /// Duel ID
        duel_id: DuelId,
        /// Cancellation reason
        reason: String,
    },

    // ============ ORACLE OPERATIONS ============

    /// Update price data (oracle/admin)
    UpdatePrice {
        /// Asset to update
        asset: Asset,
        /// New price
        price: u64,
    },
}

/// Messages sent between chains
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    // ============ LOBBY -> USER ============

    /// Notify user they've been matched
    MatchFound {
        /// Duel ID
        duel_id: DuelId,
        /// Opponent address
        opponent: AccountOwner,
        /// Asset being bet on
        asset: Asset,
        /// Bet amount
        bet_amount: Amount,
        /// Chain where duel is happening
        duel_chain: ChainId,
    },

    // ============ DUEL -> USER ============

    /// Send winnings to user
    Payout {
        /// Duel ID
        duel_id: DuelId,
        /// Payout amount
        amount: Amount,
    },

    /// Refund bet (cancelled duel)
    Refund {
        /// Duel ID
        duel_id: DuelId,
        /// Refund amount
        amount: Amount,
    },

    // ============ DUEL -> LOBBY ============

    /// Notify lobby of duel result
    DuelCompleted {
        /// Duel ID
        duel_id: DuelId,
        /// Winner address
        winner: AccountOwner,
        /// Loser address
        loser: AccountOwner,
        /// Total pot
        pot: Amount,
    },

    /// Notify lobby of duel cancellation
    DuelCancelled {
        /// Duel ID
        duel_id: DuelId,
        /// Cancellation reason
        reason: String,
    },

    // ============ USER -> LOBBY ============

    /// Confirm user has deposited funds
    FundsDeposited {
        /// Player address
        player: AccountOwner,
        /// Deposited amount
        amount: Amount,
    },
}
