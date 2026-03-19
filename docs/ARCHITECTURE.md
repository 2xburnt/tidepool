# Tidepool Frontend Architecture

> Worker-based deployment, API caching, and bundle optimization.

## Contract Addresses

| Contract | Address |
|----------|---------|
| Reputation | `xion13a7zj373ztxcm5mux7hvkvp3mr575t9rmnkhcgm0xvma2fxc6dzqkdaewv` |
| Tasks | `xion15tzqfeykxkfvflty63zg6vws75e2u334vjr8s7w7ja8rd4gpd9es7lld4c` |

---

## 1. Worker Serving Strategy

### Current Setup
- **Single Worker** (`src/worker.ts`) handles both API routes and static assets
- `wrangler.jsonc` uses `assets.directory: "./dist"` with `ASSETS` binding
- Vite builds React app to `./dist`, Worker serves it via `env.ASSETS.fetch(request)`

### Architecture

```
Request → Worker
            ├── /api/*  → handleApi() → Chain RPC → Response
            └── /*      → env.ASSETS.fetch() → Static Files
```

### Why Workers (not Pages)
- ✅ API routes require custom logic (contract queries, response shaping)
- ✅ Single deployment unit for frontend + API
- ✅ Full control over caching headers
- ❌ Pages Functions are limited to simple transforms

### Implementation Checklist
- [x] Worker entry at `src/worker.ts`
- [x] Assets binding configured in `wrangler.jsonc`
- [x] API routes under `/api/*` namespace
- [x] Fallback to ASSETS for SPA routes
- [ ] Add `_headers` file for static asset caching (or set in Worker)
- [ ] Consider `_redirects` for SPA client-side routing fallback

---

## 2. API Cache Approach

### Current State
- Basic `Cache-Control: public, max-age=10` on all 200 responses
- No Cloudflare Cache API usage
- Every request hits chain RPC

### Recommended Strategy

| Route | TTL | Stale-While-Revalidate | Cache Key |
|-------|-----|------------------------|-----------|
| `/api/leaderboard` | 30s | 60s | `leaderboard:{limit}` |
| `/api/tasks` | 15s | 30s | `tasks:{limit}:{status}` |
| `/api/agent?address=X` | 60s | 120s | `agent:{address}` |
| `/api/task?id=X` | 30s | 60s | `task:{id}` |
| `/api/stats` | 30s | 60s | `stats` |
| `/api/health` | 0 | 0 | — |

### Implementation: Cloudflare Cache API

```typescript
const cache = caches.default;

async function cachedQuery<T>(
  cacheKey: string,
  ttlSeconds: number,
  queryFn: () => Promise<T>
): Promise<T> {
  const cacheUrl = new URL(`https://cache.internal/${cacheKey}`);
  const cached = await cache.match(cacheUrl);
  
  if (cached) {
    return cached.json() as Promise<T>;
  }
  
  const data = await queryFn();
  const response = new Response(JSON.stringify(data), {
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": `public, max-age=${ttlSeconds}`,
    },
  });
  
  // Non-blocking cache write
  ctx.waitUntil(cache.put(cacheUrl, response.clone()));
  return data;
}
```

### Cache Invalidation

**Option A: Time-based expiry (recommended for MVP)**
- Let TTLs expire naturally
- Chain state changes are eventually consistent (15-30s delay acceptable)

**Option B: Active invalidation (future)**
- Listen to chain events via WebSocket
- Purge cache keys on relevant transactions
- Requires additional infra (event listener service)

### Response Headers

```typescript
function json(data: unknown, ttl = 30): Response {
  return new Response(JSON.stringify(data), {
    headers: {
      "Content-Type": "application/json",
      "Cache-Control": `public, max-age=${ttl}, stale-while-revalidate=${ttl * 2}`,
      "CDN-Cache-Control": `max-age=${ttl * 2}`,
      "Vary": "Accept-Encoding",
    },
  });
}
```

### Implementation Checklist
- [ ] Add `ExecutionContext` parameter to fetch handler for `waitUntil`
- [ ] Implement `cachedQuery` wrapper using Cache API
- [ ] Update each route with appropriate TTL
- [ ] Add `stale-while-revalidate` for better UX
- [ ] Log cache hit/miss ratio for monitoring

---

## 3. Code-Splitting Plan

### Problem
- Bundle size: ~6.9MB (mostly cosmjs + protobuf)
- `@burnt-labs/abstraxion` pulls in full chain client on initial load
- Initial page load blocked by wallet code user may never use

### Bundle Analysis

| Package | Approx Size | Lazy-loadable? |
|---------|-------------|----------------|
| `@cosmjs/*` | ~3MB | ✅ Yes |
| `protobufjs` | ~1.5MB | ✅ Yes (via cosmjs) |
| `@burnt-labs/abstraxion` | ~1MB | ✅ Yes |
| React + DOM | ~150KB | ❌ No |
| App code | ~200KB | ❌ No |

### Strategy: Lazy Load Wallet Module

**Phase 1: Split AbstraxionProvider**

```tsx
// App.tsx - Initial load (fast)
import React, { Suspense, lazy, useState } from "react";
import { Dashboard } from "./Dashboard"; // Read-only components

const WalletProvider = lazy(() => import("./WalletProvider"));

export default function App() {
  const [walletRequested, setWalletRequested] = useState(false);

  return (
    <div className="container">
      <header>
        <h1>🌊 Tidepool</h1>
        {!walletRequested ? (
          <button onClick={() => setWalletRequested(true)}>
            Connect Wallet
          </button>
        ) : (
          <Suspense fallback={<span>Loading wallet...</span>}>
            <WalletProvider>
              <WalletButton />
            </WalletProvider>
          </Suspense>
        )}
      </header>
      <Dashboard /> {/* Read-only, no wallet deps */}
    </div>
  );
}
```

**Phase 2: Separate Wallet Module**

```tsx
// WalletProvider.tsx - Lazy loaded (~5MB)
import {
  AbstraxionProvider,
  AbstraxionEmbed,
  useAbstraxionAccount,
  useAbstraxionSigningClient,
} from "@burnt-labs/abstraxion";
import { CHAIN_CONFIG } from "./config";

export default function WalletProvider({ children }: { children: React.ReactNode }) {
  return (
    <AbstraxionProvider config={{
      rpcUrl: CHAIN_CONFIG.rpcEndpoint,
      restUrl: CHAIN_CONFIG.restEndpoint,
      contracts: [CHAIN_CONFIG.contracts.reputation, CHAIN_CONFIG.contracts.tasks],
    }}>
      {children}
      <AbstraxionEmbed />
    </AbstraxionProvider>
  );
}

export { useAbstraxionAccount, useAbstraxionSigningClient };
```

### Vite Configuration

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react({ fastRefresh: false })],
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          // Keep React in main bundle
          'react-vendor': ['react', 'react-dom'],
          // Heavy chain deps in separate chunk
          'chain-vendor': [
            '@cosmjs/cosmwasm-stargate',
            '@cosmjs/stargate',
            '@cosmjs/proto-signing',
            '@cosmjs/encoding',
          ],
          // Wallet UI in separate chunk
          'wallet': ['@burnt-labs/abstraxion'],
        },
      },
    },
    chunkSizeWarningLimit: 1000, // Expect large chain chunk
  },
});
```

### Expected Results

| Chunk | Size | Load Timing |
|-------|------|-------------|
| `main.js` | ~300KB | Immediate |
| `react-vendor.js` | ~150KB | Immediate |
| `chain-vendor.js` | ~4MB | On wallet click |
| `wallet.js` | ~1MB | On wallet click |

**Initial load: ~450KB** (from ~6.9MB)

### Implementation Checklist
- [ ] Create `WalletProvider.tsx` as separate module
- [ ] Update `App.tsx` to lazy-load wallet on demand
- [ ] Configure `manualChunks` in `vite.config.ts`
- [ ] Move `client.ts` queries to use fetch (no cosmjs for reads)
- [ ] Add loading state for wallet chunk
- [ ] Test with Lighthouse / WebPageTest
- [ ] Consider preloading wallet chunk on hover (optional)

### Bonus: Preload on Interaction

```tsx
// Preload wallet chunk when user hovers connect button
const preloadWallet = () => {
  import("./WalletProvider");
};

<button 
  onMouseEnter={preloadWallet}
  onClick={() => setWalletRequested(true)}
>
  Connect Wallet
</button>
```

---

## Summary

| Area | Current | Target |
|------|---------|--------|
| Worker serving | ✅ Working | Add caching headers |
| API caching | 10s flat | Route-specific TTLs via Cache API |
| Bundle size | 6.9MB | ~450KB initial, lazy-load rest |

### Priority Order
1. **Code splitting** — biggest user impact, no backend changes
2. **API caching** — reduces chain load, improves latency
3. **Static asset headers** — minor optimization
