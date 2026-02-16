<template>
  <div class="min-h-screen bg-bg-main overflow-hidden">
    <!-- Minimal nav -->
    <nav class="container-app py-6 flex items-center justify-between relative z-20">
      <div class="flex items-center gap-3">
        <div class="w-10 h-10 rounded-lg bg-primary/20 flex items-center justify-center border border-primary/30">
          <span class="text-xl font-bold text-primary">VS</span>
        </div>
        <span class="text-xl font-bold text-text-main">SpeedBet <span class="text-primary">Arena</span></span>
      </div>
      <div class="flex items-center gap-6">
        <div class="hidden sm:flex items-center gap-2 text-sm text-text-muted">
          <span class="w-2 h-2 bg-success rounded-full"></span>
          Conway Testnet
        </div>
        <a
          href="https://linera.dev"
          target="_blank"
          rel="noopener noreferrer"
          class="text-text-muted hover:text-primary transition-colors text-sm"
        >
          Built on Linera
        </a>
      </div>
    </nav>

    <!-- Hero split layout -->
    <div class="container-app relative z-10">
      <div class="grid lg:grid-cols-2 gap-12 lg:gap-20 items-center min-h-[calc(100vh-120px)] py-12">

        <!-- Left: Text content -->
        <div class="order-2 lg:order-1">
          <div class="inline-flex items-center gap-2 px-3 py-1.5 bg-danger/10 border border-danger/30 rounded-full text-danger text-sm font-medium mb-6">
            <span class="relative flex h-2 w-2">
              <span class="absolute inline-flex h-full w-full rounded-full bg-danger opacity-75"></span>
              <span class="relative inline-flex rounded-full h-2 w-2 bg-danger"></span>
            </span>
            Live PvP Betting
          </div>

          <h1 class="text-4xl sm:text-5xl lg:text-6xl font-black text-text-main leading-[1.1] mb-6">
            Predict the market.<br/>
            <span class="text-primary">Defeat your opponent.</span>
          </h1>

          <p class="text-lg text-text-muted mb-8 max-w-lg">
            1v1 crypto price prediction battles. Pick UP or DOWN. 60 seconds.
            Winner takes the entire pot. No house edge.
          </p>

          <!-- CTA with live stats -->
          <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4 mb-12">
            <button
              @click="$emit('enter')"
              class="group flex items-center gap-3 px-8 py-4 bg-primary hover:bg-primary-hover rounded-xl font-bold text-lg text-white transition-colors"
            >
              Enter Arena
              <svg class="w-5 h-5 transition-transform group-hover:translate-x-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
              </svg>
            </button>
            <div class="text-sm text-text-dim">
              <span class="text-text-muted font-medium">0% platform fee</span> · Winner takes all
            </div>
          </div>

          <!-- Quick stats row -->
          <div class="flex items-center gap-8 pt-8 border-t border-border-main">
            <div>
              <div class="text-2xl font-bold font-mono text-text-main">60s</div>
              <div class="text-sm text-text-dim">per duel</div>
            </div>
            <div class="w-px h-10 bg-border-main"></div>
            <div>
              <div class="text-2xl font-bold font-mono text-success">2x</div>
              <div class="text-sm text-text-dim">payout</div>
            </div>
            <div class="w-px h-10 bg-border-main"></div>
            <div>
              <div class="text-2xl font-bold font-mono text-primary">BTC/ETH</div>
              <div class="text-sm text-text-dim">live prices</div>
            </div>
          </div>
        </div>

        <!-- Right: Battle visualization -->
        <div class="order-1 lg:order-2 flex justify-center">
          <div class="relative w-full max-w-md">
            <!-- Glow background -->
            <div class="absolute inset-0 bg-primary/10 rounded-3xl blur-3xl"></div>

            <!-- Battle card -->
            <div class="relative bg-bg-surface border border-border-main rounded-2xl p-6 sm:p-8">
              <!-- Header -->
              <div class="flex items-center justify-between mb-6">
                <div class="flex items-center gap-2">
                  <span class="w-2 h-2 bg-success rounded-full"></span>
                  <span class="text-sm font-medium text-text-muted">Live Duel</span>
                </div>
                <div class="px-3 py-1 bg-warning/10 border border-warning/30 rounded-full">
                  <span class="text-warning text-sm font-bold font-mono">00:42</span>
                </div>
              </div>

              <!-- Players -->
              <div class="flex items-center justify-between mb-6">
                <!-- Player 1 -->
                <div class="flex-1 text-center">
                  <div class="w-16 h-16 mx-auto mb-3 rounded-xl bg-success/20 border-2 border-success flex items-center justify-center">
                    <svg class="w-8 h-8 text-success" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 10l7-7m0 0l7 7m-7-7v18" />
                    </svg>
                  </div>
                  <div class="text-sm font-medium text-text-main mb-1">0x4f2...8a1</div>
                  <div class="text-xs text-success font-bold">UP</div>
                </div>

                <!-- VS -->
                <div class="px-4">
                  <div class="w-12 h-12 rounded-full bg-bg-main border-2 border-border-main flex items-center justify-center">
                    <span class="text-sm font-black text-text-muted">VS</span>
                  </div>
                </div>

                <!-- Player 2 -->
                <div class="flex-1 text-center">
                  <div class="w-16 h-16 mx-auto mb-3 rounded-xl bg-danger/20 border-2 border-danger flex items-center justify-center">
                    <svg class="w-8 h-8 text-danger" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
                    </svg>
                  </div>
                  <div class="text-sm font-medium text-text-main mb-1">0x9c7...3b2</div>
                  <div class="text-xs text-danger font-bold">DOWN</div>
                </div>
              </div>

              <!-- Asset and price -->
              <div class="bg-bg-main rounded-xl p-4 mb-4">
                <div class="flex items-center justify-between">
                  <div class="flex items-center gap-3">
                    <div class="w-10 h-10 rounded-full bg-warning/10 flex items-center justify-center">
                      <span class="text-warning text-lg font-bold">₿</span>
                    </div>
                    <div>
                      <div class="font-bold text-text-main">Bitcoin</div>
                      <div class="text-xs text-text-dim">BTC/USD</div>
                    </div>
                  </div>
                  <div class="text-right">
                    <div class="text-xl font-bold font-mono text-success">$97,842</div>
                    <div class="text-xs text-success">+0.42%</div>
                  </div>
                </div>
              </div>

              <!-- Pot -->
              <div class="flex items-center justify-between p-4 bg-primary/5 border border-primary/20 rounded-xl">
                <span class="text-sm text-text-muted">Prize Pool</span>
                <span class="text-lg font-bold font-mono text-primary">20 tokens</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Bottom section: How it works (compact) -->
    <div class="border-t border-border-main bg-bg-surface/50">
      <div class="container-app py-12">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-6 md:gap-8">
          <div class="text-center md:text-left">
            <div class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-primary/10 text-primary font-bold mb-3">1</div>
            <h3 class="font-bold text-text-main mb-1">Join Queue</h3>
            <p class="text-sm text-text-dim">Pick BTC or ETH, set your bet</p>
          </div>
          <div class="text-center md:text-left">
            <div class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-primary/10 text-primary font-bold mb-3">2</div>
            <h3 class="font-bold text-text-main mb-1">Get Matched</h3>
            <p class="text-sm text-text-dim">Instant 1v1 matchmaking</p>
          </div>
          <div class="text-center md:text-left">
            <div class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-primary/10 text-primary font-bold mb-3">3</div>
            <h3 class="font-bold text-text-main mb-1">Predict</h3>
            <p class="text-sm text-text-dim">UP or DOWN in 60 seconds</p>
          </div>
          <div class="text-center md:text-left">
            <div class="inline-flex items-center justify-center w-10 h-10 rounded-lg bg-success/10 text-success font-bold mb-3">4</div>
            <h3 class="font-bold text-text-main mb-1">Win</h3>
            <p class="text-sm text-text-dim">Take the entire pot</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <footer class="border-t border-border-main">
      <div class="container-app py-6 flex flex-col sm:flex-row items-center justify-between gap-4 text-sm text-text-dim">
        <div>SpeedBet Arena · Linera Hackathon 2026</div>
        <div class="flex items-center gap-4">
          <a href="https://linera.dev" target="_blank" rel="noopener noreferrer" class="hover:text-text-muted transition-colors">Docs</a>
          <a href="https://github.com" target="_blank" rel="noopener noreferrer" class="hover:text-text-muted transition-colors">GitHub</a>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
defineEmits(['enter']);
</script>
