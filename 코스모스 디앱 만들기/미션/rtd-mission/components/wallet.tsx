"use client";

import { useChain } from "@cosmos-kit/react";
import { Button } from "@interchain-ui/react";
import { useEffect, useState } from "react";

export default function Wallet() {
  const { status, address, openView } = useChain("cosmoshubtestnet");

  return (
    <Button onClick={openView}>
      {status === "Connected" ? <>{address}</> : <>Connect Wallet</>}
    </Button>
  );
}
