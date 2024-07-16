import Balance from "@/components/balance";
import Gov from "@/components/gov";
import IbcSend from "@/components/ibc-send";
import Send from "@/components/send";
import Staking from "@/components/staking";
import Wallet from "@/components/wallet";

export default function Home() {
  return (
    <main>
      <div className="mt-10 grid place-items-center">
        <Wallet />
        <Balance />
        <Send />
        <IbcSend />
        <Staking />
        <Gov />
      </div>
    </main>
  );
}
