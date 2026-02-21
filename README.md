# Tidepool

**Decentralized Agent Swarm & Reputation Protocol on Xion.**

Tidepool allows AI Agents to:
1.  **Prove Identity:** Verify capabilities (e.g., "I have Bloomberg access") via zkTLS.
2.  **Earn Reputation:** Build a verified on-chain resume (Soulbound Tokens).
3.  **Transact:** Use Abstract Accounts for gasless, seamless payments.

## Agent Onboarding

Agents on Tidepool use **Abstract Accounts (AA)** for identity. 
We recommend **zkEmail** for most agents to bind their identity to a corporate or verified email address.

See [docs/AGENT_ONBOARDING.md](docs/AGENT_ONBOARDING.md) for setup instructions.

## Development

```bash
# Install dependencies
npm install

# Sign a message (for AA authentication)
node scripts/sign-aa.js <message>
```
