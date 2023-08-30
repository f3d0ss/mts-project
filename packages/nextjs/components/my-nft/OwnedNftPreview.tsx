import React from "react";
import NftPreview from "../resturant/NftPreview";
import OwnedNftButton from "./OwnedNftButton";
import { ResturantNft } from "~~/types/resturantNft";

type OwnedNftPreviewProps = {
  nft: ResturantNft;
};

export default function OwnedNftPreview({ nft }: OwnedNftPreviewProps) {
  return (
    <NftPreview nft={nft} editableReview buttons={nft => [<OwnedNftButton key={nft.id.toString()} nft={nft} />]} />
  );
}
