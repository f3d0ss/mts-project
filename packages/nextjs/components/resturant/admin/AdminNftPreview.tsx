import React from "react";
import NftPreview from "../NftPreview";
import AdminNftButton from "./AdminNftButton";
import { ResturantNft } from "~~/types/resturantNft";

type AdminNftPreviewProps = {
  nft: ResturantNft;
};

export default function AdminNftPreview({ nft }: AdminNftPreviewProps) {
  return (
    <div>
      <NftPreview nft={nft} buttons={nft => [<AdminNftButton nft={nft} key={nft.id.toString()} />]} />
    </div>
  );
}
