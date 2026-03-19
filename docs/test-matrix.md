# Tidepool Contract Test Matrix

## Scope

Source reviewed:
- `contracts/reputation/` — CosmWasm/Rust reputation registry
- `contracts/tasks/` — CosmWasm/Rust task marketplace

Target integration addresses on **xion-testnet-2**:
- **Reputation:** `xion13a7zj373ztxcm5mux7hvkvp3mr575t9rmnkhcgm0xvma2fxc6dzqkdaewv`
- **Tasks:** `xion15tzqfeykxkfvflty63zg6vws75e2u334vjr8s7w7ja8rd4gpd9es7lld4c`

## Test actors / fixtures

- `OWNER` — contract deployer / reputation owner / initial issuer
- `ALICE` — registered agent, usually poster
- `BOB` — registered agent, usually claimant
- `CHARLIE` — unauthorized or unregistered actor
- `TASKS_ADDR` — tasks contract address configured in reputation via `SetTaskContract`
- `REP_ADDR` — reputation contract address configured in tasks at instantiate time

## Review notes worth prioritizing

1. **`tasks::PostTask` does not enforce poster registration.** README/spec language implies posters should be registered, but current code allows any address to post.
2. **`tasks::CompleteTask` is critically dependent on reputation linkage.** If reputation has not whitelisted the tasks contract via `SetTaskContract`, completion should revert.
3. **Unregistered posters create a toxic path:** they can post and a claimant can claim, but completion later reverts when poster XP is awarded in reputation.
4. **Arithmetic is plain `u64` with `+=` / `+`.** XP, counters, `next_task_id`, `AGENT_COUNT`, and `expires_at` should be stress-tested for overflow/wrap behavior.
5. **Optional bounty logic exists in current `contracts/tasks` source.** Even though the prompt focuses on XP flows, refund / payout paths should be sanity-tested because they share the completion / expiry path.

Priority legend:
- **P0** — must pass before production / public usage
- **P1** — important functional coverage
- **P2** — nice-to-have, regression / query / hardening coverage

---

## 1) Reputation contract matrix

| ID | Description | Input / Setup | Expected outcome | Priority |
|---|---|---|---|---|
| REP-01 | Register a new agent | Sender=`ALICE`; execute `Register { name: "alice" }` | Agent record created with `level=1`, `xp=10`, empty badges, `tasks_completed=0`, `tasks_posted=0`, `registered_at=current_height`; `agent_count` increments by 1 | P0 |
| REP-02 | Duplicate registration blocked | Precondition: `ALICE` already registered; `ALICE` executes `Register { name: "alice-2" }` | Tx fails with `AlreadyRegistered`; original agent state unchanged; `agent_count` unchanged | P0 |
| REP-03 | Empty or malformed name behavior | Sender=`CHARLIE`; execute `Register { name: "" }` and optionally whitespace / very long string | **Current code:** registration succeeds because no validation exists. Capture as behavior gap if product expects validation | P2 |
| REP-04 | Owner is initial issuer on instantiate | Query `IsIssuer { address: OWNER }` after instantiate | Returns `true`; config owner equals `OWNER`; `task_contract=None`; `agent_count=0` | P1 |
| REP-05 | Owner can add issuer | Sender=`OWNER`; execute `AddIssuer { address: CHARLIE }`; query `IsIssuer` | Tx succeeds; `IsIssuer(CHARLIE)` returns `true` | P0 |
| REP-06 | Non-owner cannot add issuer | Sender=`ALICE`; execute `AddIssuer { address: CHARLIE }` | Tx fails with `Unauthorized`; issuer set unchanged | P0 |
| REP-07 | Owner can remove issuer | Precondition: `CHARLIE` is issuer; sender=`OWNER`; execute `RemoveIssuer { address: CHARLIE }` | Tx succeeds; `IsIssuer(CHARLIE)` returns `false` | P0 |
| REP-08 | Non-owner cannot remove issuer | Precondition: `CHARLIE` is issuer; sender=`ALICE`; execute `RemoveIssuer { address: CHARLIE }` | Tx fails with `Unauthorized`; issuer status unchanged | P0 |
| REP-09 | Authorized issuer mints a badge | Precondition: `ALICE` registered, `CHARLIE` added as issuer; sender=`CHARLIE`; execute `MintBadge { agent: ALICE, badge_type: "rust", proof: Some("ipfs://proof") }` | Badge appended with issuer/proof/current height; ALICE XP increases by 25; level recomputed with `level_for_xp` | P0 |
| REP-10 | Unauthorized badge mint rejected | Precondition: `ALICE` registered; sender=`BOB` not in issuers; execute `MintBadge { agent: ALICE, badge_type: "rust", proof: None }` | Tx fails with `NotIssuer`; no badge or XP changes | P0 |
| REP-11 | Duplicate badge mint is idempotent | Precondition: issuer already minted badge type `rust` for `ALICE`; same issuer or another issuer mints same `badge_type` again | Tx succeeds but no duplicate badge is added and no extra XP is awarded | P1 |
| REP-12 | Badge mint for nonexistent agent fails | Sender=`OWNER` or valid issuer; execute `MintBadge { agent: UNKNOWN, badge_type: "rust", proof: None }` | Tx fails with `AgentNotFound` | P0 |
| REP-13 | Owner can set tasks contract | Sender=`OWNER`; execute `SetTaskContract { address: TASKS_ADDR }`; query config | Tx succeeds; config `task_contract=TASKS_ADDR` | P0 |
| REP-14 | Non-owner cannot set tasks contract | Sender=`ALICE`; execute `SetTaskContract { address: TASKS_ADDR }` | Tx fails with `Unauthorized`; config unchanged | P0 |
| REP-15 | Owner can award XP directly | Precondition: `ALICE` registered; sender=`OWNER`; execute `AwardXp { agent: ALICE, amount: 40, reason: "manual_bonus" }` | Tx succeeds; ALICE XP += 40; level recomputed correctly | P0 |
| REP-16 | Configured tasks contract can award XP | Precondition: `task_contract=TASKS_ADDR`, `ALICE` registered; sender=`TASKS_ADDR`; execute `AwardXp { agent: ALICE, amount: 50, reason: "task_completion:1" }` | Tx succeeds; ALICE XP increases; level updates | P0 |
| REP-17 | Unauthorized third party cannot award XP | Precondition: `ALICE` registered; sender=`CHARLIE`; execute `AwardXp { agent: ALICE, amount: 50, reason: "fake" }` | Tx fails with `Unauthorized`; XP unchanged | P0 |
| REP-18 | Award XP to nonexistent agent fails | Sender=`OWNER` or `TASKS_ADDR`; execute `AwardXp { agent: UNKNOWN, amount: 1, reason: "test" }` | Tx fails with `AgentNotFound` | P0 |
| REP-19 | Level boundary crossing is correct | Precondition: `ALICE` at 99 XP, then award 1 XP; repeat 249→250, 499→500, 999→1000 | Level changes exactly at thresholds defined by `level_for_xp` (100, 250, 500, 1000, etc.) | P0 |
| REP-20 | Tasks completed counter increments only for authorized caller | Precondition: `ALICE` registered, `task_contract=TASKS_ADDR`; sender=`TASKS_ADDR` executes `IncrementTasksCompleted { agent: ALICE }`, then sender=`CHARLIE` tries same | Authorized call increments by 1; unauthorized call fails with `Unauthorized` | P0 |
| REP-21 | Tasks posted counter increments only for authorized caller | Precondition: `ALICE` registered, `task_contract=TASKS_ADDR`; sender=`TASKS_ADDR` executes `IncrementTasksPosted { agent: ALICE }`, then sender=`CHARLIE` tries same | Authorized call increments by 1; unauthorized call fails with `Unauthorized` | P0 |
| REP-22 | Leaderboard sorts by XP descending | Precondition: several registered agents with distinct XP totals | `Leaderboard { limit }` returns highest-XP agents first and truncates to requested/capped size | P1 |
| REP-23 | Agent listing paginates and caps limit | Populate >100 agents; query `ListAgents { start_after, limit: 500 }` | Returns max 100 agents; `start_after` excludes the provided address and resumes correctly | P2 |
| REP-24 | Query config reflects live state | After registration, issuer changes, and `SetTaskContract` | `Config {}` returns correct owner, linked task contract, and agent count | P2 |

---

## 2) Tasks contract matrix

| ID | Description | Input / Setup | Expected outcome | Priority |
|---|---|---|---|---|
| TASK-01 | Instantiate stores owner, reputation address, and next task ID | Instantiate with `reputation_contract=REP_ADDR` | Config query returns `owner=instantiator`, `reputation_contract=REP_ADDR`, `next_task_id=1` | P1 |
| TASK-02 | Post a basic open task | Sender=`ALICE`; execute `PostTask { title, description, xp_reward: 50, required_badges: [], expires_in_blocks: None }` | Task `id=1` created with `status=Open`, `claimant=None`, timestamps set, `next_task_id` increments to 2 | P0 |
| TASK-03 | Post task with expiry computes deadline correctly | Sender=`ALICE`; current height=`H`; execute `PostTask { ..., expires_in_blocks: Some(100) }` | Task stores `expires_at=H+100` | P1 |
| TASK-04 | Current code allows unregistered poster to post | Sender=`CHARLIE` not registered anywhere; execute `PostTask { ... }` | **Current code:** tx succeeds. Flag as spec / product gap if posters are meant to be registered | P0 |
| TASK-05 | Post task accepts one optional bounty denom | Sender=`ALICE`; attach single coin fund; execute `PostTask { ... }` | Task stores `bounty=Some(coin)` and response includes bounty attribute | P1 |
| TASK-06 | Post task rejects multiple bounty denoms | Sender=`ALICE`; attach two different coins; execute `PostTask { ... }` | Tx fails with `MultipleDenoms`; task not created; `next_task_id` unchanged | P1 |
| TASK-07 | Registered non-poster can claim open task | Precondition: task open, `BOB` registered in reputation, task not expired | `ClaimTask { task_id }` succeeds; task status becomes `Claimed`; `claimant=BOB`; `claimed_at=current_height` | P0 |
| TASK-08 | Poster cannot self-claim | Precondition: `ALICE` posted task; sender=`ALICE`; execute `ClaimTask { task_id }` | Tx fails with `CannotClaimOwnTask`; task remains `Open` | P0 |
| TASK-09 | Unregistered claimant cannot claim | Precondition: task open; sender=`CHARLIE` absent from reputation; execute `ClaimTask { task_id }` | Tx fails with `AgentNotRegistered`; task remains `Open` | P0 |
| TASK-10 | Claim requires all required badges | Precondition: task requires `["rust","audit"]`; `BOB` registered with only `rust` | Tx fails with `MissingBadge { badge: "audit" }`; task remains `Open` | P0 |
| TASK-11 | Claim on expired open task is rejected at boundary | Precondition: task has `expires_at=H`; test at `block.height == H` and `> H` | Claim fails with `TaskNotOpen` once current height reaches expiry; no claimant assigned | P0 |
| TASK-12 | Claim non-open task rejected | Precondition: task already `Claimed`, `Completed`, or `Expired`; sender=`BOB` | Tx fails with `TaskNotOpen` | P0 |
| TASK-13 | Claim nonexistent task rejected | Execute `ClaimTask { task_id: unknown }` | Tx fails with `TaskNotFound` | P1 |
| TASK-14 | Poster approval (`CompleteTask`) finalizes claimed task | Precondition: task claimed by `BOB`; sender=`ALICE` poster executes `CompleteTask { task_id }` | Task moves to `Completed`; `completed_at=current_height`; response includes cross-contract XP messages and reward attributes | P0 |
| TASK-15 | Non-poster cannot approve completion | Precondition: task claimed by `BOB`; sender=`CHARLIE` or `BOB` executes `CompleteTask { task_id }` | Tx fails with `Unauthorized`; task stays `Claimed` | P0 |
| TASK-16 | Cannot approve an unclaimed/open task | Precondition: task status `Open`; sender=`ALICE` executes `CompleteTask { task_id }` | Tx fails with `TaskNotClaimed` | P0 |
| TASK-17 | Completing claimed task with bounty pays claimant | Precondition: claimed task has `bounty=Some(coin)` | Response includes `BankMsg::Send` to claimant and `bounty_paid` attribute; full tx succeeds atomically | P1 |
| TASK-18 | Anyone can expire an expired open task | Precondition: task status `Open`, `expires_at < current_height`; sender=`CHARLIE`; execute `ExpireTask { task_id }` | Task becomes `Expired`; if bounty exists it is refunded to poster | P1 |
| TASK-19 | Anyone can expire an expired claimed task and clear claimant | Precondition: task status `Claimed`, `claimant=BOB`, `expires_at < current_height`; sender=`CHARLIE` | Task becomes `Expired`; `claimant=None`; `claimed_at=None`; bounty refunded to poster if present | P0 |
| TASK-20 | Cannot expire before deadline | Precondition: task has future `expires_at`; execute `ExpireTask { task_id }` before expiry | Tx fails with `TaskNotExpired`; task state unchanged | P0 |
| TASK-21 | Cannot expire task with no expiry | Precondition: task `expires_at=None`; execute `ExpireTask { task_id }` | Tx fails with `TaskNotExpired` | P1 |
| TASK-22 | Cannot expire already completed or already expired task | Precondition: task status `Completed` or `Expired`; execute `ExpireTask { task_id }` | Tx fails with `TaskNotOpen` | P1 |
| TASK-23 | Query single task returns full shape | Query `GetTask { task_id }` for existing task | Response includes poster, title, xp reward, badges, status, claimant, timestamps, bounty | P2 |
| TASK-24 | Query list filters by status and paginates | Create mixed open/claimed/completed/expired tasks; query `ListTasks { status, start_after, limit }` | Returns only matching status, respects `start_after`, caps limit at 100 | P2 |
| TASK-25 | Query posted / claimed task views work | Query `MyPostedTasks { address: ALICE }` and `MyClaimedTasks { address: BOB }` | Returns only tasks posted by ALICE and only tasks claimed by BOB | P2 |

---

## 3) Cross-contract interaction matrix

| ID | Description | Input / Setup | Expected outcome | Priority |
|---|---|---|---|---|
| XCC-01 | Happy-path completion awards claimant XP in reputation | Precondition: `REP_ADDR` is set in tasks config, reputation config has `task_contract=TASKS_ADDR`, `ALICE` and `BOB` are registered, task claimed by `BOB`; `ALICE` completes | Transaction succeeds; reputation receives `AwardXp` for claimant; `BOB.xp += task.xp_reward`; `BOB.level` updates if threshold crossed | P0 |
| XCC-02 | Happy-path completion increments claimant completed counter | Same as XCC-01 | `BOB.tasks_completed += 1` in reputation | P0 |
| XCC-03 | Happy-path completion awards poster XP and increments posted counter | Same as XCC-01 | `ALICE.xp += 20` (`XP_TASK_POSTED_COMPLETED`) and `ALICE.tasks_posted += 1` in reputation | P0 |
| XCC-04 | Completion fails if tasks contract is not linked in reputation | Precondition: tasks instantiated against correct `REP_ADDR`, but reputation `task_contract=None` or wrong address; task is claimed; poster calls `CompleteTask` | One of the outbound reputation executes fails with `Unauthorized`; whole tx reverts atomically; task remains `Claimed`; no XP / counters / bounty transfer occur | P0 |
| XCC-05 | Unregistered poster path causes completion rollback | Precondition: current code allowed `CHARLIE` (unregistered) to post; registered `BOB` claimed task; reputation is otherwise linked correctly; `CHARLIE` calls `CompleteTask` | Reputation award to poster fails with `AgentNotFound`; whole tx reverts atomically; task remains `Claimed`; claimant receives no XP and no bounty | P0 |
| XCC-06 | Claim flow enforces badge-gated access using reputation query | Precondition: task requires badge(s); claimant registered in reputation with matching / non-matching badges | Claim succeeds only when reputation query returns all required badge types | P0 |
| XCC-07 | Wrong reputation contract configured in tasks breaks claims/completions safely | Instantiate tasks with bad `reputation_contract` address or a mock that does not expose expected query/execute interface | Claim fails as `AgentNotRegistered` or completion fails during outbound execute; no partial task-state corruption | P1 |
| XCC-08 | Atomicity: any failing outbound message reverts prior state write | Use mock / hostile reputation contract to force failure on 2nd/3rd outbound execute during `CompleteTask` | Despite task being saved as `Completed` before message dispatch, final committed state stays `Claimed`; no payout or partial counter updates persist | P0 |

---

## 4) Edge-case matrix

| ID | Description | Input / Setup | Expected outcome | Priority |
|---|---|---|---|---|
| EDGE-01 | Duplicate registration leaves agent count unchanged | Register `ALICE`, then repeat register | Second tx fails; `agent_count` does not increment twice | P1 |
| EDGE-02 | Awarding 0 XP is accepted but should be explicit in behavior | Precondition: `ALICE` registered; owner executes `AwardXp { amount: 0 }` | **Current code:** tx succeeds and XP remains same; log behavior for product decision | P2 |
| EDGE-03 | Posting task with `xp_reward=0` | Sender=`ALICE`; execute `PostTask { xp_reward: 0, ... }` | **Current code:** task posts successfully; completion later awards 0 XP to claimant and 20 XP to poster | P1 |
| EDGE-04 | Claim exactly at expiry boundary | Task expires at `H`; attempt claim at block `H` | Claim rejected because code uses `>= expires_at` | P0 |
| EDGE-05 | Expire exactly at expiry boundary | Task expires at `H`; call `ExpireTask` at block `H` | Expire succeeds because code only rejects when `current_height < expires_at` | P0 |
| EDGE-06 | Re-claim after expiry is impossible | Task claimed, then expired, then another user tries `ClaimTask` | Claim fails with `TaskNotOpen` because expired tasks never reopen | P1 |
| EDGE-07 | Removing issuer revokes future badge minting immediately | Add issuer, mint once, remove issuer, mint again | First mint succeeds; second fails with `NotIssuer` | P1 |
| EDGE-08 | Duplicate required badge entries on task do not bypass checks | Post task with `required_badges=["rust","rust"]`; claimant has / lacks `rust` | With badge: claim succeeds; without badge: claim fails. No weird double-count logic | P2 |
| EDGE-09 | Query limit cap works at upper bound | Populate >100 tasks / agents; query with `limit=500` | Response capped to 100 items | P2 |
| EDGE-10 | Bad address input is rejected cleanly | Use malformed address in `GetAgent`, `AddIssuer`, `SetTaskContract`, `MyPostedTasks`, etc. | Contract returns standard validation error, no state changes | P2 |

---

## 5) Security / hardening matrix

| ID | Description | Input / Setup | Expected outcome | Priority |
|---|---|---|---|---|
| SEC-01 | Unauthorized issuer operations are blocked | Non-owner attempts `AddIssuer` / `RemoveIssuer` / `SetTaskContract` | All fail with `Unauthorized`; issuer/task linkage state unchanged | P0 |
| SEC-02 | Unauthorized reputation mutation is blocked | Random actor calls `AwardXp`, `IncrementTasksCompleted`, `IncrementTasksPosted` directly on reputation | All fail with `Unauthorized` | P0 |
| SEC-03 | Self-claim prevention cannot be bypassed | Poster attempts to claim own task under normal flow and with badges/registration present | Always fails with `CannotClaimOwnTask` | P0 |
| SEC-04 | Reentrancy resistance on task completion | Instantiate tasks against a malicious reputation contract that re-enters `tasks` during outbound execute | Reentrant calls cannot exploit state because task is already non-open/non-claimable; final state should remain safe and atomic | P1 |
| SEC-05 | Reentrancy surface on reputation contract is effectively absent | Exercise `Register`, `MintBadge`, `AwardXp`, counter increments with malicious inputs | No external calls occur inside reputation executes, so no callback/reentrancy path exists | P2 |
| SEC-06 | XP arithmetic overflow on reputation award is handled or exposed | Set agent XP near `u64::MAX`; call `AwardXp` or `MintBadge` | Desired secure result: fail safely. **Current code risk:** plain `+=` may wrap in release builds; must be tested explicitly | P0 |
| SEC-07 | Counter overflow on `tasks_completed` / `tasks_posted` is handled or exposed | Set counters near `u64::MAX`; call increment executes from authorized tasks contract | Desired secure result: fail safely. **Current code risk:** possible wraparound | P0 |
| SEC-08 | `AGENT_COUNT` overflow is handled or exposed | Force `agent_count` to `u64::MAX` then register one more agent | Desired secure result: fail safely. **Current code risk:** `c + 1` may wrap | P1 |
| SEC-09 | `next_task_id` overflow is handled or exposed | Force `next_task_id=u64::MAX` then post task | Desired secure result: fail safely. **Current code risk:** `next_task_id += 1` may wrap and overwrite / reuse IDs | P0 |
| SEC-10 | `expires_at` addition overflow is handled or exposed | Use block height near `u64::MAX` and `expires_in_blocks=Some(large)` | Desired secure result: fail safely. **Current code risk:** `env.block.height + blocks` may wrap to a small value | P0 |
| SEC-11 | Bounty payout / refund stays atomic with reputation failures | Force reputation outbound call failure during `CompleteTask` on bounty-backed task | No bounty should be paid if XP side fails; tx should revert fully | P0 |
| SEC-12 | Query surfaces do not leak or mutate state | Spam all queries with valid/invalid params | Queries return data or errors only; no state mutation side effects | P2 |

---

## Recommended execution order

1. **P0 cross-contract and authorization tests first**: `REP-01/02/05-19/20/21`, `TASK-02/04/07-20`, `XCC-01..06/08`, `SEC-01..03/06/07/09/10/11`
2. **Then edge conditions** around expiry boundaries and malformed input.
3. **Then P2 query / regression coverage**.

## Highest-risk failures to watch for during testing

- `CompleteTask` reverting because reputation was never linked with `SetTaskContract`
- Unregistered poster successfully posting, then deadlocking completion later
- Silent `u64` wraparound on XP, counters, `next_task_id`, or `expires_at`
- Badge-gated claims incorrectly passing when one required badge is missing
- Partial state commits if one outbound reputation message fails during completion
