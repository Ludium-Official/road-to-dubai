"use client";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { wallets } from "@cosmos-kit/cosmostation";
import assets from "chain-registry/assets";
import { chains } from "chain-registry";
import { ChainProvider } from "@cosmos-kit/react";
import "@interchain-ui/react/styles";
import { SignerOptions } from "@cosmos-kit/core";
import { GasPrice } from "@cosmjs/stargate";
export default function Providers({ children }: { children: React.ReactNode }) {
  const queryClient = new QueryClient();
  const signerOptions: SignerOptions = {
    signingStargate(chain) {
      return {
        gasPrice: GasPrice.fromString("0.025uatom"),
        gasLimits: { upload: 1500000 },
      };
    },
  };
  return (
    <>
      <ChainProvider
        chains={chains}
        assetLists={assets}
        wallets={wallets}
        signerOptions={signerOptions}
        walletConnectOptions={{
          signClient: {
            projectId: "a8510432ebb71e6948cfd6cde54b70f7",
            relayUrl: "wss://relay.walletconnect.org",
            metadata: {
              name: "Cosmos Kit dApp",
              description: "Cosmos Kit dApp built by Create Cosmos App",
              url: "https://docs.cosmology.zone/cosmos-kit/",
              icons: [],
            },
          },
        }}
      >
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      </ChainProvider>
    </>
  );
}
