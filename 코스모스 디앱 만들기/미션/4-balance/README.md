# Query

- Cosmos-SDK 기반의 체인은 각각의 앱 체인으로 각각의 클라이언트가 존재한다.
rpc endpoint로 쿼리를 날린다.
날린 쿼리를 통해 useEffect로 account의 밸런스를 조회한다.
faucet을 받은 후 balance가 변화하는 걸 본다

## gRPC를 통한 데이터 조회

## REST API를 통한 데이터 조회

사진 개발 세팅

지갑에 neutron testnet 추가
cosmos-kit 프로젝트에 세팅
https://raw.githubusercontent.com/cosmos/chain-registry/master
cosmos-kit은 위에 있는 registry에서 체인 정보를 가져온다. neutron-pion 테스트넷 있는 것 확인


https://github.com/cosmology-tech/chain-registry/tree/main/v2/chain-registry

여기서 보면 있다. 체인이 


Faucet을 받기 위해선
https://docs.neutron.org/neutron/faq/




Endpoint를 조회하기 위해서는 
https://docs.neutron.org/neutron/faq#where-is-the-block-explorer
https://github.com/cosmos/chain-registry/blob/master/testnets/neutrontestnet/chain.json


    "rpc": [
      {
        "address": "https://rpc-falcron.pion-1.ntrn.tech",
        "provider": "Neutron"
      },
      {
        "address": "https://neutron-testnet-rpc.polkachu.com/",
        "provider": "Polkachu"
      }
    ],
    "rest": [
      {
        "address": "https://rest-falcron.pion-1.ntrn.tech",
        "provider": "Neutron"
      },
      {
        "address": "https://api.pion.remedy.tm.p2p.org",
        "provider": "P2P.ORG"
      },
      {
        "address": "https://rest.baryon-sentry-01.rs-testnet.polypore.xyz",
        "provider": "Hypha"
      }
    ],
    "grpc": [
      {
        "address": "grpc-falcron.pion-1.ntrn.tech:80",
        "provider": "Neutron"
      },
      {
        "address": "grpc.baryon.remedy.tm.p2p.org:443",
        "provider": "P2P.ORG"
      }
    ]



https://docs.cosmos.network/v0.50/learn/advanced/grpc_rest

데이터를 조회하기 위해선 체인의 gRPC, REST, RPC와 같은 엔드포인트를 호출해야 한다.

이 예제에서는 REST를 Endpoint를 호출한다.

RPC 예제를 보기 위해선 cosmos-kit의 예제들을 확인해 보도록 한다.


cosmos-kit의 useChain 훅 에서는
import { useChain } from "@cosmos-kit/react";

아래와 같은 편리한 기능들을 제공한다. REST Endpoint를 찾는 것 등

실제 노드를 운영하지 않는 프로젝트에선, 서버 접근 권한에 따라서 호출을 실패하기도 하는데, cosmos-kit의 훅을 통해 쉽게 찾아서 예제를 만들 수 있다.

  const { status, address, openView, getRestEndpoint } =
    useChain("neutrontestnet");

await getRestEndpoint() 

https://rest-falcron.pion-1.ntrn.tech/cosmos/bank/v1beta1/balances/neutron189xkhl4ywmvuv6j28952985k7aljlam5cgt7mt


```
"use client";

import { useChain } from "@cosmos-kit/react";
import { useEffect, useState } from "react";

export default function Balance() {
  const { address, getRestEndpoint } = useChain("neutrontestnet");

  const [balances, setBalances] = useState<any>();

  useEffect(() => {
    if (!address) {
      return;
    }
    const fetchBalance = async () => {
      const balances = await fetch(
        `${await getRestEndpoint()}/cosmos/bank/v1beta1/balances/${address}`
      );
      const result = await balances.json();
      setBalances(result.balances);
    };
    fetchBalance();
  }, [address]);

  return (
    <>
    <h3>Balance</h3>
      {balances &&
        balances.map((balance: any) => (
          <>
            {balance.amount} {balance.denom}
          </>
        ))}
    </>
  );
}

```
