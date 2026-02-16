<template>
  <div class="bg-bg-surface border border-border-main rounded-2xl p-6 md:p-8 max-w-2xl mx-auto">
    <!-- Header -->
    <div class="text-center mb-8">
      <div class="inline-flex items-center gap-2 px-4 py-2 bg-danger/10 text-danger rounded-full text-sm font-medium mb-4">
        <span class="w-2 h-2 bg-danger rounded-full"></span>
        LIVE DUEL
      </div>
      <h2 class="text-2xl md:text-3xl font-bold text-text-main">DUEL IN PROGRESS</h2>
    </div>

    <!-- Timer -->
    <div class="text-center mb-8">
      <div class="relative inline-flex items-center justify-center">
        <!-- Circular Progress Ring -->
        <svg class="w-32 h-32 md:w-40 md:h-40 -rotate-90" viewBox="0 0 120 120">
          <!-- Background Ring -->
          <circle
            cx="60"
            cy="60"
            r="54"
            fill="none"
            stroke="currentColor"
            stroke-width="8"
            class="text-bg-elevated"
          />
          <!-- Progress Ring -->
          <circle
            cx="60"
            cy="60"
            r="54"
            fill="none"
            stroke="currentColor"
            stroke-width="8"
            :stroke-dasharray="339.292"
            :stroke-dashoffset="timerProgress"
            stroke-linecap="round"
            :class="[
              'transition-all duration-1000',
              timeRemaining <= 10 ? 'text-danger' : 'text-primary'
            ]"
          />
        </svg>
        <!-- Timer Text -->
        <div class="absolute inset-0 flex flex-col items-center justify-center">
          <span
            class="text-4xl md:text-5xl font-mono font-bold"
            :class="timeRemaining <= 10 ? 'text-danger' : 'text-text-main'"
          >
            {{ formatTime(timeRemaining) }}
          </span>
          <span class="text-text-muted text-sm mt-1">remaining</span>
        </div>
      </div>
    </div>

    <!-- Price Display -->
    <div class="flex justify-center mb-8">
      <div class="bg-bg-elevated rounded-xl px-6 py-4 text-center border border-border-main">
        <div class="text-text-muted text-sm mb-1">{{ duel?.asset }} Price</div>
        <div class="text-3xl md:text-4xl font-bold font-mono text-warning">
          ${{ formatPrice(currentPrice) }}
        </div>
        <div
          v-if="startPrice"
          class="text-sm mt-2 font-medium"
          :class="priceChangeClass"
        >
          {{ priceChangeText }}
        </div>
      </div>
    </div>

    <!-- Players -->
    <div class="flex items-stretch gap-4 mb-8">
      <!-- Player 1 (You) -->
      <div class="flex-1 bg-bg-elevated rounded-xl p-4 border border-border-main">
        <div class="text-center">
          <div class="w-12 h-12 mx-auto mb-3 rounded-full bg-primary/20 flex items-center justify-center">
            <svg class="w-6 h-6 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
          </div>
          <div class="font-semibold text-primary mb-1">You</div>
          <div class="text-xs text-text-muted font-mono">{{ truncateAddress(duel?.player1) }}</div>
          <div v-if="prediction" class="mt-3">
            <span
              class="inline-flex items-center gap-1 px-4 py-2 rounded-lg font-bold text-sm"
              :class="prediction === 'Up' ? 'bg-success/20 text-success' : 'bg-danger/20 text-danger'"
            >
              <svg v-if="prediction === 'Up'" class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
              </svg>
              <svg v-else class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
              </svg>
              {{ prediction }}
            </span>
          </div>
        </div>
      </div>

      <!-- VS Divider -->
      <div class="flex items-center">
        <div class="w-14 h-14 rounded-full bg-bg-main border-2 border-border-main flex items-center justify-center">
          <span class="text-lg font-bold text-text-muted">VS</span>
        </div>
      </div>

      <!-- Player 2 (Opponent) -->
      <div class="flex-1 bg-bg-elevated rounded-xl p-4 border border-border-main">
        <div class="text-center">
          <div class="w-12 h-12 mx-auto mb-3 rounded-full bg-danger/20 flex items-center justify-center">
            <svg class="w-6 h-6 text-danger" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
          </div>
          <div class="font-semibold text-danger mb-1">Opponent</div>
          <div class="text-xs text-text-muted font-mono">{{ truncateAddress(duel?.player2) }}</div>
          <div v-if="opponentPredicted" class="mt-3">
            <span class="inline-flex items-center gap-1 px-4 py-2 rounded-lg bg-bg-hover text-text-muted font-bold text-sm">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
              </svg>
              LOCKED
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Prediction Buttons -->
    <div v-if="!prediction && !duelEnded" class="flex gap-4 mb-8">
      <button
        @click="handlePredict('Up')"
        :disabled="submitting"
        class="flex-1 px-8 py-5 rounded-xl font-bold text-lg text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed bg-success hover:bg-success-hover"
      >
        <span class="flex items-center justify-center gap-2">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
          </svg>
          {{ submitting ? 'SUBMITTING...' : 'UP' }}
        </span>
      </button>
      <button
        @click="handlePredict('Down')"
        :disabled="submitting"
        class="flex-1 px-8 py-5 rounded-xl font-bold text-lg text-white transition-colors disabled:opacity-50 disabled:cursor-not-allowed bg-danger hover:bg-danger-hover"
      >
        <span class="flex items-center justify-center gap-2">
          <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
          </svg>
          {{ submitting ? 'SUBMITTING...' : 'DOWN' }}
        </span>
      </button>
    </div>

    <!-- Waiting for prediction state -->
    <div v-else-if="prediction && !duelEnded" class="text-center mb-8 py-4">
      <div class="inline-flex items-center gap-2 text-text-muted">
        <svg class="w-5 h-5 animate-spin" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        <span>Waiting for duel to resolve...</span>
      </div>
    </div>

    <!-- Pot Info -->
    <div class="text-center py-4 border-t border-border-main">
      <span class="text-text-muted">Total Pot: </span>
      <span class="text-2xl font-bold font-mono text-warning">
        {{ formatAmount(totalPot) }} tokens
      </span>
    </div>

    <!-- Result -->
    <div v-if="duelEnded && duel?.winner" class="mt-6">
      <div
        class="text-center py-8 rounded-xl"
        :class="isWinner ? 'bg-success/10 border border-success/30' : 'bg-danger/10 border border-danger/30'"
      >
        <div
          class="text-4xl md:text-5xl font-bold mb-4"
          :class="isWinner ? 'text-success' : 'text-danger'"
        >
          {{ isWinner ? 'YOU WON!' : 'YOU LOST' }}
        </div>
        <div v-if="isWinner" class="text-text-muted">
          Prize: <span class="text-success font-bold">+{{ formatAmount(totalPot) }} tokens</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, ref } from 'vue';

const props = defineProps({
  duel: Object,
  timeRemaining: Number,
  currentPrice: Number,
  startPrice: Number,
});

const emit = defineEmits(['predict']);

const prediction = ref(null);
const submitting = ref(false);

// Calculate timer progress for circular ring (0 to 339.292 for full circle)
const timerProgress = computed(() => {
  const maxTime = 60; // 60 second duels
  const progress = (props.timeRemaining || 0) / maxTime;
  return 339.292 * (1 - progress);
});

async function handlePredict(direction) {
  if (submitting.value || prediction.value) return;

  submitting.value = true;
  prediction.value = direction;
  emit('predict', direction);
}

const opponentPredicted = computed(() => props.duel?.p2Prediction != null);
const duelEnded = computed(() => {
  const status = props.duel?.status;
  return status === 'Resolved' || status === 'Cancelled';
});
const isWinner = computed(() => {
  return props.duel?.winner === props.duel?.player1;
});

const totalPot = computed(() => {
  if (!props.duel?.betAmount) return '0';
  const amount = parseFloat(props.duel.betAmount);
  return (amount * 2).toString();
});

const priceChange = computed(() => {
  if (!props.currentPrice || !props.startPrice) return 0;
  return ((props.currentPrice - props.startPrice) / props.startPrice) * 100;
});

const priceChangeClass = computed(() => {
  if (priceChange.value > 0) return 'text-success';
  if (priceChange.value < 0) return 'text-danger';
  return 'text-text-muted';
});

const priceChangeText = computed(() => {
  const change = priceChange.value;
  if (change > 0) {
    return `+${change.toFixed(2)}%`;
  } else if (change < 0) {
    return `${change.toFixed(2)}%`;
  }
  return '0.00%';
});

function formatTime(seconds) {
  if (typeof seconds !== 'number' || isNaN(seconds) || seconds < 0) {
    return '00:00';
  }
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${String(mins).padStart(2, '0')}:${String(secs).padStart(2, '0')}`;
}

function formatPrice(microUsd) {
  if (!microUsd) return '---';
  return (microUsd / 1_000_000).toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

function formatAmount(amount) {
  if (!amount) return '0';
  return parseFloat(amount).toFixed(2);
}

function truncateAddress(addr) {
  if (!addr) return '';
  return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}
</script>
