"use client";

import { useChain } from "@cosmos-kit/react";
import { Coin } from "cosmjs-types/cosmos/base/v1beta1/coin";
import { useEffect, useState } from "react";
import {
  MsgDelegateEncodeObject,
  MsgUndelegateEncodeObject,
  MsgWithdrawDelegatorRewardEncodeObject,
} from "@cosmjs/stargate";

export default function Staking() {
  const { address, getRestEndpoint, getSigningStargateClient } =
    useChain("cosmoshubtestnet");

  const [balances, setBalances] = useState<Coin | null>();
  const [delegations, setDelegations] = useState<any>();
  const [rewards, setRewards] = useState<any>();
  const [amount, setAmount] = useState("");

  const delegate = async () => {
    if (!address) {
      return;
    }

    const msg: MsgDelegateEncodeObject = {
      typeUrl: "/cosmos.staking.v1beta1.MsgDelegate",
      value: {
        delegatorAddress: address,
        validatorAddress:
          "cosmosvaloper1mngvkkhm6g7nqxh4hcv8hjxvgax4m8xujzt964",
        amount: { amount: amount, denom: "uatom" },
      },
    };
    const client = await getSigningStargateClient();
    const res = await client.signAndBroadcast(address, [msg], "auto");
    console.log(res);
  };

  const undelegate = async () => {
    if (!address) {
      return;
    }

    const msg: MsgUndelegateEncodeObject = {
      typeUrl: "/cosmos.staking.v1beta1.MsgUndelegate",
      value: {
        delegatorAddress: address,
        validatorAddress:
          "cosmosvaloper1mngvkkhm6g7nqxh4hcv8hjxvgax4m8xujzt964",
        amount: { amount: amount, denom: "uatom" },
      },
    };
    const client = await getSigningStargateClient();
    const res = await client.signAndBroadcast(address, [msg], "auto");
    console.log(res);
  };

  const claim = async () => {
    if (!address) {
      return;
    }

    const msg: MsgWithdrawDelegatorRewardEncodeObject = {
      typeUrl: "/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward",
      value: {
        delegatorAddress: address,
        validatorAddress:
          "cosmosvaloper1mngvkkhm6g7nqxh4hcv8hjxvgax4m8xujzt964",
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

    const fetchBalance = async () => {
      const client = await getSigningStargateClient();
      const balances1 = await client.getBalanceStaked(address);
      setBalances(balances1);
    };

    fetchBalance();

    const fetchDelegations = async () => {
      const balances = await fetch(
        `${await getRestEndpoint()}/cosmos/staking/v1beta1/delegations/${address}`
      );
      const result = await balances.json();
      setDelegations(result);
    };
    fetchDelegations();

    const fetchRewards = async () => {
      const reward = await fetch(
        `${await getRestEndpoint()}/cosmos/distribution/v1beta1/delegators/${address}/rewards`
      );
      const result = await reward.json();
      setRewards(result);
    };
    fetchRewards();
  }, [address]);

  return (
    <>
      <h3>Staking</h3>
      {address}
      {balances?.amount} {balances?.denom}
      {JSON.stringify(delegations)}
      {JSON.stringify(rewards)}
      <input
        type="text"
        value={amount}
        placeholder="Amount"
        onChange={(e) => setAmount(e.target.value)}
      />
      <button onClick={delegate}>Delegate</button>
      <button onClick={undelegate}>Undelegate</button>
      <button onClick={claim}>Claim</button>
    </>
  );
}
