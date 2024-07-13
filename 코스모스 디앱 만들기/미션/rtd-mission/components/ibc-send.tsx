"use client";

import { useChain } from "@cosmos-kit/react";
import { MsgTransferEncodeObject } from "@cosmjs/stargate";
import { useState } from "react";

export default function IbcSend() {
  const { address, getSigningStargateClient, estimateFee, getRestEndpoint } =
    useChain("cosmoshubtestnet");
  const [receiver, setReceiver] = useState(
    "elys1nzedewmlnlpwsmu2kh3g7mvhc02wp8k29k5y7q"
  );
  const [balance, setBalance] = useState("100");

  const send = async () => {
    if (!address) {
      return;
    }

    const msg: MsgTransferEncodeObject = {
      typeUrl: "/ibc.applications.transfer.v1.MsgTransfer",
      value: {
        sender: address,
        receiver: receiver,
        sourcePort: "transfer",
        sourceChannel: "channel-3302",
        token: { denom: "uatom", amount: balance },
        timeoutHeight: { revisionNumber: BigInt(3), revisionHeight: BigInt(0) },
        timeoutTimestamp: BigInt(0),
        memo: "",
      },
    };

    const client = await getSigningStargateClient();
    const res = await client.signAndBroadcast(address, [msg], "auto");

    console.log(res);
  };

  return (
    <>
      <h3>IBC Send</h3>
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
      <button onClick={send}>IBC Send</button>
    </>
  );
}
