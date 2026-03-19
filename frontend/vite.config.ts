import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react({ fastRefresh: false })],
  build: {
    rollupOptions: {
      output: {
        manualChunks(id: string) {
          // React stays in initial bundle
          if (id.includes('node_modules/react-dom') || id.includes('node_modules/react/')) {
            return 'react-vendor';
          }
          // Heavy chain deps — separate chunk, lazy-loaded via WalletProvider
          if (id.includes('@cosmjs/')) {
            return 'chain-vendor';
          }
          // Wallet UI — separate chunk
          if (id.includes('@burnt-labs/abstraxion')) {
            return 'wallet';
          }
          // Protobuf — often pulled by cosmjs
          if (id.includes('protobufjs') || id.includes('google-protobuf') || id.includes('@bufbuild')) {
            return 'chain-vendor';
          }
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },
})
