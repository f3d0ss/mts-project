import React from "react";
import { useContractRead } from "wagmi";
import { Spinner } from "~~/components/Spinner";
import contracts from "~~/generated/usefulAbis";
import { ResturantNft } from "~~/types/resturantNft";
import { getDateFromTimestamp } from "~~/utils/scaffold-eth/getDate";

type AdminNftButtonProps = {
  nft: ResturantNft;
};
export default function AdminNftButton({ nft }: AdminNftButtonProps) {
  const { data: owner } = useContractRead({
    address: nft.resturant,
    abi: contracts.ResturantToken.abi,
    functionName: "ownerOf",
    args: [nft.id],
  });

  if (!owner) {
    return <Spinner />;
  }

  if (owner === nft.resturant) {
    return <button>Burn</button>;
  }

  if (getDateFromTimestamp(nft.reservationDate).getTime() < Date.now()) {
    return <button>NotConsumed</button>;
  }

  return <button>Refund</button>;
}
