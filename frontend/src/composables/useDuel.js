/**
 * Duel composable for SpeedBet Arena
 *
 * Provides reactive state and methods for interacting with duels,
 * matchmaking queue, and player statistics.
 */

import { ref, readonly, computed } from 'vue';
import { useLinera } from './useLinera';
import {
  GET_QUEUE_LENGTH,
  GET_ACTIVE_DUELS,
  GET_RECENT_DUELS,
  GET_LEADERBOARD,
  GET_PLAYER_STATS,
  GET_DUEL,
  JOIN_QUEUE,
  LEAVE_QUEUE,
  SUBMIT_PREDICTION,
} from '../utils/graphql';

// Reactive state
const currentDuel = ref(null);
const inDuel = ref(false);
const queueCount = ref(0);
const isQueued = ref(false);
const timeRemaining = ref(0);
const leaderboard = ref([]);
const recentDuels = ref([]);
const userStats = ref(null);
const currentPrice = ref(0);
const startPrice = ref(0);
const duelTimerInterval = ref(null);

/**
 * Main duel composable
 */
export function useDuel() {
  const { query, mutate, onNotification, chainId } = useLinera();

  /**
   * Fetch current queue count from the contract
   */
  async function fetchQueueCount() {
    try {
      const response = await query(GET_QUEUE_LENGTH);
      if (response?.data?.queueLength !== undefined) {
        queueCount.value = response.data.queueLength;
      }
    } catch (err) {
      console.error('Failed to fetch queue count:', err);
    }
  }

  /**
   * Fetch leaderboard entries
   * @param {number} limit - Maximum entries to fetch
   */
  async function fetchLeaderboard(limit = 10) {
    try {
      const response = await query(GET_LEADERBOARD, { limit });
      if (response?.data?.leaderboard) {
        leaderboard.value = response.data.leaderboard;
      }
      // Also pick up player stats and recent duels if returned (demo mode)
      if (response?.data?.playerStats) {
        userStats.value = response.data.playerStats;
      }
      if (response?.data?.recentDuels) {
        recentDuels.value = response.data.recentDuels;
      }
    } catch (err) {
      console.error('Failed to fetch leaderboard:', err);
    }
  }

  /**
   * Fetch recent duels for history display
   * @param {number} limit - Maximum duels to fetch
   */
  async function fetchRecentDuels(limit = 10) {
    try {
      const response = await query(GET_RECENT_DUELS, { limit });
      if (response?.data?.recentDuels) {
        recentDuels.value = response.data.recentDuels;
      }
    } catch (err) {
      console.error('Failed to fetch recent duels:', err);
    }
  }

  /**
   * Fetch player statistics
   * @param {string} player - Player address
   */
  async function fetchPlayerStats(player) {
    try {
      const response = await query(GET_PLAYER_STATS, { player });
      if (response?.data?.playerStats) {
        userStats.value = response.data.playerStats;
      }
    } catch (err) {
      console.error('Failed to fetch player stats:', err);
    }
  }

  /**
   * Refresh current duel state from the contract
   * @param {string} duelId - ID of the duel to refresh
   */
  async function refreshDuelState(duelId) {
    if (!duelId) return;

    try {
      const response = await query(GET_DUEL, { id: duelId });
      if (response?.data?.duel) {
        currentDuel.value = response.data.duel;
        if (response.data.duel.startPrice) {
          startPrice.value = response.data.duel.startPrice;
        }
        // Update inDuel status based on duel status
        inDuel.value = response.data.duel.status === 'ACTIVE' ||
                       response.data.duel.status === 'PENDING';
      }
    } catch (err) {
      console.error('Failed to refresh duel state:', err);
    }
  }

  /**
   * Join the matchmaking queue
   * @param {string} asset - Asset to bet on ('BTC' or 'ETH')
   * @param {string|number} betAmount - Bet amount in micro-units
   */
  async function joinQueue(asset, betAmount) {
    try {
      const response = await mutate(JOIN_QUEUE, {
        asset: asset.toUpperCase(),
        betAmount: String(betAmount),
      });

      if (response?.errors) {
        throw new Error(response.errors[0]?.message || 'Failed to join queue');
      }

      // Update local state
      isQueued.value = true;

      // Refresh queue count
      await fetchQueueCount();

      console.log('Joined queue:', { asset, betAmount });
      return response;
    } catch (err) {
      console.error('Failed to join queue:', err);
      throw err;
    }
  }

  /**
   * Leave the matchmaking queue
   * Updates UI state to reflect queue departure
   */
  async function leaveQueue() {
    try {
      const response = await mutate(LEAVE_QUEUE);

      if (response?.errors) {
        throw new Error(response.errors[0]?.message || 'Failed to leave queue');
      }

      // Update local state to reflect queue departure
      isQueued.value = false;

      // Refresh queue count
      await fetchQueueCount();

      console.log('Left queue successfully');
      return response;
    } catch (err) {
      console.error('Failed to leave queue:', err);
      throw err;
    }
  }

  /**
   * Poll for match status when in queue
   * Checks if a match has been found and transitions to duel state
   * @param {number} interval - Polling interval in milliseconds
   * @returns {function} Stop polling function
   */
  function pollForMatch(interval = 3000) {
    let polling = true;

    const poll = async () => {
      if (!polling || !isQueued.value) return;

      try {
        // Check for active duels involving this player
        const response = await query(GET_ACTIVE_DUELS);
        if (response?.data?.activeDuels?.length > 0) {
          // Found a match - transition to duel state
          const duel = response.data.activeDuels[0];
          currentDuel.value = duel;
          inDuel.value = true;
          isQueued.value = false;
          startPrice.value = duel.startPrice || 0;
          console.log('Match found!', duel);
          return; // Stop polling
        }

        // Also refresh queue count
        await fetchQueueCount();

        // Continue polling
        if (polling && isQueued.value) {
          setTimeout(poll, interval);
        }
      } catch (err) {
        console.error('Error polling for match:', err);
        if (polling && isQueued.value) {
          setTimeout(poll, interval);
        }
      }
    };

    // Start polling
    poll();

    // Return stop function
    return () => {
      polling = false;
    };
  }

  /**
   * Submit a prediction for the current duel
   * @param {string} direction - 'Up' or 'Down'
   */
  async function submitPrediction(direction) {
    if (!currentDuel.value) {
      throw new Error('No active duel');
    }

    try {
      const response = await mutate(SUBMIT_PREDICTION, {
        duelId: String(currentDuel.value.id),
        direction: direction,
      });

      if (response?.errors) {
        throw new Error(response.errors[0]?.message || 'Failed to submit prediction');
      }

      console.log('Prediction submitted:', direction);
      return response;
    } catch (err) {
      console.error('Failed to submit prediction:', err);
      throw err;
    }
  }

  /**
   * Start the 60-second countdown timer
   * Alias for startDuelTimer for API compatibility
   */
  function startCountdown() {
    startDuelTimer(60000);
  }

  /**
   * Start the duel countdown timer
   * @param {number} durationMs - Duel duration in milliseconds
   */
  function startDuelTimer(durationMs = 60000) {
    // Clear any existing timer
    if (duelTimerInterval.value) {
      clearInterval(duelTimerInterval.value);
    }

    timeRemaining.value = Math.floor(durationMs / 1000);

    duelTimerInterval.value = setInterval(() => {
      if (timeRemaining.value > 0) {
        timeRemaining.value--;
      } else {
        clearInterval(duelTimerInterval.value);
        duelTimerInterval.value = null;
      }
    }, 1000);
  }

  /**
   * Stop the duel timer
   */
  function stopDuelTimer() {
    if (duelTimerInterval.value) {
      clearInterval(duelTimerInterval.value);
      duelTimerInterval.value = null;
    }
    timeRemaining.value = 0;
  }

  /**
   * Handle a new duel notification
   * @param {object} duel - Duel information from notification
   */
  function handleDuelStarted(duel) {
    currentDuel.value = duel;
    inDuel.value = true;
    isQueued.value = false;
    startPrice.value = duel.startPrice || 0;

    // Start the 60-second timer
    startDuelTimer(60000);

    console.log('Duel started:', duel);
  }

  /**
   * Handle duel resolution notification
   * @param {object} result - Duel result information
   */
  function handleDuelResolved(result) {
    // Update current duel with results
    if (currentDuel.value) {
      currentDuel.value = {
        ...currentDuel.value,
        ...result,
        status: 'Resolved',
      };
    }

    // Stop timer
    stopDuelTimer();

    // Wait a moment to show results, then clear
    setTimeout(() => {
      currentDuel.value = null;
      inDuel.value = false;
    }, 5000);

    // Refresh stats
    fetchRecentDuels();
    fetchLeaderboard();

    console.log('Duel resolved:', result);
  }

  /**
   * Setup notification listeners for real-time updates
   */
  function setupNotifications() {
    onNotification((notification) => {
      console.log('Notification received:', notification);

      // Handle different notification types
      // The actual notification structure depends on Linera's format
      // This is a placeholder for the real implementation

      if (notification.duelStarted) {
        handleDuelStarted(notification.duelStarted);
      } else if (notification.duelResolved) {
        handleDuelResolved(notification.duelResolved);
      } else if (notification.matchFound) {
        // Transition from queue to duel
        handleDuelStarted(notification.matchFound);
      }

      // Always refresh queue count on any block update
      fetchQueueCount();
    });
  }

  /**
   * Reset all duel state (for cleanup/disconnect)
   */
  function resetState() {
    currentDuel.value = null;
    inDuel.value = false;
    isQueued.value = false;
    stopDuelTimer();
    leaderboard.value = [];
    recentDuels.value = [];
    userStats.value = null;
    currentPrice.value = 0;
    startPrice.value = 0;
  }

  return {
    // State (read-only)
    currentDuel: readonly(currentDuel),
    inDuel: readonly(inDuel),
    queueCount: readonly(queueCount),
    isQueued: readonly(isQueued),
    timeRemaining: readonly(timeRemaining),
    leaderboard: readonly(leaderboard),
    recentDuels: readonly(recentDuels),
    userStats: readonly(userStats),
    currentPrice,  // Writable - updated by App.vue with external price data
    startPrice: readonly(startPrice),

    // Methods
    fetchQueueCount,
    fetchLeaderboard,
    fetchRecentDuels,
    fetchPlayerStats,
    refreshDuelState,
    pollForMatch,
    joinQueue,
    leaveQueue,
    submitPrediction,
    startCountdown,
    setupNotifications,
    resetState,
  };
}
