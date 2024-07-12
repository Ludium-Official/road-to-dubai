"use client";

import { useChain } from "@cosmos-kit/react";
import { useState } from "react";

export default function Send() {
  const { address, getSigningStargateClient, estimateFee, getRestEndpoint } =
    useChain("cosmoshubtestnet");
  const [receiver, setReceiver] = useState("");
  const [balance, setBalance] = useState("");

  const send = async () => {
    if (!address) {
      return;
    }

    const client = await getSigningStargateClient();
    const res = await client.sendTokens(
      address,
      receiver,
      [{ amount: balance, denom: "uatom" }],
      "auto"
    );

    console.log(res);
  };

  return (
    <>
      <h3>Send</h3>
      <input
        type="text"
        placeholder="Receiver address"
        value={receiver}
        onChange={(e) => setReceiver(e.target.value)}
      />
      <input
        type="text"
        value={balance}
        placeholder="Amount"
        onChange={(e) => setBalance(e.target.value)}
      />
      <button onClick={send}>Send</button>
    </>
  );
}
