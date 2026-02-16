<template>
  <div class="bg-bg-surface/50 border-b border-border-main">
    <div class="container-app py-3">
      <div class="flex flex-wrap items-center justify-between gap-3">
        <!-- Network Badge -->
        <div class="flex items-center gap-2 px-3 py-1.5 bg-primary/10 rounded-full border border-primary/30">
          <span :class="['w-2 h-2 rounded-full', isConnected ? 'bg-success animate-pulse-glow' : 'bg-primary animate-pulse-glow']"></span>
          <span class="text-primary text-sm font-medium">Conway Testnet</span>
        </div>

        <!-- Chain & App IDs -->
        <div class="flex flex-wrap items-center gap-4 text-sm">
          <!-- Chain ID -->
          <div class="flex items-center gap-2">
            <span class="text-text-dim">Chain:</span>
            <div class="flex items-center gap-1.5 px-2 py-1 bg-bg-main rounded-md border border-border-main">
              <code class="text-primary font-mono text-xs">{{ truncatedChainId }}</code>
              <button
                v-if="chainId"
                @click="copyChainId"
                class="p-1 hover:bg-bg-elevated rounded transition-colors duration-fast"
                :title="copied ? 'Copied!' : 'Copy Chain ID'"
              >
                <svg v-if="copied" class="w-3.5 h-3.5 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
                <svg v-else class="w-3.5 h-3.5 text-text-muted hover:text-text-main" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                </svg>
              </button>
            </div>
          </div>

          <!-- App ID -->
          <template v-if="appId">
            <span class="text-border-main">|</span>
            <div class="flex items-center gap-2">
              <span class="text-text-dim">App:</span>
              <div class="flex items-center gap-1.5 px-2 py-1 bg-bg-main rounded-md border border-border-main">
                <code class="text-primary font-mono text-xs">{{ truncatedAppId }}</code>
              </div>
            </div>
          </template>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue';
import { useLinera } from '../composables/useLinera';

const props = defineProps({
  appId: {
    type: String,
    default: ''
  }
});

const { chainId, isConnected } = useLinera();

const copied = ref(false);

const truncatedChainId = computed(() => {
  if (!chainId.value) return 'Not Connected';
  if (chainId.value.length <= 16) return chainId.value;
  return `${chainId.value.slice(0, 8)}...${chainId.value.slice(-6)}`;
});

const truncatedAppId = computed(() => {
  if (!props.appId) return '';
  if (props.appId.length <= 16) return props.appId;
  return `${props.appId.slice(0, 8)}...${props.appId.slice(-6)}`;
});

async function copyChainId() {
  if (!chainId.value) return;
  try {
    await navigator.clipboard.writeText(chainId.value);
    copied.value = true;
    setTimeout(() => { copied.value = false; }, 2000);
  } catch (err) {
    console.error('Failed to copy chain ID:', err);
  }
}
</script>

