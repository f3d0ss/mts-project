import React, { ReactNode, useEffect, useState } from "react";
import Image from "next/image";
import { Spinner } from "../Spinner";
import { Address } from "../scaffold-eth";
import NftPrice from "./NftPrice";
import ResturantName from "./ResturantName";
import { useIPFSGateway } from "~~/hooks/useIPFSGateway";
import { NftMetadata } from "~~/types/nftMetadata";
import { ResturantNft } from "~~/types/resturantNft";
import { getDateFromTimestamp } from "~~/utils/scaffold-eth/getDate";

type NftPreviewProps = {
  nft: ResturantNft;
  buttons: (nft: ResturantNft) => ReactNode[];
  showOwner?: boolean;
};

export default function NftPreview({ nft, buttons, showOwner }: NftPreviewProps) {
  const { isOnline, isConnecting, getFile } = useIPFSGateway();
  const [metadata, setMetadata] = useState<NftMetadata | undefined>();
  const [image, setImage] = useState<string | undefined>();

  useEffect(() => {
    const init = async () => {
      if (metadata) return;
      const jsonFile = await getFile(nft.uri);
      if (!jsonFile) return;
      const rawJson = Buffer.from(jsonFile).toString("utf8");
      const json: NftMetadata = JSON.parse(rawJson);
      const image = await getFile(json.image);
      if (!image) return;
      const base64Image = Buffer.from(image).toString("base64");
      const imageUrl = `data:image/jpeg;base64,${base64Image}`;
      setMetadata(json);
      setImage(imageUrl);
    };
    init();
  }, [metadata, nft.uri, isOnline, getFile]);

  return (
    <div className="col-span-1 bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl rounded-t-md aspect-w-13">
      <div className="w-full aspect-square bg-base-300 rounded-t-md">
        {image ? (
          <Image alt="Image" src={image} width="0" height="0" sizes="100vw" className="w-full h-auto rounded-t-md" />
        ) : isConnecting ? (
          "Loading..."
        ) : (
          "Error connecting to IPFS"
        )}
      </div>
      <div className="p-3">
        <div className="py-2">
          <ResturantName resturantAddress={nft.resturant} />
        </div>

        <div className="py-2 hidden">
          <span className="font-bold">Id</span>: {nft ? nft.id.toString() : <Spinner />}
        </div>
        <div className={`py-2 ${!showOwner && "hidden"}`}>
          <span className="font-bold">Owner</span>: {nft ? <Address address={nft.owner} /> : <Spinner />}
        </div>

        <div className="py-2">
          <span className="font-bold">Date</span>:{" "}
          {nft ? (
            getDateFromTimestamp(nft.reservationDate).toLocaleDateString(undefined, {
              weekday: "long",
              day: "2-digit",
              month: "long",
              hour: "2-digit",
              minute: "2-digit",
              hour12: false,
            })
          ) : (
            <Spinner />
          )}
        </div>
        <div className="py-2">
          <span className="font-bold">Description</span>:{" "}
          {metadata ? metadata.description : isConnecting ? <Spinner /> : "Error connecting to IPFS"}
        </div>
        <div className="flex justify-between items-center py-2">
          {nft && (
            <>
              <NftPrice tokenAddress={nft.paymentToken} price={nft.price} />
              {buttons(nft)}
            </>
          )}
        </div>
      </div>
    </div>
  );
}
