# Balance

Cosmos-SDK를 이용한 앱체인들의 State 조회를 하는 여러 방법들을 활용하여 Account의 Token Balance를 조회한다.

## Endpoint

데이터를 조회하기 위해선 체인의 RPC나 REST API와 같은 엔드포인트를 호출해야 한다.

실제 노드를 운영하지 않는 프로젝트에선, Public Endpoint를 찾거나, API 서비스를 찾아 사용하게 된다. Production 레벨에서는 API 서비스 이용 또는 직접 노드 운영해야 하지만, 미션을 위해서는 cosmos-kit의 훅을 통해 제공되는 public endpoint를 임시로 사용한다.

```ts
import { useChain } from "@cosmos-kit/react";
...

const { getRestEndpoint, getRpcEndpoint } =
    useChain("cosmoshubtestnet");
```

https://github.com/cosmology-tech/chain-registry/tree/main/v2/chain-registry

위 repository에 등록된 정보를 호출해가며 정상 응답이 오는 endpoint를 찾아 활용하는 기능이다.

미션에서는 Rpc Endpoint를 사용하지 않고, cosmjs client에서 제공하는 함수를 사용하거나, REST API를 활용한 Endpoint를 사용하도록 한다.

## cosmjs를 통한 데이터 조회

cosmjs에서는 state 조회를 위해 client를 통해 다음과 같은 method를 제공한다.

`getChainId()`
`getHeight()`
`getAccount(searchAddress: string)`
`getSequence(address: string)`
`getBlock(height?: number)`
`getBalance(address: string, searchDenom: string)`
`getAllBalances(address: string)`
`getBalanceStaked(address: string)`
`getDelegation(delegatorAddress: string, validatorAddress: string)`
`getTx(id: string)`

다음 예제를 통해 cosmoshubtestnet 체인의 현재 지갑에 연결된 address의 모든 Balance를 조회하는 기능을 구현해본다.

연결된 지갑의 주소, client를 구하기 위해 `useChain` 훅을 사용한다.

#### **`import 및 hook`**

```ts
import { useChain } from "@cosmos-kit/react";

const { address, getStargateClient } = useChain("cosmoshubtestnet");
```

#### **`cosmjs clinet 객채 생성 및 balance 조회`**

```ts
const client = await getStargateClient();
const result = await client.getAllBalances(address);

console.log(result);
```

## REST API를 통한 데이터 조회

Cosmos-SDK는 RPC 통신 외에도 LCD라 불리는 REST API Endpoint도 제공할 수 있다. 노드 구동시 설정에서 제어할 수 있다.(Swagger 포함)

다음은 Cosmos 체인에서 제공하는 REST API Endpoint를 볼 수 있는 Swagger 이다.
https://cosmos-rest.publicnode.com/swagger/

다른 체인들의 Swagger들을 살펴보면 각 체인마다 Custom Module들에따라 지원되는 REST API 들을 확인할 수 있다.

#### **`import 및 hook`**

```ts
import { useChain } from "@cosmos-kit/react";

const { address, getRestEndpoint } = useChain("cosmoshubtestnet");
```

#### **`Balance 조회 REST API 호출`**

```ts
const balances = await fetch(
  `${await getRestEndpoint()}/cosmos/bank/v1beta1/balances/${address}`
);
const result = await balances.json();

console.log(result);
```

## 위 두 예제를 구현한 코드는 아래와 같다.

#### **`components/balance.tsx`**

```ts
"use client";

import { useChain } from "@cosmos-kit/react";
import { useEffect, useState } from "react";

export default function Balance() {
  const { address, getRestEndpoint, getStargateClient } =
    useChain("cosmoshubtestnet");

  const [restBalances, setRestBalances] = useState<any>();
  const [cosmjsBalances, setCosmjsBalances] = useState<any>();

  useEffect(() => {
    if (!address) {
      return;
    }

    const fetchRestBalance = async () => {
      const balances = await fetch(
        `${await getRestEndpoint()}/cosmos/bank/v1beta1/balances/${address}`
      );
      const result = await balances.json();
      setRestBalances(result.balances);
    };
    fetchRestBalance();

    const fetchCosmjsBalance = async () => {
      const client = await getStargateClient();
      const result = await client.getAllBalances(address);
      setCosmjsBalances(result);
    };
    fetchCosmjsBalance();
  }, [address]);

  return (
    <>
      <h3>Rest Balance</h3>
      {restBalances &&
        restBalances.map((balance: any) => (
          <div key={balance.denom}>
            {balance.amount} {balance.denom}
          </div>
        ))}
      <h3>Cosmjs Balance</h3>
      {cosmjsBalances &&
        cosmjsBalances.map((balance: any) => (
          <div key={balance.denom}>
            {balance.amount} {balance.denom}
          </div>
        ))}
    </>
  );
}
```

![m4-1](../../images/m4-1.png)

최초에는 위 Balance 부분이 노출되진 않는데, cosmostestnet faucet을 받아 balance가 노출되는 것을 확인한다.

Cosmos-SDK의 decimal은 수정 가능하지만 기본은 6자리이다.
(1000000uatom = 1ATOM)

## 참고 - RPC를 통한 데이터 조회

미션에서는 cosmjs와 REST API를 활용한 기능으로도 충분히 구현 가능하기 때문에, 설정이 어려운 RPC 데이터 조회는 생략한다. 관련 코드 및 예제를 보고 싶으면 아래 RPC Endpoint를 찾아주는 hook과 Repository를 참고한다.

```ts
import { useChain } from "@cosmos-kit/react";
...

const { getRpcEndpoint } =
    useChain("cosmoshubtestnet");
```

https://github.com/cosmology-tech/interchain
