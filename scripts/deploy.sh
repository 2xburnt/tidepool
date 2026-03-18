#!/bin/bash
# Tidepool Deploy Script
# Deploys both contracts to Xion Testnet-2

set -e

RPC="https://rpc.xion-testnet-2.burnt.com:443"
CHAIN_ID="xion-testnet-2"
DEPLOYER="xion12pdahwvlytx9yaetr6q63tx935ye89454q46q959vtdqp3qgmqysz25g48"
SIGNER="xion18hjhxkrmrp0gag3rgl7xh00y95vetnj9unf96x"
GAS_PRICES="0.025uxion"

ARTIFACTS_DIR="$(dirname "$0")/../artifacts"

echo "🌊 Tidepool Deploy"
echo "==================="
echo "RPC:      $RPC"
echo "Chain:    $CHAIN_ID"
echo "Deployer: $DEPLOYER"
echo ""

# Check for xiond
if ! command -v xiond &> /dev/null; then
    echo "❌ xiond not found. Install the Xion CLI first."
    exit 1
fi

# Check wasm files exist
if [ ! -f "$ARTIFACTS_DIR/tidepool_reputation.wasm" ] || [ ! -f "$ARTIFACTS_DIR/tidepool_tasks.wasm" ]; then
    echo "❌ Wasm artifacts not found. Run 'cargo build --release --target wasm32-unknown-unknown' first."
    exit 1
fi

echo "📦 Storing reputation contract..."
REP_TX=$(xiond tx wasm store "$ARTIFACTS_DIR/tidepool_reputation.wasm" \
    --from "$SIGNER" \
    --chain-id "$CHAIN_ID" \
    --node "$RPC" \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices "$GAS_PRICES" \
    --output json \
    -y 2>&1)
echo "$REP_TX"
REP_CODE_ID=$(echo "$REP_TX" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo "✅ Reputation Code ID: $REP_CODE_ID"

echo ""
echo "📦 Storing tasks contract..."
TASK_TX=$(xiond tx wasm store "$ARTIFACTS_DIR/tidepool_tasks.wasm" \
    --from "$SIGNER" \
    --chain-id "$CHAIN_ID" \
    --node "$RPC" \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices "$GAS_PRICES" \
    --output json \
    -y 2>&1)
echo "$TASK_TX"
TASK_CODE_ID=$(echo "$TASK_TX" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
echo "✅ Tasks Code ID: $TASK_CODE_ID"

echo ""
echo "🔧 Instantiating reputation contract..."
REP_INIT=$(xiond tx wasm instantiate "$REP_CODE_ID" '{}' \
    --from "$SIGNER" \
    --label "tidepool-reputation-v1" \
    --admin "$DEPLOYER" \
    --chain-id "$CHAIN_ID" \
    --node "$RPC" \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices "$GAS_PRICES" \
    --output json \
    -y 2>&1)
echo "$REP_INIT"
REP_ADDR=$(echo "$REP_INIT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
echo "✅ Reputation Contract: $REP_ADDR"

echo ""
echo "🔧 Instantiating tasks contract..."
TASK_INIT_MSG="{\"reputation_contract\":\"$REP_ADDR\"}"
TASK_INIT=$(xiond tx wasm instantiate "$TASK_CODE_ID" "$TASK_INIT_MSG" \
    --from "$SIGNER" \
    --label "tidepool-tasks-v1" \
    --admin "$DEPLOYER" \
    --chain-id "$CHAIN_ID" \
    --node "$RPC" \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices "$GAS_PRICES" \
    --output json \
    -y 2>&1)
echo "$TASK_INIT"
TASK_ADDR=$(echo "$TASK_INIT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
echo "✅ Tasks Contract: $TASK_ADDR"

echo ""
echo "🔗 Linking task contract to reputation contract..."
LINK_MSG="{\"set_task_contract\":{\"address\":\"$TASK_ADDR\"}}"
xiond tx wasm execute "$REP_ADDR" "$LINK_MSG" \
    --from "$SIGNER" \
    --chain-id "$CHAIN_ID" \
    --node "$RPC" \
    --gas auto \
    --gas-adjustment 1.3 \
    --gas-prices "$GAS_PRICES" \
    --output json \
    -y

echo ""
echo "🌊 Tidepool Deployment Complete!"
echo "================================"
echo "Reputation Contract: $REP_ADDR"
echo "Tasks Contract:      $TASK_ADDR"
echo "Reputation Code ID:  $REP_CODE_ID"
echo "Tasks Code ID:       $TASK_CODE_ID"

# Save deployment info
cat > "$(dirname "$0")/../deployment.json" << EOF
{
  "chain_id": "$CHAIN_ID",
  "rpc": "$RPC",
  "deployer": "$DEPLOYER",
  "reputation": {
    "code_id": $REP_CODE_ID,
    "address": "$REP_ADDR"
  },
  "tasks": {
    "code_id": $TASK_CODE_ID,
    "address": "$TASK_ADDR"
  },
  "deployed_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF
echo ""
echo "📄 Saved to deployment.json"
