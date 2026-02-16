<template>
  <header class="header">
    <div class="header-content">
      <!-- Logo & Brand -->
      <div class="brand">
        <div class="logo-icon">
          <svg viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M13 10V3L4 14h7v7l9-11h-7z" fill="currentColor"/>
          </svg>
        </div>
        <div class="brand-text">
          <span class="brand-name">SpeedBet</span>
          <span class="brand-arena">Arena</span>
        </div>
      </div>

      <!-- Navigation -->
      <nav class="nav-links">
        <a
          href="#"
          class="nav-link"
          :class="{ active: activeTab === 'play' }"
          @click.prevent="$emit('navigate', 'play')"
        >
          Play
        </a>
        <a
          href="#"
          class="nav-link"
          :class="{ active: activeTab === 'leaderboard' }"
          @click.prevent="$emit('navigate', 'leaderboard')"
        >
          Leaderboard
        </a>
        <a
          href="#"
          class="nav-link"
          :class="{ active: activeTab === 'stats' }"
          @click.prevent="$emit('navigate', 'stats')"
        >
          Stats
        </a>
      </nav>

      <!-- Right Section: Chain Info + Wallet -->
      <div class="header-right">
        <slot name="chain-info"></slot>
        <slot name="wallet"></slot>
      </div>
    </div>
  </header>
</template>

<script setup>
defineProps({
  activeTab: {
    type: String,
    default: 'play'
  }
});

defineEmits(['navigate']);
</script>

<style scoped>
.header {
  background-color: #111820;
  border-bottom: 1px solid #1E2A36;
  padding: 0 1.5rem;
  position: sticky;
  top: 0;
  z-index: 100;
}

.header-content {
  max-width: 1280px;
  margin: 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 64px;
}

.brand {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.logo-icon {
  width: 32px;
  height: 32px;
  color: #06B6D4;
}

.brand-text {
  display: flex;
  align-items: baseline;
  gap: 0.25rem;
}

.brand-name {
  font-size: 1.25rem;
  font-weight: 700;
  color: #F8FAFC;
}

.brand-arena {
  font-size: 1.25rem;
  font-weight: 700;
  color: #06B6D4;
}

.nav-links {
  display: flex;
  gap: 0.5rem;
}

.nav-link {
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  color: #94A3B8;
  text-decoration: none;
  font-weight: 500;
  transition: all 150ms ease-out;
}

.nav-link:hover {
  color: #F8FAFC;
  background-color: rgba(6, 182, 212, 0.1);
}

.nav-link.active {
  color: #F8FAFC;
  background-color: #06B6D4;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

/* Mobile responsiveness */
@media (max-width: 768px) {
  .nav-links {
    display: none;
  }

  .header-content {
    height: 56px;
  }
}
</style>
