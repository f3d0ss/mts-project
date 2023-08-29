import React, { useState } from "react";
import GetUserNftTicket from "./GetUserNftTicket";
import { useContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { FetchedNft } from "~~/types/fetchedNft";
import { getDateFromTimestamp } from "~~/utils/scaffold-eth/getDate";

type NfTicket = FetchedNft & { id: bigint; signature: `0x${string}` };

type AcceptUserNftProps = {
  resturantAddress: string;
};

export default function AcceptUserNft({ resturantAddress }: AcceptUserNftProps) {
  const [nfTicket, setNfTicket] = useState<NfTicket | undefined>();
  const writeTx = useTransactor();

  const { writeAsync: writeUseTicket, isLoading } = useContractWrite({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "useTicket",
    args: nfTicket ? [nfTicket.signature, nfTicket.id] : undefined,
  });

  return (
    <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl lg:px-2 mb-6 space-y-1 py-4 ">
      {!nfTicket ? (
        <GetUserNftTicket
          resturantAddress={resturantAddress}
          onRead={(id, nft, signature) => {
            setNfTicket({ ...nft, id, signature });
          }}
        />
      ) : (
        <button className="btn btn-secondary btn-sm" onClick={() => setNfTicket(undefined)}>
          Retry Read
        </button>
      )}

      {nfTicket && (
        <>
          <div className="py-2">
            <span className="font-bold">Date</span>: (
            {getDateFromTimestamp(nfTicket.reservationDate).toLocaleDateString(undefined, {
              weekday: "long",
              day: "2-digit",
              month: "long",
              hour: "2-digit",
              minute: "2-digit",
              hour12: false,
            })}
            )
          </div>
          <div className="py-2">
            <span className="font-bold">Id</span>: {nfTicket.id.toString()}
          </div>
          <div className="py-2">
            <span className="font-bold">Signature</span>: {nfTicket.signature.slice(0, 5)}...
            {nfTicket.signature.slice(-4)}
          </div>
          <div className="flex justify-between gap-2">
            <div
              className="flex tooltip before:content-[attr(data-tip)] before:right-[-10px] before:left-auto before:transform-none"
              data-tip="Accept the ticket"
            >
              <button
                className={`btn btn-secondary btn-sm ${isLoading ? "loading" : ""}`}
                onClick={() => writeTx(() => writeUseTicket())}
              >
                Consume
              </button>
            </div>
          </div>
        </>
      )}
    </div>
  );
}
