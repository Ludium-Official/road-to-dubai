# ibc
- ibc로 전송한다.
- 샌드처럼 확인한다.

코스모스 생태계는 인터체인 생태계로도 불린다.
앱체인간 IBC(Inter-Blockchain Communication)라는 기능을 통해 체인간 전송을 가능하도록 한다.






cosmjs에서는 sendToken delegateTokens 등 편리하게 이용할 수 있는 method들을 제공해준다.
ibc는 어떠어떠한 이유로 sendIbcTokens을 deprecated 했다. 아마 포트, pfm 등 복잡해 지는 ibc를 저 내용으로 커버할 수 없어서 직접 method를 만들도록 가이드 하는 것 같다.

이번 예제에서는 해당 직접 cosmjs-type에서 해당 메시지를 만들고 tx를 전송하도록 한다.

```js
"use client";

import { useChain } from "@cosmos-kit/react";
import { Msg } from "cosmjs-types/cosmos/gov/v1beta1/tx";
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

    const fee = await estimateFee([msg]);
    const client = await getSigningStargateClient();
    const res = await client.signAndBroadcast(
      address,
      [msg],
      "auto",
      "",
      BigInt(0)
    );

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
```

asset-list-example에서 useChainUtils
코드를 보면 ibc 관련 코드 정보를 알 수 있다.

채널. 포트. 소스. 데놈 등등 정보가 필요.

