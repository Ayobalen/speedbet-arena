<template>
  <Teleport to="body">
    <div class="fixed top-4 right-4 z-50 flex flex-col gap-3 max-w-sm">
      <TransitionGroup name="toast-list">
        <div
          v-for="toast in toasts"
          :key="toast.id"
          class="p-4 rounded-xl shadow-elevated border animate-slide-down"
          :class="getVariantClasses(toast.type)"
        >
          <div class="flex items-start gap-3">
            <!-- Icon -->
            <div
              class="w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0"
              :class="getIconBgClass(toast.type)"
            >
              <svg v-if="toast.type === 'success'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              <svg v-else-if="toast.type === 'error'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
              <svg v-else-if="toast.type === 'warning'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
              </svg>
              <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
            </div>

            <!-- Content -->
            <div class="flex-1 min-w-0">
              <p class="font-semibold text-text-main">{{ toast.title }}</p>
              <p v-if="toast.message" class="text-sm text-text-muted mt-1">{{ toast.message }}</p>
            </div>

            <!-- Close Button -->
            <button
              @click="removeToast(toast.id)"
              class="flex-shrink-0 p-1 rounded-lg text-text-muted hover:text-text-main hover:bg-bg-hover transition-colors duration-fast"
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </button>
          </div>

          <!-- Progress Bar -->
          <div v-if="toast.duration > 0" class="absolute bottom-0 left-0 right-0 h-1 rounded-b-xl overflow-hidden">
            <div
              class="h-full toast-progress"
              :class="getProgressClass(toast.type)"
              :style="{ animationDuration: `${toast.duration}ms` }"
            ></div>
          </div>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup>
import { useToast } from '../composables/useToast';

const { toasts, removeToast } = useToast();

function getVariantClasses(type) {
  const classes = {
    success: 'bg-bg-surface border-success/30',
    error: 'bg-bg-surface border-danger/30',
    warning: 'bg-bg-surface border-warning/30',
    info: 'bg-bg-surface border-primary/30'
  };
  return classes[type] || classes.info;
}

function getIconBgClass(type) {
  const classes = {
    success: 'bg-success/20 text-success',
    error: 'bg-danger/20 text-danger',
    warning: 'bg-warning/20 text-warning',
    info: 'bg-primary/20 text-primary'
  };
  return classes[type] || classes.info;
}

function getProgressClass(type) {
  const classes = {
    success: 'bg-success',
    error: 'bg-danger',
    warning: 'bg-warning',
    info: 'bg-primary'
  };
  return classes[type] || classes.info;
}
</script>

<style scoped>
.toast-list-enter-active,
.toast-list-leave-active {
  transition: all 0.2s ease-out;
}

.toast-list-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-list-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

.toast-list-move {
  transition: transform 0.2s ease-out;
}

.toast-progress {
  animation: shrink linear forwards;
}

@keyframes shrink {
  from {
    width: 100%;
  }
  to {
    width: 0%;
  }
}
</style>
