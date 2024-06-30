"use client";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { wallets } from "@cosmos-kit/cosmostation";
import assets from "chain-registry/assets";
import { chains } from "chain-registry";
import { ChainProvider } from "@cosmos-kit/react";
import "@interchain-ui/react/styles";
export default function Providers({ children }: { children: React.ReactNode }) {
  const queryClient = new QueryClient();
  return (
    <>
      <ChainProvider chains={chains} assetLists={assets} wallets={wallets}>
        <QueryClientProvider client={queryClient}>
          {children}
        </QueryClientProvider>
      </ChainProvider>
    </>
  );
}
