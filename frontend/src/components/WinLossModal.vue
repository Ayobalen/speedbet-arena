<template>
  <Teleport to="body">
    <Transition name="result-modal">
      <div
        v-if="visible"
        class="fixed inset-0 z-50 bg-bg-main/90 backdrop-blur-sm flex items-center justify-center p-4"
        @click.self="$emit('close')"
      >
        <div
          class="relative bg-bg-surface border rounded-2xl p-8 max-w-md w-full text-center"
          :class="isWin ? 'border-success/30' : 'border-danger/30'"
        >
          <!-- Result Icon -->
          <div class="mb-6">
            <div
              class="w-20 h-20 mx-auto rounded-full flex items-center justify-center"
              :class="isWin ? 'bg-success/20' : 'bg-danger/20'"
            >
              <svg v-if="isWin" class="w-10 h-10 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <svg v-else class="w-10 h-10 text-danger" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>
          </div>

          <!-- Result Text -->
          <h2
            class="text-3xl font-bold mb-2"
            :class="isWin ? 'text-success' : 'text-danger'"
          >
            {{ isWin ? 'VICTORY!' : 'DEFEAT' }}
          </h2>

          <p class="text-text-muted mb-6">
            {{ isWin ? 'You predicted correctly!' : 'Better luck next time!' }}
          </p>

          <!-- Prize/Loss Amount -->
          <div
            class="px-6 py-4 rounded-xl mb-6"
            :class="isWin ? 'bg-success/10' : 'bg-danger/10'"
          >
            <div class="text-text-muted text-sm mb-1">
              {{ isWin ? 'You Won' : 'You Lost' }}
            </div>
            <div
              class="text-2xl font-bold font-mono"
              :class="isWin ? 'text-success' : 'text-danger'"
            >
              {{ isWin ? '+' : '-' }}{{ amount }} tokens
            </div>
          </div>

          <!-- Duel Summary -->
          <div class="grid grid-cols-2 gap-4 mb-6 text-sm">
            <div class="bg-bg-main rounded-lg p-3">
              <div class="text-text-dim">Your Pick</div>
              <div
                class="font-bold"
                :class="yourPrediction === 'Up' ? 'text-success' : 'text-danger'"
              >
                {{ yourPrediction }}
              </div>
            </div>
            <div class="bg-bg-main rounded-lg p-3">
              <div class="text-text-dim">Actual</div>
              <div
                class="font-bold"
                :class="actualDirection === 'Up' ? 'text-success' : 'text-danger'"
              >
                {{ actualDirection }}
              </div>
            </div>
          </div>

          <!-- Actions -->
          <div class="flex gap-3">
            <button
              @click="$emit('close')"
              class="flex-1 py-3 bg-bg-main hover:bg-bg-elevated text-text-main rounded-xl font-semibold transition-colors border border-border-main"
            >
              View Stats
            </button>
            <button
              @click="$emit('playAgain')"
              class="flex-1 py-3 bg-primary hover:bg-primary-hover text-white rounded-xl font-semibold transition-colors"
            >
              Play Again
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  isWin: {
    type: Boolean,
    default: false
  },
  amount: {
    type: [Number, String],
    default: 0
  },
  yourPrediction: {
    type: String,
    default: 'Up'
  },
  actualDirection: {
    type: String,
    default: 'Up'
  }
});

defineEmits(['close', 'playAgain']);
</script>

<style scoped>
.result-modal-enter-active,
.result-modal-leave-active {
  transition: opacity 0.2s ease-out;
}

.result-modal-enter-from,
.result-modal-leave-to {
  opacity: 0;
}
</style>
