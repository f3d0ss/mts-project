import React, { useState } from "react";
import { useContractWrite, useNetwork } from "wagmi";
import { InputBase } from "~~/components/scaffold-eth";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { getTargetNetwork } from "~~/utils/scaffold-eth";

type AcceptUserNftProps = {
  resturantAddress: string;
};

export default function AcceptUserNft({ resturantAddress }: AcceptUserNftProps) {
  const [tokenId, setTokenId] = useState<bigint | string>("");
  const [signature, setSignature] = useState<`0x${string}` | undefined>();
  const [ticket, setTicket] = useState<string>("");
  const writeTx = useTransactor();

  const { chain } = useNetwork();
  const writeDisabled = !chain || chain?.id !== getTargetNetwork().id || !signature;

  const { writeAsync: writeUseTicket, isLoading } = useContractWrite({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "useTicket",
    args: signature ? [signature, BigInt(tokenId)] : undefined,
  });

  const decodeTicket = (jsonTicket: string) => {
    console.log({ jsonTicket });
    try {
      const ticket = JSON.parse(jsonTicket);
      setTokenId(ticket.tokenId);
      setSignature(ticket.signature);
    } catch {}
    setTicket(jsonTicket);
  };

  return (
    <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl lg:px-2 mb-6 space-y-1 py-4 ">
      <InputBase value={ticket} onChange={decodeTicket} name="Ticket" placeholder="Ticket" />

      <div className="flex justify-between gap-2">
        <div
          className={`flex ${
            writeDisabled &&
            "tooltip before:content-[attr(data-tip)] before:right-[-10px] before:left-auto before:transform-none"
          }`}
          data-tip={`${writeDisabled && "Wallet not connected or in the wrong network"}`}
        >
          <button
            className={`btn btn-secondary btn-sm ${isLoading ? "loading" : ""}`}
            disabled={writeDisabled}
            onClick={() => writeTx(() => writeUseTicket())}
          >
            Consume
          </button>
        </div>
      </div>
    </div>
  );
}
