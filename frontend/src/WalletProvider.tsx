import React from "react";
import {
  AbstraxionProvider,
  AbstraxionEmbed,
} from "@burnt-labs/abstraxion";
import { CHAIN_CONFIG } from "./config";

export default function WalletProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <AbstraxionProvider
      config={{
        rpcUrl: CHAIN_CONFIG.rpcEndpoint,
        restUrl: CHAIN_CONFIG.restEndpoint,
        contracts: [
          CHAIN_CONFIG.contracts.reputation,
          CHAIN_CONFIG.contracts.tasks,
        ],
      }}
    >
      {children}
      <AbstraxionEmbed />
    </AbstraxionProvider>
  );
}
