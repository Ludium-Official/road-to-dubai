# Send

Cosmos-SDK의 기본 모듈인 x/bank 모듈의 send 기능을 활용하여 Send 메시지를 만들고 사인 및 전송하는 법을 알아본다.

## 프로젝트 설정 수정

cosmos-kit의 ChainProvider 설정에 signingOptions를 추가한다. 해당 옵션은 아래와 같은 옵션들을 cosmjs client를 사용할 때 지정하기 위한 옵션이다.

```ts
readonly registry?: Registry;
readonly aminoTypes?: AminoTypes;
readonly broadcastTimeoutMs?: number;
readonly broadcastPollIntervalMs?: number;
readonly gasPrice?: GasPrice;
```

미션에서는 미션 진행에 지장이 없도록 gasPrice를 설정 후 gas 옵션을 "auto"로 사용하기 위해 해당 설정을 진행한다.

#### **`providers.tsx`**

```ts
"use client";
import React from "react";
import { wallets } from "@cosmos-kit/cosmostation";
import assets from "chain-registry/assets";
import { chains } from "chain-registry";
import { ChainProvider } from "@cosmos-kit/react";
import "@interchain-ui/react/styles";
import { SignerOptions } from "@cosmos-kit/core";
import { GasPrice } from "@cosmjs/stargate";
import { Chain } from "@chain-registry/types";
export default function Providers({ children }: { children: React.ReactNode }) {
  const signerOptions: SignerOptions = {
    signingStargate(chain) {
      if ((chain as Chain)?.chain_name === "cosmoshubtestnet") {
        return {
          gasPrice: GasPrice.fromString("0.025uatom"),
        };
      }
    },
  };
  return (
    <ChainProvider
      chains={chains}
      assetLists={assets}
      wallets={wallets}
      signerOptions={signerOptions}
    >
      {children}
    </ChainProvider>
  );
}
```

## cosmjs를 이용한 Send Transaction 전송

cosmjs에서는 send, sign, broadcast를 위해 client를 통해 다음과 같은 method를 제공한다.

미션에서는 아래 method중 sendTokens를 통해 토큰을 전송해본다.

`simulate`
`sign`
`broadcastTxSync`
`broadcastTx`
`signAndBroadcastSync`
`signAndBroadcast`
`sendTokens`
`delegateTokens`

#### **`import 및 hook`**

```ts
import { useChain } from "@cosmos-kit/react";

const { address, getSigningStargateClient } = useChain("cosmoshubtestnet");
```

`getSigningStargateClient`은 `getStargateClient`와 signing(tx sign, broadcast 등) 실제 state 변경을 위한 기능을 할 수 있는 차이를 가진다고 보면 된다.

#### **`cosmjs signing clinet 객채 생성 및 token send`**

```ts
const receiver = "cosmos1xxxxxxxx";
const balance = "10000";
const client = await getSigningStargateClient();
const res = await client.sendTokens(
  address,
  receiver,
  [{ amount: balance, denom: "uatom" }],
  "auto"
);

console.log(res);
```

## 위 예제를 구현한 코드는 아래와 같다.

#### **`components/send.tsx`**

```ts
"use client";

import { useChain } from "@cosmos-kit/react";
import { useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";

export default function Send() {
  const { address, getSigningStargateClient } = useChain("cosmoshubtestnet");
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
      <Input
        type="text"
        placeholder="Receiver address"
        value={receiver}
        onChange={(e) => setReceiver(e.target.value)}
      />
      <Input
        type="text"
        value={balance}
        placeholder="Amount"
        onChange={(e) => setBalance(e.target.value)}
      />
      <Button onClick={send}>Send</Button>
    </>
  );
}
```

전송 후 Mintscan에서 Tx가 잘 반영되었는지 확인해본다.
https://mintscan.io/cosmoshub-testnet/address/${address}
