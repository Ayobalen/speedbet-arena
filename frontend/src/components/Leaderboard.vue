<template>
  <div class="bg-bg-surface border border-border-main rounded-2xl p-6">
    <div class="flex items-center justify-between mb-6">
      <h2 class="text-xl font-bold text-text-main">Leaderboard</h2>
      <span class="text-text-muted text-sm">Top Players</span>
    </div>

    <div v-if="entries && entries.length > 0" class="space-y-3">
      <div
        v-for="(entry, index) in paginatedEntries"
        :key="entry.player"
        :class="[
          'flex items-center justify-between p-4 rounded-xl transition-all duration-fast',
          getActualIndex(index) < 3 ? 'bg-bg-elevated border border-border-main' : 'bg-bg-main'
        ]"
      >
        <div class="flex items-center gap-4">
          <!-- Rank -->
          <div
            :class="[
              'w-10 h-10 rounded-full flex items-center justify-center font-bold text-lg',
              getRankBgClass(getActualIndex(index))
            ]"
          >
            {{ getMedalOrRank(getActualIndex(index)) }}
          </div>
          <!-- Player Info -->
          <div>
            <span class="font-mono text-text-main text-sm">
              {{ truncateAddress(entry.player) }}
            </span>
            <div class="text-xs text-text-muted mt-0.5">
              {{ entry.stats?.wins || 0 }}W - {{ entry.stats?.losses || 0 }}L
            </div>
          </div>
        </div>

        <!-- Stats -->
        <div class="text-right">
          <div class="text-lg font-bold text-text-main">
            {{ entry.stats?.win_rate || 0 }}%
          </div>
          <div class="text-sm text-warning font-medium font-mono">
            +{{ formatAmount(entry.stats?.total_won) }}
          </div>
        </div>
      </div>
    </div>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex justify-center items-center gap-4 mt-6 pt-4 border-t border-border-main">
      <button
        @click="prevPage"
        :disabled="page === 1"
        class="px-4 py-2 bg-bg-main border border-border-main rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-bg-elevated hover:border-border-hover transition-all duration-fast text-sm font-medium"
      >
        Previous
      </button>
      <span class="text-text-muted text-sm">
        Page {{ page }} of {{ totalPages }}
      </span>
      <button
        @click="nextPage"
        :disabled="page >= totalPages"
        class="px-4 py-2 bg-bg-main border border-border-main rounded-lg disabled:opacity-50 disabled:cursor-not-allowed hover:bg-bg-elevated hover:border-border-hover transition-all duration-fast text-sm font-medium"
      >
        Next
      </button>
    </div>

    <!-- Empty State -->
    <div v-else-if="!entries || entries.length === 0" class="text-center py-12">
      <div class="w-16 h-16 mx-auto mb-4 rounded-full bg-bg-elevated flex items-center justify-center">
        <svg class="w-8 h-8 text-text-dim" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
      </div>
      <p class="text-text-muted">No players on leaderboard yet</p>
      <p class="text-text-dim text-sm mt-1">Be the first to compete!</p>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';

const props = defineProps({
  entries: {
    type: Array,
    default: () => []
  }
});

const page = ref(1);
const itemsPerPage = 10;

const totalPages = computed(() => {
  return Math.ceil((props.entries?.length || 0) / itemsPerPage);
});

const paginatedEntries = computed(() => {
  if (!props.entries) return [];
  const start = (page.value - 1) * itemsPerPage;
  const end = start + itemsPerPage;
  return props.entries.slice(start, end);
});

function getActualIndex(paginatedIndex) {
  return (page.value - 1) * itemsPerPage + paginatedIndex;
}

function prevPage() {
  if (page.value > 1) page.value--;
}

function nextPage() {
  if (page.value < totalPages.value) page.value++;
}

function truncateAddress(addr) {
  if (!addr) return '';
  if (addr.length <= 12) return addr;
  return `${addr.slice(0, 6)}...${addr.slice(-4)}`;
}

function formatAmount(amount) {
  if (!amount) return '0';
  const tokens = parseFloat(amount) / 1_000_000;
  return tokens.toFixed(2);
}

function getRankBgClass(index) {
  if (index === 0) return 'bg-warning/20 text-warning';
  if (index === 1) return 'bg-text-muted/20 text-text-secondary';
  if (index === 2) return 'bg-warning/10 text-warning/80';
  return 'bg-bg-hover text-text-muted';
}

function getMedalOrRank(index) {
  if (index === 0) return '1st';
  if (index === 1) return '2nd';
  if (index === 2) return '3rd';
  return index + 1;
}
</script>
