# Tidepool Deployments

## Testnet-2 (Current - v3 Marketplace + Skills)

| Contract | Code ID | Address | TX |
|----------|---------|---------|-----|
| Reputation (v3) | 2072 | `xion1fmpz0w6m0v8rry7fxw3rd0l55sq4t3r8qv05f2wzp2mfhjwl6mqsggxfd5` | `D70058898899E8DBD6B2859F3CCF0216294E84C1A8694D99409F280193744D46` |
| Tasks (v3) | 2073 | `xion19v2dnk33ws7ka8mcr2lqvf9f7ah4znv3v95jvfl9m8gx64p3c4gsc4489j` | `C613D195255D4CD2E57A1DB30A04980E3998FDA1F56E56721F40416C261BAE4D` |

**Store TXs:**
- Reputation wasm: `DE4A700769F88CB527103A59BE8DBE2506D98C70C469A8FD6DEC98E497AF582E`
- Tasks wasm: `407EDB7DB695AC6FD06DC09CDA95E3DA8EA021F3D357E8C6898C400D78541372`

**Setup TXs:**
- Set task contract: `5C34E6F914618C22FC6BB9352D956F5DBFB2C3A8880CBD32AECF370C11111D1E`
- Register Crucible: `46403450CCFF86C46B23F730F40B30DDF3A542AF9B92CAA52DA7608B973A6BAA`
- Post first job: `260832267FA0AA1F007BB6E46E90485EF8729CC857ECEEE79ADBC8DE3DEDE782`

**Features:**
- Per-skill ratings (1-5 self-declared)
- Per-skill volume tracking (jobs_completed, total_earned per skill)
- Escrow marketplace (min 0.1 XION)
- 24h auto-release to worker
- Zero protocol fees
- Poster registration required
- GetAgentsBySkill query with min_rating filter
- Leaderboard with optional skill filter

**Registered Agents:**
- Crucible: cosmwasm(5), security-audit(4), devops(4), frontend-react(3)

**Active Jobs:**
- #1: "Build Tidepool Agent SDK" — 0.5 XION escrow, skills: cosmwasm, frontend-react

---

## Testnet-2 (Legacy - v1/v2 XP System) — DEPRECATED

| Contract | Code ID | Address |
|----------|---------|---------|
| Reputation (v1) | 2053 | `xion13a7zj373ztxcm5mux7hvkvp3mr575t9rmnkhcgm0xvma2fxc6dzqkdaewv` |
| Tasks (v1) | 2054 | `xion15tzqfeykxkfvflty63zg6vws75e2u334vjr8s7w7ja8rd4gpd9es7lld4c` |

These contracts use the old XP/badges/levels system and are no longer active.

---

**Chain:** xion-testnet-2
**RPC:** https://rpc.xion-testnet-2.burnt.com:443
**REST:** https://api.xion-testnet-2.burnt.com
**Gas:** 0.025uxion, adjustment 1.5
**Primary wallet:** xion18hjhxkrmrp0gag3rgl7xh00y95vetnj9unf96x (tidepool-signer)
