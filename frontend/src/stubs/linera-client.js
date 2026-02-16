// Mock Linera Client for development without the real @linera/client package
// In production, this will be replaced by the real package via CDN or npm

export class Faucet {
  constructor(url) {
    this.url = url;
    console.log('[Mock] Faucet created:', url);
  }

  async createWallet() {
    console.log('[Mock] Creating mock wallet');
    return { id: 'mock-wallet-' + Date.now() };
  }

  async claimChain(client) {
    console.log('[Mock] Claiming mock chain');
    // Also initialize the application reference for the client
    client._mockApplication = this.createMockApplication();
    return 'e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65';
  }

  createMockApplication() {
    return {
      query: async (request) => {
        console.log('[Mock] Query:', request);
        const parsed = JSON.parse(request);

        // Return mock data based on query
        return JSON.stringify({
          data: {
            chainId: 'e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65',
            feeBps: 100,
            minBet: '1000000',
            maxBet: '100000000000',
            paused: false,
            queueLength: 2,
            queue: [
              {
                player: '0x1234567890abcdef',
                asset: 'BTC',
                betAmount: '1000000000',
                joinedAt: Date.now().toString()
              }
            ],
            totalDuels: 42,
            totalVolume: '5000000000000',
            totalFees: '50000000000'
          }
        });
      }
    };
  }
}

export class Client {
  constructor(wallet) {
    this.wallet = wallet;
    console.log('[Mock] Client created');
  }

  frontend() {
    const self = this;
    return {
      application: async (appId) => {
        console.log('[Mock] Getting application:', appId);
        // Return the pre-created mock application or create a new one
        return self._mockApplication || {
          query: async (request) => {
            console.log('[Mock] Query:', request);
            return JSON.stringify({
              data: {
                chainId: 'e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65',
                feeBps: 100,
                minBet: '1000000',
                maxBet: '100000000000',
                paused: false,
                queueLength: 2,
                queue: [],
                totalDuels: 42,
                totalVolume: '5000000000000',
                totalFees: '50000000000'
              }
            });
          }
        };
      }
    };
  }

  onNotification(callback) {
    console.log('[Mock] Notification subscription registered');
    // Simulate occasional notifications
    setInterval(() => {
      callback({ reason: { NewBlock: { height: Date.now() } } });
    }, 30000);
  }
}

// Default export is the WASM initialization function
export default async function initWasm() {
  console.log('[Mock] WASM initialized (mock mode)');
  return true;
}
