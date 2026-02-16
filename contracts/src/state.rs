//! State definitions using Linera Views

use crate::types::*;
use async_graphql::SimpleObject;
use linera_sdk::linera_base_types::{AccountOwner, Amount};
use linera_sdk::views::{linera_views, MapView, QueueView, RegisterView, RootView, View, ViewStorageContext};

/// Main application state using Linera Views
///
/// This state is shared across the application and persisted to storage.
/// Different chains may have different views of this state based on their role.
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct SpeedBetState {
    // ============ Platform Settings ============

    /// Platform fee in basis points (100 = 1%)
    pub fee_bps: RegisterView<u16>,

    /// Minimum bet amount
    pub min_bet: RegisterView<Amount>,

    /// Maximum bet amount
    pub max_bet: RegisterView<Amount>,

    /// Platform admin address
    pub admin: RegisterView<Option<AccountOwner>>,

    /// Is platform paused
    pub paused: RegisterView<bool>,

    // ============ Matchmaking ============

    /// Matchmaking queue
    pub queue: QueueView<QueueEntry>,

    // ============ Duels ============

    /// Active duels by ID
    pub active_duels: MapView<u64, DuelInfo>,

    /// Completed duels (recent history)
    pub recent_duels: QueueView<DuelInfo>,

    /// Next duel ID counter
    pub next_duel_id: RegisterView<u64>,

    // ============ Players ============

    /// Player statistics by owner
    pub player_stats: MapView<AccountOwner, PlayerStats>,

    /// Player balances
    pub balances: MapView<AccountOwner, Amount>,

    // ============ Prices ============

    /// Current prices by asset
    pub prices: MapView<Asset, PriceData>,

    // ============ Statistics ============

    /// Total platform volume
    pub total_volume: RegisterView<Amount>,

    /// Total fees collected
    pub total_fees: RegisterView<Amount>,

    /// Total duels completed
    pub total_duels: RegisterView<u64>,
}

impl SpeedBetState {
    /// Initialize the state with default values
    pub async fn initialize(
        &mut self,
        admin: AccountOwner,
        fee_bps: u16,
        min_bet: Amount,
        max_bet: Amount,
    ) {
        self.admin.set(Some(admin));
        self.fee_bps.set(fee_bps);
        self.min_bet.set(min_bet);
        self.max_bet.set(max_bet);
        self.paused.set(false);
        self.next_duel_id.set(1);
        self.total_volume.set(Amount::ZERO);
        self.total_fees.set(Amount::ZERO);
        self.total_duels.set(0);
    }

    /// Check if address is admin
    pub async fn is_admin(&self, owner: &AccountOwner) -> bool {
        match self.admin.get() {
            Some(admin) => *admin == *owner,
            None => false,
        }
    }

    /// Get player balance
    pub async fn get_balance(&self, player: &AccountOwner) -> Amount {
        self.balances
            .get(player)
            .await
            .ok()
            .flatten()
            .unwrap_or(Amount::ZERO)
    }

    /// Credit player balance
    pub async fn credit(&mut self, player: &AccountOwner, amount: Amount) {
        let current = self.get_balance(player).await;
        let new_balance = current.saturating_add(amount);
        self.balances.insert(player, new_balance).unwrap();
    }

    /// Debit player balance
    pub async fn debit(&mut self, player: &AccountOwner, amount: Amount) -> Result<(), String> {
        let current = self.get_balance(player).await;
        if current < amount {
            return Err("Insufficient balance".to_string());
        }
        let new_balance = current.saturating_sub(amount);
        self.balances.insert(player, new_balance).unwrap();
        Ok(())
    }

    /// Get player stats
    pub async fn get_stats(&self, player: &AccountOwner) -> PlayerStats {
        self.player_stats
            .get(player)
            .await
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    /// Update player stats
    pub async fn update_stats(&mut self, player: &AccountOwner, stats: PlayerStats) {
        self.player_stats.insert(player, stats).unwrap();
    }

    /// Get next duel ID and increment
    pub async fn next_duel_id(&mut self) -> DuelId {
        let id = *self.next_duel_id.get();
        self.next_duel_id.set(id + 1);
        DuelId(id)
    }

    /// Add entry to queue
    pub async fn add_to_queue(&mut self, entry: QueueEntry) {
        self.queue.push_back(entry);
    }

    /// Find and remove matching queue entry
    pub async fn find_match(
        &mut self,
        player: &AccountOwner,
        asset: Asset,
        bet_amount: Amount,
    ) -> Option<QueueEntry> {
        let entries: Vec<_> = self.queue.elements().await.unwrap_or_default();

        let mut match_index = None;
        let mut matched_entry = None;

        for (index, entry) in entries.iter().enumerate() {
            if entry.asset == asset
                && entry.bet_amount == bet_amount
                && entry.player != *player
            {
                match_index = Some(index);
                matched_entry = Some(entry.clone());
                break;
            }
        }

        if let Some(index) = match_index {
            // Remove from queue (rebuild queue without this entry)
            self.queue.clear();
            for (i, e) in entries.into_iter().enumerate() {
                if i != index {
                    self.queue.push_back(e);
                }
            }
        }

        matched_entry
    }

    /// Remove player from queue
    pub async fn remove_from_queue(&mut self, player: &AccountOwner) -> bool {
        let entries: Vec<_> = self.queue.elements().await.unwrap_or_default();
        let original_len = entries.len();

        self.queue.clear();
        for entry in entries.into_iter() {
            if entry.player != *player {
                self.queue.push_back(entry);
            }
        }

        self.queue.count() < original_len
    }

    /// Add active duel
    pub async fn add_duel(&mut self, duel: DuelInfo) {
        let id = duel.id.0;
        self.active_duels.insert(&id, duel).unwrap();
    }

    /// Get active duel
    pub async fn get_duel(&self, id: DuelId) -> Option<DuelInfo> {
        self.active_duels.get(&id.0).await.ok().flatten()
    }

    /// Update duel
    pub async fn update_duel(&mut self, duel: DuelInfo) {
        let id = duel.id.0;
        self.active_duels.insert(&id, duel).unwrap();
    }

    /// Complete duel and move to history
    pub async fn complete_duel(&mut self, duel: DuelInfo) {
        // Remove from active
        self.active_duels.remove(&duel.id.0).unwrap();

        // Add to recent history
        self.recent_duels.push_back(duel);

        // Keep only last 100 duels
        while self.recent_duels.count() > 100 {
            self.recent_duels.delete_front();
        }

        // Update total duels
        let total = *self.total_duels.get();
        self.total_duels.set(total + 1);
    }

    /// Get current price for asset
    pub async fn get_price(&self, asset: &Asset) -> Option<PriceData> {
        self.prices.get(asset).await.ok().flatten()
    }

    /// Set price for asset
    pub async fn set_price(&mut self, price: PriceData) {
        let asset = price.asset;
        self.prices.insert(&asset, price).unwrap();
    }

    /// Record platform volume and fees
    pub async fn record_volume(&mut self, volume: Amount, fees: Amount) {
        let current_volume = *self.total_volume.get();
        let current_fees = *self.total_fees.get();
        self.total_volume.set(current_volume.saturating_add(volume));
        self.total_fees.set(current_fees.saturating_add(fees));
    }

    /// Get queue length
    pub fn queue_length(&self) -> usize {
        self.queue.count()
    }

    /// Check if player is already in queue
    pub async fn is_in_queue(&self, player: &AccountOwner) -> bool {
        let entries: Vec<_> = self.queue.elements().await.unwrap_or_default();
        entries.iter().any(|e| e.player == *player)
    }
}
