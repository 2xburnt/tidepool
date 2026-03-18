# Tidepool Architecture — Hackathon MVP

## Overview
Tidepool is a Decentralized Agent Swarm & Reputation Protocol on Xion.  
Two core contracts: **Reputation** (identity + XP + badges) and **Tasks** (work marketplace).

---

## 1. Reputation Contract v2

### State

```rust
pub struct Config {
    pub owner: Addr,
    pub task_contract: Option<Addr>,  // authorized to award XP
}

pub struct Agent {
    pub name: String,
    pub level: u64,
    pub xp: u64,
    pub badges: Vec<Badge>,
    pub tasks_completed: u64,
    pub tasks_posted: u64,
    pub registered_at: u64,  // block height
}

pub struct Badge {
    pub badge_type: String,     // e.g. "skill:rust", "achievement:first_task", "verified:email"
    pub issuer: Addr,
    pub issued_at: u64,         // block height
    pub proof: Option<String>,  // zkTLS proof hash (future)
}

// Storage
pub const CONFIG: Item<Config> = Item::new("config");
pub const AGENTS: Map<&Addr, Agent> = Map::new("agents");
pub const ISSUERS: Map<&Addr, bool> = Map::new("issuers");
pub const AGENT_COUNT: Item<u64> = Item::new("agent_count");
```

### XP & Leveling

| Action | XP |
|---|---|
| Register | 10 |
| Complete a task | 50 |
| Post a task (completed by someone) | 20 |
| Earn a badge | 25 |
| Reach new level | bonus 10 × level |

| Level | XP Required (cumulative) |
|---|---|
| 1 | 0 |
| 2 | 100 |
| 3 | 250 |
| 4 | 500 |
| 5 | 1000 |
| 6 | 2000 |
| 7 | 3500 |
| 8 | 5500 |
| 9 | 8000 |
| 10 | 12000 |

### Messages

```rust
#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Register { name: String },
    MintBadge { agent: String, badge_type: String, proof: Option<String> },
    AddIssuer { address: String },
    RemoveIssuer { address: String },
    SetTaskContract { address: String },
    // Called by task contract only:
    AwardXp { agent: String, amount: u64, reason: String },
    IncrementTasksCompleted { agent: String },
    IncrementTasksPosted { agent: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(AgentResponse)]
    GetAgent { address: String },
    #[returns(AgentsListResponse)]
    ListAgents { start_after: Option<String>, limit: Option<u32> },
    #[returns(LeaderboardResponse)]
    Leaderboard { limit: Option<u32> },
    #[returns(ConfigResponse)]
    Config {},
    #[returns(bool)]
    IsIssuer { address: String },
}
```

### Query Responses

```rust
pub struct AgentResponse {
    pub address: Addr,
    pub name: String,
    pub level: u64,
    pub xp: u64,
    pub badges: Vec<Badge>,
    pub tasks_completed: u64,
    pub tasks_posted: u64,
    pub registered_at: u64,
}

pub struct AgentsListResponse {
    pub agents: Vec<AgentResponse>,
}

pub struct LeaderboardResponse {
    pub agents: Vec<AgentResponse>,  // sorted by XP desc
}

pub struct ConfigResponse {
    pub owner: Addr,
    pub task_contract: Option<Addr>,
    pub agent_count: u64,
}
```

---

## 2. Task Contract (NEW)

### State

```rust
pub struct TaskConfig {
    pub owner: Addr,
    pub reputation_contract: Addr,
    pub next_task_id: u64,
}

pub struct Task {
    pub id: u64,
    pub poster: Addr,
    pub title: String,
    pub description: String,
    pub xp_reward: u64,
    pub required_badges: Vec<String>,  // badge_types required to claim
    pub status: TaskStatus,
    pub claimant: Option<Addr>,
    pub created_at: u64,    // block height
    pub claimed_at: Option<u64>,
    pub completed_at: Option<u64>,
    pub expires_at: Option<u64>,  // block height
}

#[cw_serde]
pub enum TaskStatus {
    Open,
    Claimed,
    Completed,
    Expired,
}

// Storage
pub const TASK_CONFIG: Item<TaskConfig> = Item::new("task_config");
pub const TASKS: Map<u64, Task> = Map::new("tasks");
pub const POSTER_TASKS: Map<(&Addr, u64), bool> = Map::new("poster_tasks");    // index
pub const CLAIMANT_TASKS: Map<(&Addr, u64), bool> = Map::new("claimant_tasks"); // index
```

### Messages

```rust
#[cw_serde]
pub struct InstantiateMsg {
    pub reputation_contract: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    PostTask {
        title: String,
        description: String,
        xp_reward: u64,
        required_badges: Vec<String>,
        expires_in_blocks: Option<u64>,
    },
    ClaimTask { task_id: u64 },
    CompleteTask { task_id: u64 },  // only poster can confirm
    ExpireTask { task_id: u64 },    // anyone can expire if past deadline
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(TaskResponse)]
    GetTask { task_id: u64 },
    #[returns(TasksListResponse)]
    ListTasks { status: Option<TaskStatus>, start_after: Option<u64>, limit: Option<u32> },
    #[returns(TasksListResponse)]
    MyPostedTasks { address: String },
    #[returns(TasksListResponse)]
    MyClaimedTasks { address: String },
    #[returns(TaskConfigResponse)]
    Config {},
}
```

### Task Flow
1. **PostTask** → creates task with status=Open, poster must be registered agent
2. **ClaimTask** → agent claims (must be registered, meet badge requirements), status=Claimed
3. **CompleteTask** → poster confirms, XP awarded via cross-contract call to reputation, status=Completed
4. **ExpireTask** → if past deadline, anyone can expire, status reverts claimant

### Cross-Contract Integration
When a task is completed, the Task contract sends a `WasmMsg::Execute` to the Reputation contract:
- `AwardXp { agent: claimant, amount: xp_reward, reason: "task_completion" }`
- `IncrementTasksCompleted { agent: claimant }`
- `AwardXp { agent: poster, amount: 20, reason: "task_posted_completed" }`
- `IncrementTasksPosted { agent: poster }`

---

## 3. Directory Structure

```
tidepool/
├── contracts/
│   ├── reputation/        # Reputation contract v2
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── contract.rs
│   │       ├── lib.rs
│   │       ├── msg.rs
│   │       ├── state.rs
│   │       └── error.rs    # NEW: custom errors
│   └── tasks/             # Task marketplace contract
│       ├── Cargo.toml
│       └── src/
│           ├── contract.rs
│           ├── lib.rs
│           ├── msg.rs
│           ├── state.rs
│           └── error.rs
├── packages/
│   └── tidepool-types/    # Shared types between contracts
│       ├── Cargo.toml
│       └── src/lib.rs
├── scripts/
│   ├── sign-aa.js
│   ├── deploy.sh          # Build + deploy both contracts
│   └── demo.sh            # Demo script for hackathon
├── cli/                   # Node.js CLI
│   ├── package.json
│   └── src/
│       └── index.ts
├── frontend/              # React dashboard
│   └── ...
├── docs/
│   ├── ARCHITECTURE.md
│   └── AGENT_ONBOARDING.md
├── Cargo.toml             # Workspace
└── README.md
```

---

## 4. Deployment Plan

1. Build contracts: `cargo build --release --target wasm32-unknown-unknown`
2. Optimize: `cosmwasm/optimizer` or manual strip
3. Store code on testnet via AA account
4. Instantiate reputation contract
5. Instantiate task contract with reputation contract address
6. Set task contract address in reputation contract
7. Register Crucible as first agent
8. Mint initial badges
9. Post demo tasks

### Accounts
- **Deployer AA**: `xion12pdahwvlytx9yaetr6q63tx935ye89454q46q959vtdqp3qgmqysz25g48`
- **Signer**: `xion18hjhxkrmrp0gag3rgl7xh00y95vetnj9unf96x`
- **RPC**: `https://rpc.xion-testnet-2.burnt.com:443`
