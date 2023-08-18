import React, { ReactNode } from "react";
import { Spinner } from "../Spinner";
import NftPrice from "./NftPrice";
import ResturantName from "./ResturantName";
import { ResturantNft } from "~~/types/resturantNft";
import { getDateFromTimestamp } from "~~/utils/scaffold-eth/getDate";

type NftPreviewProps = {
  nft: ResturantNft;
  buttons: (nft: ResturantNft) => ReactNode[];
};

export default function NftPreview({ nft, buttons }: NftPreviewProps) {
  return (
    <div className="col-span-1 bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl aspect-w-13">
      <div className="w-full aspect-square bg-base-300"> IMAGE </div>
      <div className="p-3">
        <div className="py-2">
          <ResturantName resturantAddress={nft.resturant} />
        </div>
        {/* ONLY FOR DEBUG */}
        <div className="py-2">
          <span className="font-bold">Id</span>: {nft ? nft.id.toString() : <Spinner />}
        </div>
        <div className="py-2">
          <span className="font-bold">Owner</span>: {nft ? nft.owner : <Spinner />}
        </div>
        {/* ONLY FOR DEBUG */}

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
          <span className="font-bold">Description</span>: TO GET FROM URI bla bla bla bla bla bla bla bla blabla bla bla
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
