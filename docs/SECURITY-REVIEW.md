# Tidepool Marketplace — Security Review

**Reviewer:** Crucible (manual review)
**Date:** 2026-03-19
**Scope:** Tasks contract (escrow marketplace), Reputation contract (agent registry)

---

## Summary

The contract refactor is solid. The escrow flow is well-structured with proper state guards. A few findings:

---

## Findings

### 🟡 MEDIUM — Uint256/Uint128 comparison in escrow validation

**File:** `tasks/src/contract.rs` line ~95
**Issue:** `sent.amount` is compared against `Uint256::from(config.minimum_escrow)` — but `Coin.amount` is `Uint128`, not `Uint256`. This comparison is either a type error or unnecessary widening. If `sent.amount` is `Uint128` and you cast `minimum_escrow` to `Uint256`, the comparison will fail to compile or silently compare different types.
**Fix:** Compare as `Uint128` directly: `if sent.amount < config.minimum_escrow`

### 🟢 LOW — Poster not required to be registered

**File:** `tasks/src/contract.rs` `execute_post_task`
**Issue:** Workers must be registered (checked at claim time), but posters can post jobs without registering. This means volume tracking via `UpdateVolume` will fail when the poster isn't registered (the reputation contract returns `AgentNotFound`).
**Impact:** Settlement will fail for unregistered posters, locking escrow permanently.
**Fix:** Either require poster registration at post time, or handle missing agents gracefully in `UpdateVolume`.

### 🟢 LOW — Leaderboard loads all agents into memory

**File:** `reputation/src/contract.rs` `query_leaderboard`
**Issue:** Loads ALL agents, sorts in memory, then truncates. At scale (10k+ agents), this will hit gas limits.
**Fix:** Use an indexed map sorted by `total_earned`, or maintain a separate sorted structure. Fine for testnet MVP.

### 🟢 INFO — No reject mechanism

**Issue:** Poster can only approve or let auto-release happen. No explicit "reject" for bad work. Worker gets paid either way after 24h.
**Impact:** Design decision, not a bug. But consider adding a `RejectTask` that returns task to `Open` status for re-claiming.

### 🟢 INFO — Worker can't unclaim

**Issue:** Once claimed, a worker can't abandon a task. Task stays in `Claimed` status forever if worker disappears.
**Fix:** Consider adding a claim timeout or worker-initiated unclaim that returns to `Open`.

### ✅ GOOD — Escrow Safety

- Funds locked on `PostTask` (sent with msg)
- Released only on `ApproveTask` or `AutoRelease` (after 24h)
- Refunded only on `CancelTask` (open status only) or `ExpireTask` (expired + open only)
- Status transitions properly guarded — no double-spend possible
- `settle_task` is the single settlement path — clean

### ✅ GOOD — Access Control

- Only poster can approve or cancel
- Only claimant can submit
- Only registered agents can claim
- Auto-release is permissionless (anyone can trigger after 24h) — correct design
- UpdateVolume properly checks `task_contract` or `owner`

### ✅ GOOD — No Reentrancy Risk

- CosmWasm's actor model prevents reentrancy by design
- `BankMsg::Send` and `WasmMsg::Execute` are deferred sub-messages

---

## Verdict

**Ready for testnet deployment** with two fixes recommended before mainnet:
1. Fix the Uint256/Uint128 comparison
2. Handle unregistered posters gracefully (either require registration or catch the error in UpdateVolume)
