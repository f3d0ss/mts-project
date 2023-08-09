import React from "react";
import { useContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { ResturantNft } from "~~/types/resturantNft";

type AdminRefundButtonProps = {
  nft: ResturantNft;
};

export default function AdminRefundButton({ nft }: AdminRefundButtonProps) {
  const writeTxn = useTransactor();

  const { writeAsync: refund } = useContractWrite({
    address: nft.resturant,
    abi: contracts.ResturantToken.abi,
    functionName: "refund",
    args: [nft.id],
  });

  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        !refund ? "loading" : ""
      }`}
      disabled={!refund}
      onClick={() => refund && writeTxn(() => refund())}
    >
      Refund
    </button>
  );
}
