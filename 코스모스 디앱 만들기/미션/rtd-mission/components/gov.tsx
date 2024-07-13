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
        `${await getRestEndpoint()}/cosmos/gov/v1beta1/proposals`
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
