import React from "react";
import { useContractWrite, usePrepareContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { ResturantNft } from "~~/types/resturantNft";

type BuyNftButtonProps = {
  nft: ResturantNft;
};

export default function BuyNftButton({ nft }: BuyNftButtonProps) {
  const { config: buyConfig } = usePrepareContractWrite({
    address: nft.resturant,
    abi: contracts.ResturantToken.abi,
    functionName: "buyNFT",
    args: [nft.id],
    value: 0n,
  });

  const { write: buyToken } = useContractWrite(buyConfig);

  console.log("Render those buttons biatch");

  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        !buyToken ? "loading" : ""
      }`}
      disabled={!buyToken}
      onClick={() => buyToken && buyToken()}
    >
      Buy Token
    </button>
  );
}
