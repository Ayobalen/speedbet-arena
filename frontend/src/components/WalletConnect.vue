<template>
  <div class="relative">
    <!-- Connected State -->
    <div v-if="isConnected" class="flex items-center gap-3">
      <div class="hidden sm:flex items-center gap-2 px-3 py-2 bg-bg-surface rounded-lg border border-border-main">
        <span class="w-2 h-2 bg-success rounded-full"></span>
        <span class="text-text-muted text-sm font-mono">{{ truncateAddress(wallet) }}</span>
      </div>
      <button
        @click="handleDisconnect"
        class="px-4 py-2 bg-bg-surface hover:bg-bg-elevated text-text-muted hover:text-danger rounded-lg border border-border-main hover:border-danger/50 transition-all duration-fast text-sm font-medium"
      >
        Disconnect
      </button>
    </div>

    <!-- Connecting State -->
    <button
      v-else-if="isConnecting"
      disabled
      class="flex items-center gap-2 px-5 py-2.5 bg-bg-elevated text-text-dim rounded-lg font-semibold cursor-not-allowed"
    >
      <svg class="animate-spin h-4 w-4" fill="none" viewBox="0 0 24 24">
        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
      </svg>
      <span>Connecting...</span>
    </button>

    <!-- Disconnected State -->
    <div v-else class="flex flex-col items-end gap-2">
      <button
        @click="handleConnect"
        class="flex items-center gap-2 px-5 py-2.5 bg-primary hover:bg-primary-hover text-white rounded-lg font-semibold transition-colors"
      >
        <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z" />
        </svg>
        <span>Connect Wallet</span>
      </button>
      <span v-if="error" class="text-danger text-xs max-w-xs truncate animate-slide-down">
        {{ error }}
      </span>
    </div>
  </div>
</template>

<script setup>
import { useLinera } from '../composables/useLinera';

const emit = defineEmits(['connect', 'disconnect']);

const { isConnected, isConnecting, wallet, error, connect, disconnect: doDisconnect } = useLinera();

async function handleConnect() {
  try {
    await connect();
    emit('connect', wallet.value);
  } catch (err) {
    console.error('Connection failed:', err);
  }
}

function handleDisconnect() {
  doDisconnect();
  emit('disconnect');
}

function truncateAddress(addr) {
  if (!addr) return '';
  const addrStr = typeof addr === 'object' ? (addr.id || addr.address || JSON.stringify(addr)) : String(addr);
  if (addrStr.length <= 12) return addrStr;
  return `${addrStr.slice(0, 6)}...${addrStr.slice(-4)}`;
}
</script>
