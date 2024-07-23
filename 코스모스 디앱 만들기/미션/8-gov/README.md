# Governance

거버넌스는 현재 진행중인 현재 진행중인 Proposal 목록, 그리고 투표하는 Msg를 생성 및 전송하는 것을 구현한다.

현재는 cosmoshub testnet에 진행중인 proposal이 없어서 예제 코드를 통해서만 알아본다.

## 예제

#### **`components/gov.tsx`**

```ts
"use client";

import { useChain } from "@cosmos-kit/react";
import { useEffect, useState } from "react";
import { MsgVoteEncodeObject } from "@cosmjs/stargate";
import { VoteOption } from "cosmjs-types/cosmos/gov/v1beta1/gov";

export default function Gov() {
  const { address, getRestEndpoint, getSigningStargateClient } =
    useChain("cosmoshubtestnet");

  const [proposals, setProposals] = useState<any>();

  const yes = async () => {
    if (!address) {
      return;
    }

    const msg: MsgVoteEncodeObject = {
      typeUrl: "/cosmos.gov.v1beta1.MsgVote",
      value: {
        proposalId: BigInt(1),
        voter: address,
        option: VoteOption.VOTE_OPTION_YES,
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
        `${await getRestEndpoint()}/cosmos/gov/v1beta1/proposals?proposal_status=PROPOSAL_STATUS_VOTING_PERIOD`
      );
      const result = await res.json();
      setProposals(result);
    };
    fetchProposals();
  }, [address]);

  return (
    <>
      <h3>Governance</h3>
      {JSON.stringify(proposals)}
      <button onClick={yes}>Yes</button>
    </>
  );
}
```
