import Balance from "@/components/balance";
import Wallet from "@/components/wallet";

export default function Home() {
  return (
    <main>
      <div className="mt-10 grid place-items-center">
        <Wallet />
        <Balance />
      </div>
    </main>
  );
}
