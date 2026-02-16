<template>
  <div class="bg-bg-surface border border-border-main rounded-2xl p-6 max-w-md mx-auto">
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-xl font-bold text-text-main">Matchmaking Queue</h2>
      <div class="flex items-center gap-2">
        <span class="text-text-muted text-sm">Players waiting:</span>
        <span class="px-3 py-1 bg-primary/20 text-primary rounded-full font-bold text-lg">
          {{ queueCount }}
        </span>
      </div>
    </div>

    <template v-if="!isQueued">
      <!-- Join Queue Form -->
      <div class="space-y-6">
        <!-- Asset Selection -->
        <div>
          <label class="block text-text-muted text-sm font-medium mb-3">Select Asset</label>
          <div class="grid grid-cols-2 gap-3">
            <button
              @click="selectedAsset = 'BTC'"
              :class="[
                'relative py-4 px-4 rounded-xl font-semibold transition-all duration-fast border-2',
                selectedAsset === 'BTC'
                  ? 'bg-primary/10 border-primary text-text-main'
                  : 'bg-bg-main border-border-main text-text-muted hover:border-border-hover hover:bg-bg-elevated'
              ]"
            >
              <span class="flex items-center justify-center gap-2">
                <span class="text-warning text-xl">₿</span>
                BTC
              </span>
              <span v-if="selectedAsset === 'BTC'" class="absolute top-2 right-2 w-2 h-2 bg-primary rounded-full"></span>
            </button>
            <button
              @click="selectedAsset = 'ETH'"
              :class="[
                'relative py-4 px-4 rounded-xl font-semibold transition-all duration-fast border-2',
                selectedAsset === 'ETH'
                  ? 'bg-primary/10 border-primary text-text-main'
                  : 'bg-bg-main border-border-main text-text-muted hover:border-border-hover hover:bg-bg-elevated'
              ]"
            >
              <span class="flex items-center justify-center gap-2">
                <span class="text-info text-xl">Ξ</span>
                ETH
              </span>
              <span v-if="selectedAsset === 'ETH'" class="absolute top-2 right-2 w-2 h-2 bg-primary rounded-full"></span>
            </button>
          </div>
        </div>

        <!-- Bet Amount -->
        <div>
          <label class="block text-text-muted text-sm font-medium mb-3">Bet Amount</label>
          <div class="relative">
            <input
              v-model="betAmount"
              type="number"
              min="1"
              max="100"
              placeholder="Enter amount"
              class="w-full px-4 py-4 bg-bg-main border border-border-main rounded-xl text-text-main text-lg font-mono focus:border-primary focus:ring-2 focus:ring-primary/20 focus:outline-none transition-all duration-fast"
            />
            <span class="absolute right-4 top-1/2 -translate-y-1/2 text-text-muted text-sm">tokens</span>
          </div>
          <p class="text-text-dim text-xs mt-2">Min: 1 token • Max: 100 tokens</p>
        </div>

        <!-- Quick Bet Amounts -->
        <div class="flex gap-2">
          <button
            v-for="amount in [5, 10, 25, 50]"
            :key="amount"
            @click="betAmount = amount"
            :class="[
              'flex-1 py-2 rounded-lg text-sm font-medium transition-all duration-fast',
              betAmount === amount
                ? 'bg-primary text-white'
                : 'bg-bg-main text-text-muted hover:bg-bg-elevated border border-border-main'
            ]"
          >
            {{ amount }}
          </button>
        </div>

        <!-- Join Button -->
        <button
          @click="handleJoin"
          :disabled="!isConnected || !selectedAsset || !betAmount || isJoining"
          class="w-full py-4 bg-primary hover:bg-primary-hover text-white disabled:bg-bg-elevated disabled:text-text-dim disabled:cursor-not-allowed rounded-xl font-bold text-lg transition-colors"
        >
          <span v-if="isJoining" class="flex items-center justify-center gap-3">
            <svg class="animate-spin h-5 w-5" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <span>Joining...</span>
          </span>
          <span v-else-if="!isConnected" class="flex items-center justify-center gap-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
            </svg>
            Connect Wallet First
          </span>
          <span v-else class="flex items-center justify-center gap-2">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            Join Queue
          </span>
        </button>
      </div>
    </template>

    <template v-else>
      <!-- In Queue View -->
      <div class="text-center py-8">
        <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-primary/10 border-2 border-primary/30 flex items-center justify-center">
          <svg class="w-8 h-8 text-primary animate-spin" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
          </svg>
        </div>
        <h3 class="text-xl font-bold text-text-main mb-2">Looking for opponent...</h3>
        <p class="text-text-muted mb-6">This usually takes less than a minute</p>
        <button
          @click="$emit('leave')"
          class="px-6 py-3 bg-danger/10 hover:bg-danger/20 text-danger rounded-xl font-semibold transition-all duration-fast border border-danger/30"
        >
          Leave Queue
        </button>
      </div>
    </template>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const props = defineProps({
  queueCount: {
    type: Number,
    default: 0
  },
  isQueued: {
    type: Boolean,
    default: false
  },
  isConnected: {
    type: Boolean,
    default: false
  }
});

const emit = defineEmits(['join', 'leave']);

const selectedAsset = ref('BTC');
const betAmount = ref(10);
const isJoining = ref(false);

async function handleJoin() {
  if (selectedAsset.value && betAmount.value) {
    isJoining.value = true;
    try {
      const betInMicro = betAmount.value * 1_000_000;
      emit('join', selectedAsset.value, betInMicro);
    } finally {
      setTimeout(() => {
        isJoining.value = false;
      }, 1000);
    }
  }
}
</script>
