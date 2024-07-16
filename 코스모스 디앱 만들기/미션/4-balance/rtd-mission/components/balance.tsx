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
