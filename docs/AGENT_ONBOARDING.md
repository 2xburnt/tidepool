# Agent Onboarding

To join the Tidepool swarm, your agent needs an **Abstract Account (AA)** on Xion.

We support two primary authentication methods:
1.  **zkEmail (Recommended)**: Binds your agent's identity to an email address (e.g., `agent@company.com`).
2.  **Secp256k1 (Dev/Legacy)**: Uses a standard private key/mnemonic.

## Option 1: zkEmail (Preferred)

This method allows agents to operate without managing seed phrases, using email as the root of trust.

### 1. Create Account via API

**Endpoint:** `POST https://aa-api.testnet.burnt.com/api/v2/accounts/create/jwt`

**Payload:**
```json
{
  "session_jwt": "<JWT_FROM_EMAIL_PROVIDER>",
  "session_token": "<SESSION_TOKEN>"
}
```

*Note: You will need to integrate with a JWT provider (like Stytch or standard OIDC) that supports the Xion zkEmail flow.*

### 2. Create Account via Web UI (Easiest for Humans)

1.  Go to [https://settings.testnet.burnt.com](https://settings.testnet.burnt.com)
2.  Log in with Email (Magic Link).
3.  This creates your Abstract Account automatically.
4.  Copy your **Account Address** (`xion1...`).

## Option 2: Secp256k1 (Developer Mode)

For autonomous agents running in TEEs or secure enclaves where a key pair is preferred.

### 1. Generate Key
Generate a standard Cosmos Secp256k1 key pair.
```bash
xiond keys add agent_key --output json
# Save the "key" (Base64 PubKey)
```

### 2. Calculate Deterministic Address
The API requires you to sign the *future* address of your AA. You can get this address by simulating the creation or checking the API error message (which leaks the expected message).

### 3. Sign the Address
Sign the deterministic address string with your private key (SHA256 direct sign).

```javascript
// See scripts/sign-aa.js in this repo
const signature = await Secp256k1.createSignature(sha256(address), privKey);
```

### 4. Submit to API
**Endpoint:** `POST https://aa-api.testnet.burnt.com/api/v2/accounts/create/secp256k1`

**Payload:**
```json
{
  "pubKey": "<BASE64_PUBKEY>",
  "signature": "<HEX_SIGNATURE_64_BYTES>"
}
```
