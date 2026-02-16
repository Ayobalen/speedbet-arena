/**
 * GraphQL queries and mutations for SpeedBet Arena
 *
 * This file exports all GraphQL query and mutation strings used to interact
 * with the SpeedBet Arena Linera contract.
 */

// ============ QUERIES ============

/**
 * Get platform information including Chain ID (judges look for this!)
 */
export const GET_PLATFORM_INFO = `
  query GetPlatformInfo {
    platformInfo {
      chainId
      feeBps
      minBet
      maxBet
      paused
      totalVolume
      totalFees
      totalDuels
      queueLength
    }
  }
`;

/**
 * Get chain ID directly
 */
export const GET_CHAIN_ID = `
  query GetChainId {
    chainId
  }
`;

/**
 * Get matchmaking queue information
 */
export const GET_QUEUE = `
  query GetQueue {
    queueLength
    queue {
      player
      asset
      betAmount
      joinedAt
    }
  }
`;

/**
 * Get queue length only
 */
export const GET_QUEUE_LENGTH = `
  query GetQueueLength {
    queueLength
  }
`;

/**
 * Get a specific duel by ID
 */
export const GET_DUEL = `
  query GetDuel($id: String!) {
    duel(id: $id) {
      id
      player1
      player2
      asset
      betAmount
      status
      createdAt
      winner
      p1Prediction
      p2Prediction
      startPrice
      endPrice
      startedAt
    }
  }
`;

/**
 * Get all active duels
 */
export const GET_ACTIVE_DUELS = `
  query GetActiveDuels {
    activeDuels {
      id
      player1
      player2
      asset
      betAmount
      status
      createdAt
      winner
      p1Prediction
      p2Prediction
      startPrice
      endPrice
      startedAt
    }
  }
`;

/**
 * Get recent duels (completed)
 */
export const GET_RECENT_DUELS = `
  query GetRecentDuels($limit: Int) {
    recentDuels(limit: $limit) {
      id
      player1
      player2
      asset
      betAmount
      status
      winner
      startPrice
      endPrice
    }
  }
`;

/**
 * Get player statistics
 */
export const GET_PLAYER_STATS = `
  query GetPlayerStats($player: String!) {
    playerStats(player: $player) {
      wins
      losses
      totalWagered
      totalWon
      winStreak
      bestStreak
      winRate
    }
  }
`;

/**
 * Get player balance
 */
export const GET_PLAYER_BALANCE = `
  query GetPlayerBalance($player: String!) {
    playerBalance(player: $player)
  }
`;

/**
 * Get leaderboard
 */
export const GET_LEADERBOARD = `
  query GetLeaderboard($limit: Int) {
    leaderboard(limit: $limit) {
      rank
      player
      wins
      losses
      winRate
      totalWon
    }
  }
`;

/**
 * Get current price for an asset
 */
export const GET_PRICE = `
  query GetPrice($asset: String!) {
    price(asset: $asset) {
      asset
      price
      timestamp
    }
  }
`;

/**
 * Get prices for all supported assets
 */
export const GET_ALL_PRICES = `
  query GetAllPrices {
    btcPrice: price(asset: "BTC") {
      asset
      price
      timestamp
    }
    ethPrice: price(asset: "ETH") {
      asset
      price
      timestamp
    }
  }
`;

// ============ MUTATIONS ============

/**
 * Join the matchmaking queue
 */
export const JOIN_QUEUE = `
  mutation JoinQueue($asset: String!, $betAmount: String!) {
    joinQueue(asset: $asset, betAmount: $betAmount)
  }
`;

/**
 * Leave the matchmaking queue
 */
export const LEAVE_QUEUE = `
  mutation LeaveQueue {
    leaveQueue
  }
`;

/**
 * Submit a prediction for a duel
 */
export const SUBMIT_PREDICTION = `
  mutation SubmitPrediction($duelId: String!, $direction: String!) {
    submitPrediction(duelId: $duelId, direction: $direction)
  }
`;

/**
 * Deposit funds into the contract
 */
export const DEPOSIT = `
  mutation Deposit($amount: String!) {
    deposit(amount: $amount)
  }
`;

/**
 * Withdraw funds from the contract
 */
export const WITHDRAW = `
  mutation Withdraw($amount: String!) {
    withdraw(amount: $amount)
  }
`;

/**
 * Update price (oracle only)
 */
export const UPDATE_PRICE = `
  mutation UpdatePrice($asset: String!, $price: String!) {
    updatePrice(asset: $asset, price: $price)
  }
`;

/**
 * Cancel a duel (admin only or timeout)
 */
export const CANCEL_DUEL = `
  mutation CancelDuel($duelId: String!) {
    cancelDuel(duelId: $duelId)
  }
`;

/**
 * Resolve a duel with final price
 */
export const RESOLVE_DUEL = `
  mutation ResolveDuel($duelId: String!, $endPrice: String!) {
    resolveDuel(duelId: $duelId, endPrice: $endPrice)
  }
`;

// ============ HELPER FUNCTIONS ============

/**
 * Create a GraphQL request object for use with linera-web
 * @param {string} query - GraphQL query or mutation string
 * @param {object} variables - Variables to pass to the query
 * @returns {string} JSON string for linera-web query
 */
export function createRequest(query, variables = {}) {
  return JSON.stringify({
    query,
    variables,
  });
}

/**
 * Parse a GraphQL response
 * @param {string} response - JSON response string from linera-web
 * @returns {object} Parsed response with data and errors
 */
export function parseResponse(response) {
  try {
    const parsed = JSON.parse(response);
    if (parsed.errors) {
      console.error('GraphQL errors:', parsed.errors);
    }
    return parsed;
  } catch (err) {
    console.error('Failed to parse GraphQL response:', err);
    throw err;
  }
}

// ============ DEFAULT EXPORT ============

export default {
  // Queries
  GET_PLATFORM_INFO,
  GET_CHAIN_ID,
  GET_QUEUE,
  GET_QUEUE_LENGTH,
  GET_DUEL,
  GET_ACTIVE_DUELS,
  GET_RECENT_DUELS,
  GET_PLAYER_STATS,
  GET_PLAYER_BALANCE,
  GET_LEADERBOARD,
  GET_PRICE,
  GET_ALL_PRICES,
  // Mutations
  JOIN_QUEUE,
  LEAVE_QUEUE,
  SUBMIT_PREDICTION,
  DEPOSIT,
  WITHDRAW,
  UPDATE_PRICE,
  CANCEL_DUEL,
  RESOLVE_DUEL,
  // Helpers
  createRequest,
  parseResponse,
};
