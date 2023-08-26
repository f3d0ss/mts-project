import React from "react";
import AdminBurnButton from "./AdminBurnButton";
import AdminConsumeButton from "./AdminConsumeButton";
import AdminRefundButton from "./AdminRefundButton";
import { useContractRead } from "wagmi";
import { Spinner } from "~~/components/Spinner";
import contracts from "~~/generated/usefulAbis";
import { ResturantNft } from "~~/types/resturantNft";
import { EXPIRATION_RANGE } from "~~/utils/constats";
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
    return <AdminBurnButton nft={nft} />;
  }

  if (getDateFromTimestamp(nft.reservationDate).getTime() + EXPIRATION_RANGE < Date.now()) {
    return <AdminConsumeButton nft={nft} />;
  }

  return <AdminRefundButton nft={nft} />;
}
