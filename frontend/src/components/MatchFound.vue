<template>
  <Teleport to="body">
    <Transition name="match-overlay">
      <div
        v-if="visible"
        class="fixed inset-0 z-50 bg-bg-main/95 backdrop-blur-sm flex items-center justify-center"
      >
        <div class="text-center max-w-md mx-auto px-4">
          <!-- VS Badge -->
          <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-primary/20 border-2 border-primary flex items-center justify-center">
            <span class="text-3xl font-bold text-primary">VS</span>
          </div>

          <!-- Match Found Text -->
          <h2 class="text-3xl md:text-4xl font-bold text-text-main mb-6">
            OPPONENT FOUND!
          </h2>

          <!-- Players -->
          <div class="flex items-center justify-center gap-6 mb-6">
            <!-- Player 1 -->
            <div class="text-center">
              <div class="w-14 h-14 mx-auto mb-2 rounded-full bg-primary/20 flex items-center justify-center">
                <svg class="w-7 h-7 text-primary" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </div>
              <div class="text-primary font-medium text-sm">You</div>
              <div class="text-text-dim text-xs font-mono">{{ truncateAddress(player1) }}</div>
            </div>

            <div class="text-xl font-bold text-text-dim">VS</div>

            <!-- Player 2 -->
            <div class="text-center">
              <div class="w-14 h-14 mx-auto mb-2 rounded-full bg-danger/20 flex items-center justify-center">
                <svg class="w-7 h-7 text-danger" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
                </svg>
              </div>
              <div class="text-danger font-medium text-sm">Opponent</div>
              <div class="text-text-dim text-xs font-mono">{{ truncateAddress(player2) }}</div>
            </div>
          </div>

          <!-- Match Info -->
          <div class="flex items-center justify-center gap-4 mb-8">
            <div class="px-4 py-2 bg-bg-surface rounded-lg border border-border-main">
              <span class="text-text-dim text-sm">Asset: </span>
              <span class="text-warning font-bold">{{ asset }}</span>
            </div>
            <div class="px-4 py-2 bg-bg-surface rounded-lg border border-border-main">
              <span class="text-text-dim text-sm">Bet: </span>
              <span class="text-primary font-bold font-mono">{{ betAmount }} tokens</span>
            </div>
          </div>

          <!-- Countdown -->
          <div>
            <div class="text-text-muted text-sm mb-2">Starting in</div>
            <div class="text-5xl font-bold font-mono text-primary">
              {{ countdown }}
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { ref, watch, onUnmounted } from 'vue';

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  player1: {
    type: String,
    default: ''
  },
  player2: {
    type: String,
    default: ''
  },
  asset: {
    type: String,
    default: 'BTC'
  },
  betAmount: {
    type: Number,
    default: 0
  },
  duration: {
    type: Number,
    default: 3000
  }
});

const emit = defineEmits(['start']);

const countdown = ref(3);
let countdownInterval = null;

watch(() => props.visible, (isVisible) => {
  if (isVisible) {
    countdown.value = 3;
    countdownInterval = setInterval(() => {
      countdown.value--;
      if (countdown.value <= 0) {
        clearInterval(countdownInterval);
        emit('start');
      }
    }, 1000);
  } else {
    if (countdownInterval) {
      clearInterval(countdownInterval);
    }
  }
});

onUnmounted(() => {
  if (countdownInterval) {
    clearInterval(countdownInterval);
  }
});

function truncateAddress(addr) {
  if (!addr) return '...';
  return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}
</script>

<style scoped>
.match-overlay-enter-active,
.match-overlay-leave-active {
  transition: opacity 0.2s ease-out;
}

.match-overlay-enter-from,
.match-overlay-leave-to {
  opacity: 0;
}
</style>
