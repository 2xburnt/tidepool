# Research Memo: zkTLS Integration for Tidepool

**Date:** March 19, 2026
**Topic:** Verifying off-chain agent work via zkTLS
**Status:** Sprint 2 Planning

## 1. Executive Summary
zkTLS (Zero-Knowledge Transport Layer Security) enables Tidepool agents to prove they performed actions on Web2 platforms (GitHub, Twitter, Linear) without exposing API keys or requiring direct platform integrations (OAuth). This allows "permissionless" task verification for the reputation system.

## 2. Core Technologies

### TLSNotary / MPC-TLS
- **Mechanism:** Splits the TLS session keys between the Prover (Agent) and a Verifier (MPC node) using Multi-Party Computation.
- **Flow:** The Agent browses the target site (e.g., GitHub PR page). The Verifier signs off on the encrypted stream but cannot see the plaintext. The Agent generates a ZK proof that the decrypted content matches a specific template (e.g., "PR #123 merged").
- **Pros:** High privacy, decentralized verification.

### Reclaim Protocol
- **Mechanism:** Uses an HTTP proxy and ZK circuits to generate proofs of HTTPS traffic.
- **Integration:** Provides an SDK for generating proofs and on-chain verification contracts.
- **Trust Model:** Relies on the integrity of the Reclaim witness nodes (or TEEs in newer versions).

### zkPass
- **Mechanism:** Combines MPC and 3P-TLS. Uses a "Transporter" node to facilitate the handshake.
- **Output:** A compact ZK proof verifiable on-chain.

## 3. Integration with Tidepool (Xion/CosmWasm)

### Architecture
1. **Client-Side (Agent):** Agent runs a local zkTLS prover (e.g., TLSNotary wasm or Reclaim SDK) when completing a task.
2. **Task Definition:** Tidepool task definitions must include a "regex schema" defining what the response should look like (e.g., `{"merged": true, "author": "agent_id"}`).
3. **On-Chain Verification:**
   - A generic `Verifier` contract on Xion stores the verification keys for the zkTLS provider.
   - When an Agent submits a "Task Complete" transaction, they include the zkTLS proof bytes.
   - The contract verifies the proof against the task's regex schema.

### Trust Model
- **Web2 Server Security:** Relies on the target server (e.g., GitHub) properly implementing TLS.
- **Provider Security:** We must trust the zkTLS provider's MPC setup (or TEE attestation) not to collude with the Agent.
- **Data Freshness:** Proofs must include timestamps to prevent replay attacks.

## 4. Limitations & Maturity
- **Brittle Parsing:** If GitHub/Twitter changes their HTML/JSON structure, the regex schema breaks. Requires maintenance.
- **Latency:** Proof generation can take 10-60 seconds on client hardware.
- **Maturity:** Reclaim and zkPass are mainnet-ready but evolving. TLSNotary is highly decentralized but requires more custom integration work.

## 5. Recommendation
Start with **Reclaim Protocol** or **zkPass** for Sprint 2 due to their existing CosmWasm verifier libraries. Use for high-value tasks (e.g., "Merged PR") where API integration is too heavy.
