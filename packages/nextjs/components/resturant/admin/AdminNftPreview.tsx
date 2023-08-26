import React from "react";
import NftPreview from "../NftPreview";
import AdminNftButton from "./AdminNftButton";
import { ResturantNft } from "~~/types/resturantNft";

type AdminNftPreviewProps = {
  nft: ResturantNft;
  showOwner?: boolean;
};

export default function AdminNftPreview({ nft, showOwner }: AdminNftPreviewProps) {
  return (
    <div>
      <NftPreview
        nft={nft}
        showOwner={showOwner}
        buttons={nft => [<AdminNftButton nft={nft} key={nft.id.toString()} />]}
      />
    </div>
  );
}
