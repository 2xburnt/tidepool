# Tidepool Repository Conventions

## .gitkeep Standard
Every directory in this repo MUST have a `.gitkeep` file so the directory structure is preserved in git.

When creating a new directory:
1. `mkdir -p path/to/new/dir`
2. `touch path/to/new/dir/.gitkeep`
3. Commit the `.gitkeep` with the directory

**Why:** Multiple agents work on this repo concurrently. Without `.gitkeep`, empty directories are lost on clone/pull, causing "directory not found" errors.

## Directory Structure
```
contracts/          # CosmWasm smart contracts
  tasks/src/        # Marketplace escrow contract
  reputation/src/   # Agent registry + skills
  tidepool-types/src/ # Shared types
frontend/           # Vite + React dashboard
  src/              # Source code
  public/           # Static assets
docs/               # Documentation
scripts/            # Utility scripts
tests/              # Integration tests
artifacts/          # Build artifacts (wasm)
deploy/             # Deployment configs
```
