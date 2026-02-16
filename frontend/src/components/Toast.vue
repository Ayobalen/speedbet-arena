<template>
  <Teleport to="body">
    <Transition name="toast">
      <div
        v-if="visible"
        class="fixed top-4 right-4 z-50 max-w-sm p-4 rounded-xl shadow-elevated border animate-slide-down"
        :class="variantClasses"
      >
        <div class="flex items-start gap-3">
          <!-- Icon -->
          <div
            class="w-8 h-8 rounded-full flex items-center justify-center flex-shrink-0"
            :class="iconBgClass"
          >
            <svg v-if="type === 'success'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <svg v-else-if="type === 'error'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
            <svg v-else-if="type === 'warning'" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>

          <!-- Content -->
          <div class="flex-1 min-w-0">
            <p class="font-semibold text-text-main">{{ title }}</p>
            <p v-if="message" class="text-sm text-text-muted mt-1">{{ message }}</p>
          </div>

          <!-- Close Button -->
          <button
            @click="close"
            class="flex-shrink-0 p-1 rounded-lg text-text-muted hover:text-text-main hover:bg-bg-hover transition-colors duration-fast"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        <!-- Progress Bar -->
        <div v-if="autoClose && duration > 0" class="absolute bottom-0 left-0 right-0 h-1 rounded-b-xl overflow-hidden">
          <div
            class="h-full transition-all ease-linear"
            :class="progressClass"
            :style="{ width: `${progress}%` }"
          ></div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue';

const props = defineProps({
  type: {
    type: String,
    default: 'info',
    validator: (value) => ['success', 'error', 'warning', 'info'].includes(value)
  },
  title: {
    type: String,
    default: ''
  },
  message: {
    type: String,
    default: ''
  },
  duration: {
    type: Number,
    default: 5000
  },
  autoClose: {
    type: Boolean,
    default: true
  }
});

const emit = defineEmits(['close']);

const visible = ref(false);
const progress = ref(100);
let timeoutId = null;
let progressInterval = null;

const variantClasses = computed(() => {
  const classes = {
    success: 'bg-bg-surface border-success/30',
    error: 'bg-bg-surface border-danger/30',
    warning: 'bg-bg-surface border-warning/30',
    info: 'bg-bg-surface border-primary/30'
  };
  return classes[props.type] || classes.info;
});

const iconBgClass = computed(() => {
  const classes = {
    success: 'bg-success/20 text-success',
    error: 'bg-danger/20 text-danger',
    warning: 'bg-warning/20 text-warning',
    info: 'bg-primary/20 text-primary'
  };
  return classes[props.type] || classes.info;
});

const progressClass = computed(() => {
  const classes = {
    success: 'bg-success',
    error: 'bg-danger',
    warning: 'bg-warning',
    info: 'bg-primary'
  };
  return classes[props.type] || classes.info;
});

function close() {
  visible.value = false;
  if (timeoutId) {
    clearTimeout(timeoutId);
    timeoutId = null;
  }
  if (progressInterval) {
    clearInterval(progressInterval);
    progressInterval = null;
  }
  emit('close');
}

function show() {
  visible.value = true;
  progress.value = 100;

  if (props.autoClose && props.duration > 0) {
    timeoutId = setTimeout(close, props.duration);

    // Update progress bar
    const interval = 50;
    const decrement = (100 / props.duration) * interval;
    progressInterval = setInterval(() => {
      progress.value = Math.max(0, progress.value - decrement);
    }, interval);
  }
}

onMounted(() => {
  show();
});

onUnmounted(() => {
  if (timeoutId) clearTimeout(timeoutId);
  if (progressInterval) clearInterval(progressInterval);
});

defineExpose({ show, close });
</script>

<style scoped>
.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s ease-out;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}
</style>
