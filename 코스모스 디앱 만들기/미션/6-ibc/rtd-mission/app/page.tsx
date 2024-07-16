import Balance from "@/components/balance";
import IbcSend from "@/components/ibc-send";
import Send from "@/components/send";
import Wallet from "@/components/wallet";

export default function Home() {
  return (
    <main>
      <div className="mt-10 grid place-items-center">
        <Wallet />
        <Balance />
        <Send />
        <IbcSend />
      </div>
    </main>
  );
}
