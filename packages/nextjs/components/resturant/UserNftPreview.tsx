import React from "react";
import NftPreview from "./NftPreview";
import UserNftButton from "./UserNftButton";
import { ResturantNft } from "~~/types/resturantNft";

type UserNftPreviewProps = {
  nft: ResturantNft;
};

export default function UserNftPreview({ nft }: UserNftPreviewProps) {
  return <NftPreview nft={nft} buttons={nft => [<UserNftButton key={nft.id.toString()} nft={nft} />]} />;
}
