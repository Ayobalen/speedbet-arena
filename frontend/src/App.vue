<template>
  <!-- Toast Notifications -->
  <ToastContainer />

  <!-- Landing Page -->
  <LandingHero v-if="showLanding" @enter="enterArena" />

  <!-- Main App -->
  <div v-else id="app" class="min-h-screen bg-bg-main text-text-main flex flex-col">
    <!-- Header -->
    <header class="sticky top-0 z-50 glass border-b border-border-main">
      <div class="container-app py-4 flex justify-between items-center">
        <button
          @click="goToLanding"
          class="flex items-center gap-3 hover:opacity-80 transition-opacity"
          title="Back to home"
        >
          <!-- Logo -->
          <div class="w-10 h-10 rounded-lg bg-primary/20 flex items-center justify-center">
            <span class="text-xl font-bold text-primary">VS</span>
          </div>
          <div class="text-left">
            <h1 class="text-xl font-bold text-text-main">
              SpeedBet <span class="text-primary">Arena</span>
            </h1>
            <p class="text-xs text-text-muted hidden sm:block">Prediction Combat</p>
          </div>
        </button>
        <WalletConnect />
      </div>
    </header>

    <!-- Chain Info Banner -->
    <ChainInfo />

    <!-- Main Content -->
    <main class="flex-1 container-app py-8">
      <!-- Price Display -->
      <PriceDisplay :btc-price="btcPrice" :eth-price="ethPrice" class="mb-8" />

      <!-- Tab Navigation -->
      <nav class="flex gap-2 mb-8 overflow-x-auto pb-2 scrollbar-hide">
        <button
          v-for="tab in tabs"
          :key="tab.id"
          @click="activeTab = tab.id"
          :class="[
            'px-6 py-3 rounded-xl font-semibold text-sm transition-colors whitespace-nowrap',
            activeTab === tab.id
              ? 'bg-primary text-white'
              : 'bg-bg-surface text-text-muted hover:bg-bg-elevated hover:text-text-main border border-border-main hover:border-border-hover'
          ]"
        >
          <span class="flex items-center gap-2">
            <!-- Arena Icon (Swords) -->
            <svg v-if="tab.id === 'arena'" class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M14.5 17.5L3 6V3h3l11.5 11.5"/><path d="M13 19l6-6"/><path d="M16 16l4 4"/><path d="M19 21l2-2"/><path d="M14.5 6.5L18 3h3v3l-3.5 3.5"/><path d="M5 14l4 4"/><path d="M7 17l-3 3"/>
            </svg>
            <!-- Leaderboard Icon (Trophy) -->
            <svg v-else-if="tab.id === 'leaderboard'" class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/><path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/><path d="M4 22h16"/><path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/><path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/><path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"/>
            </svg>
            <!-- History Icon (Clock) -->
            <svg v-else-if="tab.id === 'history'" class="w-4 h-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/>
            </svg>
            {{ tab.label }}
          </span>
        </button>
      </nav>

      <!-- Tab Content -->
      <div>
        <!-- Arena Tab -->
        <div v-if="activeTab === 'arena'">
          <template v-if="!inDuel">
            <MatchmakingQueue
              @join="handleJoinQueue"
              @leave="handleLeaveQueue"
              :queue-count="queueCount"
              :is-queued="isQueued"
              :is-connected="isConnected"
            />
          </template>
          <template v-else>
            <DuelArena
              :duel="currentDuel"
              :time-remaining="timeRemaining"
              :current-price="currentPrice"
              :start-price="startPrice"
              @predict="handlePrediction"
            />
          </template>
        </div>

        <!-- Leaderboard Tab -->
        <Leaderboard v-if="activeTab === 'leaderboard'" :entries="leaderboard" />

        <!-- History Tab -->
        <UserStats
          v-if="activeTab === 'history'"
          :stats="userStats"
          :history="recentDuels"
          :is-connected="isConnected"
        />
      </div>
    </main>

    <!-- Footer -->
    <footer class="border-t border-border-main py-6 mt-auto">
      <div class="container-app">
        <div class="flex flex-col sm:flex-row items-center justify-between gap-4 text-center sm:text-left">
          <div>
            <p class="text-text-muted text-sm">
              SpeedBet Arena - Built on <span class="text-primary font-medium">Linera</span> Conway Testnet
            </p>
          </div>
          <div class="flex items-center gap-4 text-sm">
            <a
              href="https://linera.dev"
              target="_blank"
              rel="noopener noreferrer"
              class="text-text-muted hover:text-primary transition-colors duration-fast"
            >
              Linera
            </a>
            <span class="text-border-main">|</span>
            <a
              href="https://linera.dev/developers/getting_started/hello_linera.html"
              target="_blank"
              rel="noopener noreferrer"
              class="text-text-muted hover:text-primary transition-colors duration-fast"
            >
              Docs
            </a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, markRaw } from 'vue';
import { useLinera } from './composables/useLinera';
import { useDuel } from './composables/useDuel';
import { useToast } from './composables/useToast';

// Components
import LandingHero from './components/LandingHero.vue';
import ToastContainer from './components/ToastContainer.vue';
import ChainInfo from './components/ChainInfo.vue';
import WalletConnect from './components/WalletConnect.vue';
import PriceDisplay from './components/PriceDisplay.vue';
import MatchmakingQueue from './components/MatchmakingQueue.vue';
import DuelArena from './components/DuelArena.vue';
import Leaderboard from './components/Leaderboard.vue';
import UserStats from './components/UserStats.vue';

// Landing page state
const showLanding = ref(true);

function enterArena() {
  showLanding.value = false;
}

function goToLanding() {
  showLanding.value = true;
  activeTab.value = 'arena';
}

// Tab Icons (inline SVG components)
const SwordsIcon = markRaw({
  template: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14.5 17.5L3 6V3h3l11.5 11.5"/><path d="M13 19l6-6"/><path d="M16 16l4 4"/><path d="M19 21l2-2"/><path d="M14.5 6.5L18 3h3v3l-3.5 3.5"/><path d="M5 14l4 4"/><path d="M7 17l-3 3"/></svg>`
});

const TrophyIcon = markRaw({
  template: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 9H4.5a2.5 2.5 0 0 1 0-5H6"/><path d="M18 9h1.5a2.5 2.5 0 0 0 0-5H18"/><path d="M4 22h16"/><path d="M10 14.66V17c0 .55-.47.98-.97 1.21C7.85 18.75 7 20.24 7 22"/><path d="M14 14.66V17c0 .55.47.98.97 1.21C16.15 18.75 17 20.24 17 22"/><path d="M18 2H6v7a6 6 0 0 0 12 0V2Z"/></svg>`
});

const ClockIcon = markRaw({
  template: `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>`
});

// Composables
const {
  chainId,
  appId,
  isConnected,
  wallet,
  connect,
  disconnect
} = useLinera();

const {
  currentDuel,
  inDuel,
  queueCount,
  isQueued,
  timeRemaining,
  leaderboard,
  recentDuels,
  userStats,
  currentPrice,
  startPrice,
  fetchQueueCount,
  joinQueue,
  leaveQueue,
  submitPrediction,
  fetchLeaderboard,
  setupNotifications,
} = useDuel();

const toast = useToast();

// Tabs
const tabs = [
  { id: 'arena', label: 'Arena', icon: SwordsIcon },
  { id: 'leaderboard', label: 'Leaderboard', icon: TrophyIcon },
  { id: 'history', label: 'History', icon: ClockIcon },
];
const activeTab = ref('arena');

// Prices (fetched from external API)
const btcPrice = ref(0);
const ethPrice = ref(0);

// Interval references for cleanup
let priceInterval = null;
let queueInterval = null;

// Fetch prices from external API with fallback
async function fetchPrices() {
  try {
    // Try direct CoinGecko API first
    const response = await fetch(
      'https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd'
    );
    if (!response.ok) throw new Error('CoinGecko API error');
    const data = await response.json();

    btcPrice.value = Math.round(data.bitcoin.usd * 1_000_000); // micro-USD
    ethPrice.value = Math.round(data.ethereum.usd * 1_000_000);
  } catch (err) {
    // Fallback: Try CORS proxy
    try {
      const proxyResponse = await fetch(
        'https://corsproxy.io/?' + encodeURIComponent('https://api.coingecko.com/api/v3/simple/price?ids=bitcoin,ethereum&vs_currencies=usd')
      );
      if (proxyResponse.ok) {
        const data = await proxyResponse.json();
        btcPrice.value = Math.round(data.bitcoin.usd * 1_000_000);
        ethPrice.value = Math.round(data.ethereum.usd * 1_000_000);
      } else {
        throw new Error('Proxy failed');
      }
    } catch (proxyErr) {
      // Final fallback: Use realistic mock prices if both fail
      // Only set if prices are still 0 (initial state)
      if (btcPrice.value === 0 && ethPrice.value === 0) {
        // Use approximate current market prices as fallback
        btcPrice.value = 89800_000_000; // ~$89,800 in micro-USD
        ethPrice.value = 2960_000_000;  // ~$2,960 in micro-USD
        console.log('Using fallback prices due to API unavailability');
      }
    }
  }

  // Update current price based on active duel
  if (currentDuel.value) {
    currentPrice.value = currentDuel.value.asset === 'BTC'
      ? btcPrice.value
      : ethPrice.value;
  }
}

// Handlers
async function handleJoinQueue(asset, betAmount) {
  try {
    await joinQueue(asset, betAmount);
    toast.success('Joined Queue', `Looking for a ${asset} opponent...`);
  } catch (err) {
    toast.error('Failed to Join', err.message);
  }
}

async function handleLeaveQueue() {
  try {
    await leaveQueue();
    toast.info('Left Queue', 'You have left the matchmaking queue');
  } catch (err) {
    toast.error('Failed to Leave', err.message);
  }
}

async function handlePrediction(direction) {
  try {
    await submitPrediction(direction);
    toast.success('Prediction Submitted', `You predicted ${direction.toUpperCase()}`);
  } catch (err) {
    toast.error('Prediction Failed', err.message);
  }
}

// Initialize
onMounted(async () => {
  try {
    // Connect to Linera
    await connect();

    // Setup notifications
    setupNotifications();

    // Fetch initial data
    fetchQueueCount();
    fetchLeaderboard();
    fetchPrices();

    // Poll prices every 5 seconds
    priceInterval = setInterval(fetchPrices, 5000);

    // Poll queue every 3 seconds
    queueInterval = setInterval(fetchQueueCount, 3000);

  } catch (err) {
    console.error('Initialization error:', err);
  }
});

// Clean up intervals on unmount
onUnmounted(() => {
  if (priceInterval) {
    clearInterval(priceInterval);
    priceInterval = null;
  }
  if (queueInterval) {
    clearInterval(queueInterval);
    queueInterval = null;
  }
});
</script>

<style>
/* Hide scrollbar but keep functionality */
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
</style>
