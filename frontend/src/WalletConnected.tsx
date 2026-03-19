import React, { useEffect, useState, useCallback } from "react";
import {
  useAbstraxionAccount,
  useAbstraxionSigningClient,
  useAbstraxionClient,
} from "@burnt-labs/abstraxion";
import { type Agent, type Task, fetchLeaderboard, fetchTasks } from "./client";
import { CHAIN_CONFIG } from "./config";
import { TaskList } from "./App";

function shortenAddr(addr: string): string {
  return addr.slice(0, 10) + "..." + addr.slice(-6);
}

/**
 * Wallet-connected UI — lazy-loaded only when user clicks "Connect Wallet".
 * This module pulls in abstraxion + cosmjs via its imports.
 */
export default function WalletConnected() {
  const { data: account, isConnected } = useAbstraxionAccount();
  const { client: signingClient } = useAbstraxionSigningClient();
  const { client: abstraxionClient } = useAbstraxionClient();

  if (!isConnected || !account?.bech32Address) {
    return (
      <button
        className="btn-connect"
        onClick={() => abstraxionClient?.authenticate()}
      >
        Connect Wallet
      </button>
    );
  }

  return (
    <div className="wallet-connected">
      <span className="wallet-addr">
        {shortenAddr(account.bech32Address)}
      </span>
    </div>
  );
}
