"use client";

import { useChain } from "@cosmos-kit/react";
import { Button } from "@interchain-ui/react";

export default function Wallet() {
  const { chain, status, username, openView } = useChain("cosmoshub");
  return (
    <Button onClick={openView}>
      {status === "Connected" ? (
        <>
          {chain.chain_name} {username}
        </>
      ) : (
        <>Connect Wallet</>
      )}
    </Button>
  );
}
