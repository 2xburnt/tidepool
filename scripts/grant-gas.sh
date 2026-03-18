#!/bin/bash
# Grant fee allowance to a new Tidepool agent
# Usage: ./grant-gas.sh <agent_address>

set -e

GRANTER="xion18hjhxkrmrp0gag3rgl7xh00y95vetnj9unf96x"
NODE="--node https://rpc.xion-testnet-2.burnt.com:443"
CHAIN="--chain-id xion-testnet-2"
FROM="--from tidepool-signer --keyring-backend test"
GAS="--gas auto --gas-adjustment 1.5 --gas-prices 0.025uxion"
SPEND_LIMIT="10000000uxion"  # 10 XION per agent
EXPIRATION="2027-01-01T00:00:00Z"

GRANTEE="${1:?Usage: $0 <agent_address>}"

echo "Granting fee allowance to $GRANTEE..."
echo "  Spend limit: $SPEND_LIMIT"
echo "  Allowed: MsgExecuteContract only"
echo "  Expiration: $EXPIRATION"

xiond tx feegrant grant \
  "$GRANTER" "$GRANTEE" \
  --spend-limit "$SPEND_LIMIT" \
  --allowed-messages "/cosmwasm.wasm.v1.MsgExecuteContract" \
  --expiration "$EXPIRATION" \
  $NODE $CHAIN $FROM $GAS -y

echo "Done. Agent can now use --fee-granter $GRANTER on Tidepool contract txs."
