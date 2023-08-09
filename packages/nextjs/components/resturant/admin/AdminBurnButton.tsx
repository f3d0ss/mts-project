import React from "react";
import { useContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { ResturantNft } from "~~/types/resturantNft";

type AdminBurnButtonProps = {
  nft: ResturantNft;
};

export default function AdminBurnButton({ nft }: AdminBurnButtonProps) {
  const writeTxn = useTransactor();

  const { writeAsync: burnNftNotSold } = useContractWrite({
    address: nft.resturant,
    abi: contracts.ResturantToken.abi,
    functionName: "burnNftNotSold",
    args: [nft.id],
  });

  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        !burnNftNotSold ? "loading" : ""
      }`}
      disabled={!burnNftNotSold}
      onClick={() => burnNftNotSold && writeTxn(() => burnNftNotSold())}
    >
      Burn
    </button>
  );
}
