import React from "react";
import { useContractWrite, usePrepareContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { ResturantNft } from "~~/types/resturantNft";

type ApproveNftButtonProps = {
  nft: ResturantNft;
};

export default function ApproveNftButton({ nft }: ApproveNftButtonProps) {
  const writeTxn = useTransactor();

  const approveArgs: [string, bigint] | undefined = nft.price ? [nft.resturant, nft.price] : undefined;

  const { config: approveConfig } = usePrepareContractWrite({
    address: nft.paymentToken,
    abi: contracts.ERC20.abi,
    functionName: "approve",
    args: approveArgs,
  });

  const { writeAsync: approveToken } = useContractWrite(approveConfig);

  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        !approveToken ? "loading" : ""
      }`}
      disabled={!approveToken}
      onClick={() => approveToken && writeTxn(() => approveToken())}
    >
      Approve Tokens
    </button>
  );
}
