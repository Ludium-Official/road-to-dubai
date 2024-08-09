# Governance

거버넌스는 현재 진행중인 현재 진행중인 Proposal 목록, 그리고 투표하는 Msg를 생성 및 전송하는 것을 구현한다.

현재는 cosmoshub testnet에 진행중인 proposal이 없어서 예제 코드를 통해서만 알아본다.

## 구현

### Proposal 목록 조회(REST API)

최근 3개의 제안을 조회한다. 파라미터를 통해 상태별 등 조회 조건을 선택할 수 있다(swagger 참고)

```ts
const { getRestEndpoint } = useChain("cosmoshubtestnet");
const res = await fetch(
  `${await getRestEndpoint()}/cosmos/gov/v1/proposals?pagination.limit=3&pagination.reverse=true`
);
const result = await res.json();
```

### 투표 기능 구현

```ts
const msg: MsgVoteEncodeObject = {
      typeUrl: "/cosmos.gov.v1beta1.MsgVote",
      value: {
        proposalId: BigInt(id),
        voter: address,
        option: yes ? VoteOption.VOTE_OPTION_YES : VoteOption.VOTE_OPTION_NO,
      },
    };
const client = await getSigningStargateClient();
const res = await client.signAndBroadcast(address, [msg], "auto");
```

#### **`components/gov.tsx`**

```ts
"use client";

import { useChain } from "@cosmos-kit/react";
import { useEffect, useState } from "react";
import { MsgVoteEncodeObject } from "@cosmjs/stargate";
import { VoteOption } from "cosmjs-types/cosmos/gov/v1beta1/gov";
import { Button } from "./ui/button";
import { Badge } from "./ui/badge";

export default function Gov() {
  const { address, getRestEndpoint, getSigningStargateClient } =
    useChain("cosmoshubtestnet");

  const [proposals, setProposals] = useState<any>();

  const vote = async (id: string, yes: boolean) => {
    if (!address) {
      return;
    }

    const msg: MsgVoteEncodeObject = {
      typeUrl: "/cosmos.gov.v1beta1.MsgVote",
      value: {
        proposalId: BigInt(id),
        voter: address,
        option: yes ? VoteOption.VOTE_OPTION_YES : VoteOption.VOTE_OPTION_NO,
      },
    };
    const client = await getSigningStargateClient();
    const res = await client.signAndBroadcast(address, [msg], "auto");
    console.log(res);
  };

  useEffect(() => {
    if (!address) {
      return;
    }
    const fetchProposals = async () => {
      const res = await fetch(
        `${await getRestEndpoint()}/cosmos/gov/v1/proposals?pagination.limit=3&pagination.reverse=true`
      );
      const result = await res.json();
      setProposals(result.proposals);
    };
    fetchProposals();
  }, [address]);

  return (
    <div className="space-y-3">
      <h3 className="text-xl font-bold">Governance</h3>
      {proposals &&
        proposals.map((proposal: any) => {
          return (
            <div className="flex space-x-2 items-center" key={proposal.id}>
              <div className="w-full">
                <Badge>{proposal.status}</Badge> {proposal.id}. {proposal.title}
              </div>
              <Button
                onClick={() => vote(proposal.id, true)}
                disabled={proposal.status !== "PROPOSAL_STATUS_VOTING_PERIOD"}
              >
                Yes
              </Button>
              <Button
                onClick={() => vote(proposal.id, false)}
                disabled={proposal.status !== "PROPOSAL_STATUS_VOTING_PERIOD"}
              >
                No
              </Button>
            </div>
          );
        })}
    </div>
  );
}
```

#### **`app/pages.tsx`**

```ts
import Balance from "@/components/balance";
import Gov from "@/components/gov";
import IbcSend from "@/components/ibc-send";
import Send from "@/components/send";
import Staking from "@/components/staking";
import Wallet from "@/components/wallet";

export default function Home() {
  return (
    <main>
      <div className="m-10 grid gap-14 w-2/5 mx-auto">
        <h1 className="text-3xl font-bold">Cosmos dApp</h1>
        <Wallet />
        <Gov />
        <Staking />
        <IbcSend />
        <Send />
        <Balance />
      </div>
    </main>
  );
}
```

## 결과

최신 Proposal 3개와 투표를 할 수 있는 기능 구현이 완료되었다.

![m8-1](../../images/m8-1.png)