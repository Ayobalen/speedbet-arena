<template>
  <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
    <!-- BTC Price Card -->
    <div class="bg-bg-surface border border-border-main rounded-xl p-4 hover:border-border-hover transition-colors duration-fast">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-warning/10 flex items-center justify-center">
            <span class="text-warning text-xl font-bold">₿</span>
          </div>
          <div>
            <div class="text-text-muted text-sm">Bitcoin</div>
            <div class="text-text-dim text-xs">BTC/USD</div>
          </div>
        </div>
        <div class="text-right">
          <div class="text-2xl font-bold font-mono text-text-main" :class="btcPriceClass">
            ${{ formatPrice(btcPrice) }}
          </div>
          <div class="text-xs text-text-muted">Live</div>
        </div>
      </div>
    </div>

    <!-- ETH Price Card -->
    <div class="bg-bg-surface border border-border-main rounded-xl p-4 hover:border-border-hover transition-colors duration-fast">
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-info/10 flex items-center justify-center">
            <span class="text-info text-xl font-bold">Ξ</span>
          </div>
          <div>
            <div class="text-text-muted text-sm">Ethereum</div>
            <div class="text-text-dim text-xs">ETH/USD</div>
          </div>
        </div>
        <div class="text-right">
          <div class="text-2xl font-bold font-mono text-text-main" :class="ethPriceClass">
            ${{ formatPrice(ethPrice) }}
          </div>
          <div class="text-xs text-text-muted">Live</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
  btcPrice: {
    type: Number,
    default: 0
  },
  ethPrice: {
    type: Number,
    default: 0
  }
});

const btcPriceClass = ref('');
const ethPriceClass = ref('');

let prevBtcPrice = 0;
let prevEthPrice = 0;

watch(() => props.btcPrice, (newPrice) => {
  if (prevBtcPrice > 0 && newPrice !== prevBtcPrice) {
    btcPriceClass.value = newPrice > prevBtcPrice ? 'price-up' : 'price-down';
    setTimeout(() => { btcPriceClass.value = ''; }, 300);
  }
  prevBtcPrice = newPrice;
});

watch(() => props.ethPrice, (newPrice) => {
  if (prevEthPrice > 0 && newPrice !== prevEthPrice) {
    ethPriceClass.value = newPrice > prevEthPrice ? 'price-up' : 'price-down';
    setTimeout(() => { ethPriceClass.value = ''; }, 300);
  }
  prevEthPrice = newPrice;
});

function formatPrice(microUsd) {
  if (!microUsd) return '0.00';
  return (microUsd / 1_000_000).toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  });
}
</script>
