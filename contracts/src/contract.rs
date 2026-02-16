//! SpeedBet Arena Contract Implementation
//!
//! This is the main contract binary that handles all state mutations.

#![cfg_attr(target_arch = "wasm32", no_main)]

use linera_sdk::{
    abi::WithContractAbi,
    linera_base_types::{AccountOwner, Amount, Timestamp},
    views::{RootView, View},
    Contract, ContractRuntime,
};
use speedbet_arena::{
    InstantiationArgument, SpeedBetAbi, SpeedBetState,
    operations::{Message, Operation},
    types::*,
};
use thiserror::Error;

/// Declare the contract implementation
linera_sdk::contract!(SpeedBetContract);

/// The SpeedBet Arena contract
pub struct SpeedBetContract {
    /// Application state
    state: SpeedBetState,
    /// Contract runtime for chain operations
    runtime: ContractRuntime<Self>,
}

/// Errors that can occur during contract execution
#[derive(Debug, Error)]
pub enum ContractError {
    /// Unauthorized action
    #[error("Unauthorized: only admin can perform this action")]
    Unauthorized,

    /// Platform is paused
    #[error("Platform is currently paused")]
    Paused,

    /// Invalid bet amount
    #[error("Invalid bet amount: must be between {min} and {max}")]
    InvalidBetAmount {
        /// Minimum allowed
        min: Amount,
        /// Maximum allowed
        max: Amount,
    },

    /// Insufficient balance
    #[error("Insufficient balance: have {have}, need {need}")]
    InsufficientBalance {
        /// Current balance
        have: Amount,
        /// Required balance
        need: Amount,
    },

    /// Already in queue
    #[error("Already in queue")]
    AlreadyInQueue,

    /// Not in queue
    #[error("Not in queue")]
    NotInQueue,

    /// Duel not found
    #[error("Duel not found: {0}")]
    DuelNotFound(DuelId),

    /// Invalid duel state
    #[error("Invalid duel state: expected {expected:?}, got {actual:?}")]
    InvalidDuelState {
        /// Expected state
        expected: DuelStatus,
        /// Actual state
        actual: DuelStatus,
    },

    /// Not a participant
    #[error("Not a participant in this duel")]
    NotParticipant,

    /// Already predicted
    #[error("Already predicted")]
    AlreadyPredicted,

    /// Asset not supported
    #[error("Asset not supported: {0:?}")]
    AssetNotSupported(Asset),

    /// State error
    #[error("State error: {0}")]
    StateError(String),
}

impl WithContractAbi for SpeedBetContract {
    type Abi = SpeedBetAbi;
}

impl Contract for SpeedBetContract {
    /// Message type for cross-chain communication
    type Message = Message;

    /// Argument type for instantiation
    type InstantiationArgument = InstantiationArgument;

    /// Application parameters (none for this app)
    type Parameters = ();

    /// Event values for streams (none for this app)
    type EventValue = ();

    /// Load the contract state from storage
    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = SpeedBetState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        SpeedBetContract { state, runtime }
    }

    /// Called once when the application is created
    async fn instantiate(&mut self, argument: Self::InstantiationArgument) {
        // Get the creator as admin
        let admin = self.runtime
            .authenticated_signer()
            .expect("Instantiation must be authenticated");

        // Initialize state
        self.state.initialize(
            admin,
            argument.fee_bps,
            argument.min_bet,
            argument.max_bet,
        ).await;

        log::info!("SpeedBet Arena initialized with admin: {:?}", admin);
    }

    /// Execute an operation
    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            // Admin operations
            Operation::UpdateSettings { fee_bps, min_bet, max_bet } => {
                self.update_settings(fee_bps, min_bet, max_bet).await;
            }
            Operation::SetPaused { paused } => {
                self.set_paused(paused).await;
            }

            // Player operations
            Operation::Deposit { amount } => {
                self.deposit(amount).await;
            }
            Operation::Withdraw { amount } => {
                self.withdraw(amount).await;
            }

            // Matchmaking operations
            Operation::JoinQueue { asset, bet_amount } => {
                self.join_queue(asset, bet_amount).await;
            }
            Operation::LeaveQueue => {
                self.leave_queue().await;
            }

            // Duel operations
            Operation::SubmitPrediction { duel_id, direction } => {
                self.submit_prediction(duel_id, direction).await;
            }
            Operation::StartDuel { duel_id, start_price } => {
                self.start_duel(duel_id, start_price).await;
            }
            Operation::ResolveDuel { duel_id, end_price } => {
                self.resolve_duel(duel_id, end_price).await;
            }
            Operation::CancelDuel { duel_id, reason } => {
                self.cancel_duel(duel_id, reason).await;
            }

            // Oracle operations
            Operation::UpdatePrice { asset, price } => {
                self.update_price(asset, price).await;
            }
        }
    }

    /// Handle incoming cross-chain messages
    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            Message::MatchFound { duel_id, opponent, asset, bet_amount, duel_chain } => {
                // User chain receives notification of match
                let _ = (duel_id, opponent, asset, bet_amount, duel_chain);
            }
            Message::Payout { duel_id, amount } => {
                self.handle_payout(duel_id, amount).await;
            }
            Message::Refund { duel_id, amount } => {
                self.handle_refund(duel_id, amount).await;
            }
            Message::DuelCompleted { duel_id, winner, loser, pot } => {
                self.handle_duel_completed(duel_id, winner, loser, pot).await;
            }
            Message::DuelCancelled { duel_id, reason } => {
                self.handle_duel_cancelled(duel_id, reason).await;
            }
            Message::FundsDeposited { player, amount } => {
                // Debug log removed
            }
        }
    }

    /// Save the state after execution
    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}

// ============ IMPLEMENTATION METHODS ============

impl SpeedBetContract {
    /// Get the authenticated signer or panic
    fn signer(&mut self) -> AccountOwner {
        self.runtime
            .authenticated_signer()
            .expect("Operation must be authenticated")
    }

    /// Get current system time
    fn now(&mut self) -> Timestamp {
        self.runtime.system_time()
    }

    /// Check if platform is paused.
    ///
    /// # Panics
    /// Panics with "Platform is paused" if the paused flag is set to true.
    async fn check_not_paused(&self) {
        if *self.state.paused.get() {
            panic!("Platform is paused");
        }
    }

    /// Check if caller is admin.
    ///
    /// # Panics
    /// Panics with "Unauthorized: admin required" if the caller is not the admin.
    async fn check_admin(&mut self) {
        let signer = self.signer();
        if !self.state.is_admin(&signer).await {
            panic!("Unauthorized: admin required");
        }
    }

    // --- ADMIN METHODS ---

    async fn update_settings(
        &mut self,
        fee_bps: Option<u16>,
        min_bet: Option<Amount>,
        max_bet: Option<Amount>,
    ) {
        self.check_admin().await;

        if let Some(fee) = fee_bps {
            assert!(fee <= 1000, "Fee cannot exceed 10%");
            self.state.fee_bps.set(fee);
        }
        if let Some(min) = min_bet {
            self.state.min_bet.set(min);
        }
        if let Some(max) = max_bet {
            self.state.max_bet.set(max);
        }

        log::info!(
            "Settings updated: fee_bps={:?}, min_bet={:?}, max_bet={:?}",
            fee_bps, min_bet, max_bet
        );
    }

    async fn set_paused(&mut self, paused: bool) {
        self.check_admin().await;
        self.state.paused.set(paused);
        log::info!("Platform paused: {}", paused);
    }

    // --- PLAYER METHODS ---

    async fn deposit(&mut self, amount: Amount) {
        let player = self.signer();

        // In production, this would involve token transfers
        // For MVP, we just credit the balance
        self.state.credit(&player, amount).await;

        log::info!("Player {:?} deposited {:?}", player, amount);
    }

    async fn withdraw(&mut self, amount: Amount) {
        let player = self.signer();

        self.state.debit(&player, amount).await
            .expect("Insufficient balance for withdrawal");

        // In production, this would involve token transfers

        log::info!("Player {:?} withdrew {:?}", player, amount);
    }

    // --- MATCHMAKING METHODS ---

    async fn join_queue(&mut self, asset: Asset, bet_amount: Amount) {
        self.check_not_paused().await;
        let player = self.signer();
        let now = self.now();

        // Check if player is already in queue (double-join prevention)
        assert!(
            !self.state.is_in_queue(&player).await,
            "Already in queue"
        );

        // Validate asset is supported
        match asset {
            Asset::BTC | Asset::ETH => {} // Supported assets
            #[allow(unreachable_patterns)]
            _ => panic!("Unsupported asset type: {:?}", asset),
        }

        // Validate bet amount
        let min_bet = *self.state.min_bet.get();
        let max_bet = *self.state.max_bet.get();
        assert!(
            bet_amount >= min_bet && bet_amount <= max_bet,
            "Bet must be between {:?} and {:?}",
            min_bet, max_bet
        );

        // Check player has sufficient balance
        let balance = self.state.get_balance(&player).await;
        assert!(balance >= bet_amount, "Insufficient balance");

        // Try to find a match
        if let Some(opponent_entry) = self.state.find_match(&player, asset, bet_amount).await {
            // Match found! Create duel
            let duel_id = self.state.next_duel_id().await;

            // Debit both players
            self.state.debit(&opponent_entry.player, bet_amount).await.unwrap();
            self.state.debit(&player, bet_amount).await.unwrap();

            // Create duel
            let duel = DuelInfo::new(
                duel_id,
                opponent_entry.player,  // First in queue is player1
                player,                  // Joiner is player2
                asset,
                bet_amount,
                now,
            );

            self.state.add_duel(duel.clone()).await;

            log::info!(
                "Duel {:?} created between player1={:?} and player2={:?} for {:?} with bet {:?}",
                duel_id,
                opponent_entry.player,
                player,
                asset,
                bet_amount
            );

            // Record stats
            let mut stats1 = self.state.get_stats(&opponent_entry.player).await;
            let mut stats2 = self.state.get_stats(&player).await;
            stats1.record_bet(bet_amount);
            stats2.record_bet(bet_amount);
            self.state.update_stats(&opponent_entry.player, stats1).await;
            self.state.update_stats(&player, stats2).await;

            // Send notifications to both players
            let chain_id = self.runtime.chain_id();

            self.runtime
                .prepare_message(Message::MatchFound {
                    duel_id,
                    opponent: player,
                    asset,
                    bet_amount,
                    duel_chain: chain_id,
                })
                .with_authentication()
                .send_to(chain_id); // In production, send to opponent's chain

            self.runtime
                .prepare_message(Message::MatchFound {
                    duel_id,
                    opponent: opponent_entry.player,
                    asset,
                    bet_amount,
                    duel_chain: chain_id,
                })
                .with_authentication()
                .send_to(chain_id); // In production, send to joiner's chain

        } else {
            // No match, add to queue
            let entry = QueueEntry {
                player,
                asset,
                bet_amount,
                joined_at: now,
            };
            self.state.add_to_queue(entry).await;

            log::info!(
                "Player {:?} joined queue for {:?} with bet {:?}",
                player,
                asset,
                bet_amount
            );
        }
    }

    async fn leave_queue(&mut self) {
        let player = self.signer();

        let removed = self.state.remove_from_queue(&player).await;
        assert!(removed, "Player not in queue");

        log::info!("Player {:?} left queue", player);
    }

    // --- DUEL METHODS ---

    async fn submit_prediction(&mut self, duel_id: DuelId, direction: Direction) {
        self.check_not_paused().await;
        let player = self.signer();

        // Get duel
        let mut duel = self.state.get_duel(duel_id).await
            .expect("Duel not found");

        // Validate state
        assert!(
            duel.status == DuelStatus::WaitingForPredictions,
            "Cannot predict: duel not waiting for predictions"
        );

        // Validate participant
        assert!(duel.is_participant(&player), "Not a participant");

        // Set prediction
        let success = duel.set_prediction(&player, direction);
        assert!(success, "Already predicted");

        // Log prediction
        log::info!(
            "Player {:?} predicted {:?} for duel {:?}",
            player,
            direction,
            duel_id
        );

        // Update duel
        self.state.update_duel(duel.clone()).await;

        // Check if both have predicted
        if duel.both_predicted() {
            log::info!("Duel {:?}: Both players have predicted", duel_id);
            // Frontend will call StartDuel with current price
        }
    }

    async fn start_duel(&mut self, duel_id: DuelId, start_price: u64) {
        let mut duel = self.state.get_duel(duel_id).await
            .expect("Duel not found");

        assert!(
            duel.status == DuelStatus::WaitingForPredictions,
            "Duel not in correct state"
        );
        assert!(duel.both_predicted(), "Both players must predict first");

        // Lock in start price and begin timer
        duel.start_price = Some(start_price);
        duel.started_at = Some(self.now());
        duel.status = DuelStatus::Active;

        self.state.update_duel(duel).await;

        log::info!("Duel {:?} started at price {}", duel_id, start_price);
    }

    async fn resolve_duel(&mut self, duel_id: DuelId, end_price: u64) {
        let mut duel = self.state.get_duel(duel_id).await
            .expect("Duel not found");

        assert!(
            duel.status == DuelStatus::Active,
            "Duel not active"
        );

        // Set end price and determine winner
        duel.end_price = Some(end_price);
        let winner = duel.determine_winner().expect("Could not determine winner");
        let loser = if winner == duel.player1 { duel.player2 } else { duel.player1 };

        // Calculate payout with platform fee
        let fee_bps = *self.state.fee_bps.get();
        let total_pot = duel.total_pot();

        // Calculate fee: fee = total_pot * fee_bps / 10000
        let fee_micros = (u128::from(total_pot) * u128::from(fee_bps) / 10000) as u64;
        let fee = Amount::from_micros(fee_micros.into());
        let payout = total_pot.saturating_sub(fee);

        // Credit winner
        self.state.credit(&winner, payout).await;

        // Update stats
        let mut winner_stats = self.state.get_stats(&winner).await;
        let mut loser_stats = self.state.get_stats(&loser).await;
        winner_stats.record_win(payout);
        loser_stats.record_loss();
        self.state.update_stats(&winner, winner_stats).await;
        self.state.update_stats(&loser, loser_stats).await;

        // Record volume
        self.state.record_volume(total_pot, fee).await;

        // Complete duel
        self.state.complete_duel(duel.clone()).await;

        log::info!(
            "Duel {:?} resolved: winner={:?}, loser={:?}, payout={:?}, fee={:?}",
            duel_id, winner, loser, payout, fee
        );

        // Send payout message
        self.runtime
            .prepare_message(Message::Payout {
                duel_id,
                amount: payout,
            })
            .with_authentication()
            .send_to(self.runtime.chain_id());
    }

    async fn cancel_duel(&mut self, duel_id: DuelId, reason: String) {
        let mut duel = self.state.get_duel(duel_id).await
            .expect("Duel not found");

        assert!(
            duel.status != DuelStatus::Resolved,
            "Cannot cancel resolved duel"
        );

        // Refund both players
        self.state.credit(&duel.player1, duel.bet_amount).await;
        self.state.credit(&duel.player2, duel.bet_amount).await;

        duel.status = DuelStatus::Cancelled;
        self.state.complete_duel(duel.clone()).await;

        log::info!("Duel {:?} cancelled: {}", duel_id, reason);

        // Send refund messages
        self.runtime
            .prepare_message(Message::Refund {
                duel_id,
                amount: duel.bet_amount,
            })
            .with_authentication()
            .send_to(self.runtime.chain_id());
    }

    // --- ORACLE METHODS ---

    async fn update_price(&mut self, asset: Asset, price: u64) {
        let price_data = PriceData {
            asset,
            price,
            timestamp: self.now(),
        };

        self.state.set_price(price_data).await;

        log::info!("Price updated: {:?} = {}", asset, price);
    }

    // --- MESSAGE HANDLERS ---

    async fn handle_payout(&mut self, duel_id: DuelId, amount: Amount) {
        let player = self.signer();
        self.state.credit(&player, amount).await;
        log::info!("Payout received for duel {:?}: {:?} to {:?}", duel_id, amount, player);
    }

    async fn handle_refund(&mut self, duel_id: DuelId, amount: Amount) {
        let player = self.signer();
        self.state.credit(&player, amount).await;
        log::info!("Refund received for duel {:?}: {:?} to {:?}", duel_id, amount, player);
    }

    async fn handle_duel_completed(
        &mut self,
        duel_id: DuelId,
        winner: AccountOwner,
        loser: AccountOwner,
        pot: Amount,
    ) {
        log::info!(
            "Duel {:?} completed: winner={:?}, loser={:?}, pot={:?}",
            duel_id, winner, loser, pot
        );
    }

    async fn handle_duel_cancelled(&mut self, duel_id: DuelId, reason: String) {
        log::info!("Duel {:?} cancelled: {}", duel_id, reason);
    }
}
