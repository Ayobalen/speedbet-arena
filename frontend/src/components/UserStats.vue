<template>
  <div class="space-y-6">
    <!-- Stats Overview -->
    <div class="bg-bg-surface border border-border-main rounded-2xl p-6">
      <h2 class="text-xl font-bold text-text-main mb-6">Your Statistics</h2>

      <div v-if="stats" class="grid grid-cols-2 md:grid-cols-4 gap-4">
        <!-- Wins -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-success">{{ stats.wins || 0 }}</div>
          <div class="text-text-muted text-sm mt-1">Wins</div>
        </div>
        <!-- Losses -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-danger">{{ stats.losses || 0 }}</div>
          <div class="text-text-muted text-sm mt-1">Losses</div>
        </div>
        <!-- Win Rate -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-primary">{{ stats.win_rate || 0 }}%</div>
          <div class="text-text-muted text-sm mt-1">Win Rate</div>
        </div>
        <!-- Current Streak -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-warning">
            <span v-if="stats.win_streak > 0">{{ stats.win_streak }}</span>
            <span v-else>0</span>
          </div>
          <div class="text-text-muted text-sm mt-1">Current Streak</div>
        </div>
        <!-- Best Streak -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-warning">{{ stats.best_streak || 0 }}</div>
          <div class="text-text-muted text-sm mt-1">Best Streak</div>
        </div>
        <!-- Total Won -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-success font-mono">{{ formatAmount(stats.total_won) }}</div>
          <div class="text-text-muted text-sm mt-1">Total Won</div>
        </div>
        <!-- Total Wagered -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold text-info font-mono">{{ formatAmount(stats.total_wagered) }}</div>
          <div class="text-text-muted text-sm mt-1">Total Wagered</div>
        </div>
        <!-- ROI -->
        <div class="bg-bg-main rounded-xl p-4 text-center">
          <div class="text-3xl font-bold" :class="roi >= 0 ? 'text-success' : 'text-danger'">
            {{ roi >= 0 ? '+' : '' }}{{ roi }}%
          </div>
          <div class="text-text-muted text-sm mt-1">ROI</div>
        </div>
      </div>

      <div v-else class="text-center py-8">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-bg-elevated flex items-center justify-center">
          <svg class="w-8 h-8 text-text-dim" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
          </svg>
        </div>
        <p v-if="isConnected" class="text-text-muted">No stats yet</p>
        <p v-if="isConnected" class="text-text-dim text-sm mt-1">Play your first duel to see stats!</p>
        <p v-else class="text-text-muted">Connect wallet to see your stats</p>
      </div>
    </div>

    <!-- Recent Duels -->
    <div class="bg-bg-surface border border-border-main rounded-2xl p-6">
      <h2 class="text-xl font-bold text-text-main mb-6">Recent Duels</h2>

      <div v-if="history && history.length > 0" class="space-y-3">
        <div
          v-for="duel in history"
          :key="duel.id"
          class="flex items-center justify-between p-4 bg-bg-main rounded-xl"
        >
          <div class="flex items-center gap-4">
            <span
              :class="[
                'px-3 py-1.5 rounded-lg text-xs font-bold',
                duel.winner === 'You' ? 'bg-success/20 text-success' : 'bg-danger/20 text-danger'
              ]"
            >
              {{ duel.winner === 'You' ? 'WIN' : 'LOSS' }}
            </span>
            <div>
              <span class="text-text-main">vs</span>
              <span class="text-text-muted font-mono ml-2">{{ truncateAddress(duel.opponent) }}</span>
            </div>
          </div>
          <div class="text-right">
            <div class="font-mono text-text-main">{{ duel.asset }}</div>
            <div class="text-sm text-text-muted">
              <span :class="duel.winner === 'You' ? 'text-success' : 'text-danger'">
                {{ duel.winner === 'You' ? '+' : '-' }}{{ formatAmount(duel.bet_amount) }}
              </span>
              tokens
            </div>
          </div>
        </div>
      </div>

      <div v-else class="text-center py-8">
        <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-bg-elevated flex items-center justify-center">
          <svg class="w-8 h-8 text-text-dim" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <p class="text-text-muted">No duel history yet</p>
        <p class="text-text-dim text-sm mt-1">Start your first duel in the Arena!</p>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';

const props = defineProps({
  stats: {
    type: Object,
    default: null
  },
  history: {
    type: Array,
    default: () => []
  },
  isConnected: {
    type: Boolean,
    default: false
  }
});

const roi = computed(() => {
  if (!props.stats?.total_wagered || props.stats.total_wagered === 0) return 0;
  const profit = (props.stats.total_won || 0) - props.stats.total_wagered;
  return Math.round((profit / props.stats.total_wagered) * 100);
});

function truncateAddress(addr) {
  if (!addr) return '';
  if (addr.length <= 12) return addr;
  return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}

function formatAmount(microUnits) {
  if (!microUnits) return '0';
  return (microUnits / 1_000_000).toFixed(2);
}
</script>
