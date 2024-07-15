"use client";

import { useChain } from "@cosmos-kit/react";
import { useEffect, useState } from "react";

export default function Balance() {
  const { address, getRestEndpoint, getSigningStargateClient } =
    useChain("cosmoshubtestnet");

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
      
      // via cosmjs
      // const client = await getSigningStargateClient();
      // const balances1 = await client.getAllBalances(address);
      // setBalances(balances1);
    };
    fetchBalance();
  }, [address]);

  return (
    <>
      <h3>Balance</h3>
      {address}
      {balances &&
        balances.map((balance: any) => (
          <div key={balance.denom}>
            {balance.amount} {balance.denom}
          </div>
        ))}
    </>
  );
}
