# Research Memo: Cross-Chain Reputation via IBC

**Date:** March 19, 2026
**Topic:** Porting Tidepool Reputation (Xion) to IBC Ecosystem
**Status:** Sprint 2 Planning

## 1. Objective
Enable agents with high Reputation Scores (XP/Badges) on Xion to access gated opportunities on other Cosmos chains (e.g., becoming a validator on a consumer chain, accessing DeFi leverage on Osmosis) without re-earning trust.

## 2. Technical Architecture

### Proof Format: ICS-23 Merkle Proofs
- **Mechanism:** Standard Cosmos state verification.
- **Flow:**
  1. Tidepool contract on Xion updates an agent's score in its state: `Store[AgentAddr] = Score`.
  2. Agent requests a proof of this key-value pair from a Xion archive node.
  3. Agent submits this proof to a destination chain contract.
- **Verification:** The destination chain's IBC Light Client (07-tendermint) verifies the proof against the Xion consensus state root.

### Transport Layer: IBC Packets vs. Queries
- **Option A: Interchain Queries (ICQ):** The destination chain *pulls* data. "What is Agent X's score on Xion?"
  - *Pros:* synchronous-ish UX on destination.
  - *Cons:* Requires ICQ relayer infrastructure (expensive/complex).
- **Option B: IBC Packet Push (Recommended):** The Agent *pushes* their score.
  - Tidepool (Xion) sends an IBC packet -> Destination Chain.
  - Packet contains: `{ agent_id, score, badges, timestamp }`.
  - Destination contract receives packet and updates local registry.

## 3. Integration with Xion Abstract Accounts
- **Identity Mapping:** Xion uses Abstract Accounts (smart contract wallets).
- **Challenge:** The address `xion1...` does not exist on `osmosis1...`.
- **Solution:**
  - **Interchain Accounts (ICA):** The Xion Abstract Account controls an ICA on the target chain. The reputation is associated with the ICA controller.
  - **Account Linking:** The user signs a message on the target chain proving ownership of the Xion Abstract Account, linking the two identities in the Tidepool registry.

## 4. Relaying Model
- Standard IBC relayers (Hermes, Go-Relayer) can handle the packet flow.
- No custom relayer logic needed if using standard IBC send/receive packet flows.

## 5. Ecosystem Precedents
- **Bad Kids / Stargaze:** NFT ownership gating across chains (often uses ICQ or snapshot exports).
- **Mars Protocol:** Credit accounts moving across chains (similar trust model).

## 6. Sprint 2 Implementation Plan
1. Define the `ReputationPacket` IBC data structure.
2. Implement `ibc_packet_send` in the Tidepool Core contract (Xion).
3. Create a lightweight `ReputationReceiver` contract for standard CosmWasm chains (Neutron/Osmosis).
