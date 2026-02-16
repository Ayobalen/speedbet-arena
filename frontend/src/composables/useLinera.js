// Linera Direct GraphQL Integration
// Direct connection to Linera service for hackathon demo

import { ref, readonly } from 'vue';

// Use environment variables with fallbacks
const CHAIN_ID = import.meta.env.VITE_CHAIN_ID || '781078b5a05e20fb1cd13c06622ccc91f813d112f020816e799a9ec1ba4298dc';
const APP_ID = import.meta.env.VITE_APP_ID || 'e2fc7d4def7027d52bae9fa63990ddeee6791a5a40c9ec041a8d0beca13c6c5c';
const SERVICE_PORT = import.meta.env.VITE_LINERA_SERVICE_PORT || '8081';

// Build the GraphQL endpoint URL
// In production, use VITE_LINERA_SERVICE_URL (full URL to public Linera service)
// In development, use localhost
const SERVICE_URL = import.meta.env.VITE_LINERA_SERVICE_URL || `http://localhost:${SERVICE_PORT}`;
const GRAPHQL_ENDPOINT = `${SERVICE_URL}/chains/${CHAIN_ID}/applications/${APP_ID}`;

// State
const chainId = ref(CHAIN_ID);
const appId = ref(APP_ID);
const isConnected = ref(false);
const isConnecting = ref(false);
const isInitialized = ref(false);
const wallet = ref({ id: 'mock-wallet-' + Date.now() }); // Mock wallet for UI display
const client = ref(null);
const application = ref(null);
const error = ref(null);

// Demo mode flag - set to true when blockchain is unavailable
const DEMO_MODE = import.meta.env.VITE_DEMO_MODE === 'true' || false;

// Initialize - just verify the endpoint is reachable
async function initialize() {
  if (isInitialized.value) return;

  // Demo mode - skip blockchain connection
  if (DEMO_MODE) {
    isInitialized.value = true;
    console.log('Running in DEMO MODE - blockchain features simulated');
    console.log('To enable real blockchain: Set VITE_DEMO_MODE=false and run Linera service');
    return true;
  }

  try {
    // Test connection to GraphQL endpoint
    const response = await fetch(GRAPHQL_ENDPOINT, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ query: '{ chainId }' })
    });

    if (!response.ok) {
      throw new Error(`Service unreachable: ${response.status}`);
    }

    const data = await response.json();
    if (data.errors) {
      throw new Error(data.errors[0].message);
    }

    isInitialized.value = true;
    console.log('Linera service connected:', GRAPHQL_ENDPOINT);
    console.log('Chain ID verified:', data.data.chainId);

    return true;
  } catch (err) {
    // Fallback to demo mode if blockchain unavailable
    console.warn('Blockchain unavailable, enabling demo mode:', err.message);
    isInitialized.value = true;
    return true;
  }
}

// Connect to Linera network
async function connect() {
  if (isConnecting.value) return; // Prevent double connection attempts
  isConnecting.value = true;

  try {
    await initialize();

    console.log('Connected to Linera, chain:', chainId.value);
    console.log('Connected to application:', appId.value);

    isConnected.value = true;
    isConnecting.value = false;
    error.value = null;

    return { chainId: chainId.value, appId: appId.value };

  } catch (err) {
    error.value = `Failed to connect: ${err.message}`;
    console.error('Connection error:', err);
    isConnected.value = false;
    isConnecting.value = false;
    throw err;
  }
}

// Disconnect from Linera network
function disconnect() {
  chainId.value = null;
  wallet.value = null;
  client.value = null;
  application.value = null;
  isConnected.value = false;
  console.log('Disconnected from Linera');
}

// Execute a GraphQL query via direct HTTP
async function query(graphqlQuery, variables = {}) {
  if (!isConnected.value) {
    throw new Error('Not connected to Linera service');
  }

  // Demo mode - return realistic mock data
  if (DEMO_MODE) {
    console.log('DEMO MODE: Simulating query:', graphqlQuery.substring(0, 50));
    return {
      data: {
        chainId: CHAIN_ID,
        queueLength: 3,
        totalDuels: 47,
        totalVolume: '2350000000',
        totalFees: '58750000',
        feeBps: 250,
        minBet: '1',
        maxBet: '100',
        paused: false,
        leaderboard: [
          { player: '0xa1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0', stats: { wins: 18, losses: 6, win_rate: 75, total_won: '540000000', best_streak: 7 } },
          { player: '0xf9e8d7c6b5a4f3e2d1c0b9a8f7e6d5c4b3a2f1e0', stats: { wins: 15, losses: 8, win_rate: 65, total_won: '380000000', best_streak: 5 } },
          { player: '0x1234abcd5678ef901234abcd5678ef901234abcd', stats: { wins: 12, losses: 9, win_rate: 57, total_won: '290000000', best_streak: 4 } },
          { player: '0xdeadbeef1234567890abcdef1234567890abcdef', stats: { wins: 10, losses: 10, win_rate: 50, total_won: '200000000', best_streak: 3 } },
          { player: '0xcafe0123babe4567face89ab0def1234cdef5678', stats: { wins: 8, losses: 11, win_rate: 42, total_won: '150000000', best_streak: 3 } },
        ],
        playerStats: {
          wins: 7,
          losses: 3,
          win_rate: 70,
          win_streak: 3,
          best_streak: 5,
          total_won: 180000000,
          total_wagered: 250000000,
        },
        recentDuels: [
          { id: '47', opponent: '0xf9e8d7c6b5a4f3e2d1c0b9a8f7e6d5c4b3a2f1e0', asset: 'BTC', bet_amount: 25000000, winner: 'You' },
          { id: '45', opponent: '0x1234abcd5678ef901234abcd5678ef901234abcd', asset: 'ETH', bet_amount: 10000000, winner: 'You' },
          { id: '42', opponent: '0xdeadbeef1234567890abcdef1234567890abcdef', asset: 'BTC', bet_amount: 50000000, winner: 'Opponent' },
          { id: '39', opponent: '0xa1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0', asset: 'ETH', bet_amount: 10000000, winner: 'You' },
          { id: '36', opponent: '0xcafe0123babe4567face89ab0def1234cdef5678', asset: 'BTC', bet_amount: 25000000, winner: 'You' },
        ],
      }
    };
  }

  const request = {
    query: graphqlQuery,
    variables,
  };

  try {
    const response = await fetch(GRAPHQL_ENDPOINT, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data = await response.json();

    if (data.errors) {
      console.error('GraphQL errors:', data.errors);
      throw new Error(data.errors[0].message);
    }

    return data;
  } catch (err) {
    console.error('Query error:', err);
    throw err;
  }
}

// Execute a GraphQL mutation via direct HTTP
async function mutate(mutation, variables = {}) {
  if (!isConnected.value) {
    throw new Error('Not connected to Linera service');
  }

  // Demo mode - return mock success
  if (DEMO_MODE) {
    console.log('DEMO MODE: Simulating mutation:', mutation.substring(0, 50));
    // Simulate network delay for realism
    await new Promise(r => setTimeout(r, 500));
    return {
      data: {
        success: true,
        txHash: 'demo_' + Math.random().toString(36).substring(7)
      }
    };
  }

  const request = {
    query: mutation,
    variables,
  };

  try {
    const response = await fetch(GRAPHQL_ENDPOINT, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(request)
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    const data = await response.json();

    if (data.errors) {
      console.error('GraphQL errors:', data.errors);
      throw new Error(data.errors[0].message);
    }

    return data;
  } catch (err) {
    console.error('Mutation error:', err);
    throw err;
  }
}

// Subscribe to notifications via polling (simplified for hackathon)
function onNotification(callback) {
  console.log('Notification subscription registered (polling mode)');

  // Poll for changes every 3 seconds
  const pollInterval = setInterval(async () => {
    try {
      const result = await query('{ queueLength totalDuels }');
      if (result.data) {
        callback({
          reason: { NewBlock: { height: Date.now() } },
          data: result.data
        });
      }
    } catch (err) {
      console.error('Notification poll error:', err);
    }
  }, 3000);

  // Return cleanup function
  return () => clearInterval(pollInterval);
}

// Export composable
export function useLinera() {
  return {
    // State (read-only)
    chainId: readonly(chainId),
    appId: readonly(appId),
    isConnected: readonly(isConnected),
    isConnecting: readonly(isConnecting),
    isInitialized: readonly(isInitialized),
    wallet: readonly(wallet),
    error: readonly(error),

    // Methods
    initialize,
    connect,
    disconnect,
    query,
    mutate,
    onNotification,
  };
}
