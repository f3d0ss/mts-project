import React from "react";
import GenerateTicketButton from "./GenerateTicketButton";
import { ResturantNft } from "~~/types/resturantNft";

type OwnedNftButtonProps = {
  nft: ResturantNft;
};

export default function OwnedNftButton({ nft }: OwnedNftButtonProps) {
  if (!nft.locked) {
    return <GenerateTicketButton nft={nft} />;
  } else {
    return <p className="font-bold px-4">Used</p>;
  }
}
