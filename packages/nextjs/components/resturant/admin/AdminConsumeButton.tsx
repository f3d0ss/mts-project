import React from "react";
import { useContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { ResturantNft } from "~~/types/resturantNft";

type AdminConsumeButtonProps = {
  nft: ResturantNft;
};

export default function AdminConsumeButton({ nft }: AdminConsumeButtonProps) {
  const writeTxn = useTransactor();

  const { writeAsync: burnTicketNotConsumed } = useContractWrite({
    address: nft.resturant,
    abi: contracts.ResturantToken.abi,
    functionName: "burnTicketNotConsumed",
    args: [nft.id],
  });

  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        !burnTicketNotConsumed ? "loading" : ""
      }`}
      disabled={!burnTicketNotConsumed}
      onClick={() => burnTicketNotConsumed && writeTxn(() => burnTicketNotConsumed())}
    >
      Consume
    </button>
  );
}
