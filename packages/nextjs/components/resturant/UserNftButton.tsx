import React from "react";
import ApproveNftButton from "./ApproveNftButton";
import BuyNftButton from "./BuyNftButton";
import { useAccount, useBalance, useContractRead } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { ResturantNft } from "~~/types/resturantNft";

type UserNftButtonProps = {
  nft: ResturantNft;
};

export default function UserNftButton({ nft }: UserNftButtonProps) {
  const account = useAccount();
  const allowanceArgs: [string, string] | undefined = account.address ? [account.address, nft.resturant] : undefined;
  const { data: allawance } = useContractRead({
    address: nft.paymentToken,
    abi: contracts.ERC20.abi,
    functionName: "allowance",
    args: allowanceArgs,
    watch: true,
  });

  const { address: accountAddress } = useAccount();

  const { data: balance, isLoading } = useBalance({
    token: nft.paymentToken,
    address: accountAddress,
    watch: true,
  });

  let hasEnoughFund = false;
  if (balance && balance.value >= nft.price) {
    hasEnoughFund = true;
  }

  let needAllowance = false;
  if (allawance !== undefined && allawance < nft.price) {
    needAllowance = true;
  }

  console.log("Render those buttons biatch");

  if (!hasEnoughFund) {
    return (
      <button
        className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
          isLoading ? "loading" : ""
        }`}
        disabled
      >
        Not Enough Funds
      </button>
    );
  }

  if (needAllowance) {
    return <ApproveNftButton nft={nft} />;
  }

  return <BuyNftButton nft={nft} />;
}
