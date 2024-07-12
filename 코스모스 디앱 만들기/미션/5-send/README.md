# Token send

토큰 샌드를 통해 코스모스에서 메시지를 만들고 사인하는 법을 알아본다.

cosmjs에선 메시지를 만드는 기능
cosmos-kit에선 클라이언트를 만들어서 send, broadcast하는 기능

broadcast란 체인에 데이터를 전송하는 것을 말한다.

balance를 조회했었다.
이번에는 send transaction을 통해서 0.1을 다른 계정으로 전송한다.

전송하고 민트스캔에서 tx를 확인한다.
## 프로젝트

```ts
"use client";

import { useChain } from "@cosmos-kit/react";
import { useState } from "react";

export default function Send() {
  const { address, getSigningStargateClient, estimateFee, getRestEndpoint } =
    useChain("neutrontestnet");
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
      [{ amount: balance, denom: "untrn" }],
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

```