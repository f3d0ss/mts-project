import React from "react";
import dynamic from "next/dynamic";
import { NextPage } from "next";
import { useAccount } from "wagmi";
import { MetaHeader } from "~~/components/MetaHeader";
import { Spinner } from "~~/components/Spinner";
import OwnedNftPreview from "~~/components/my-nft/OwnedNftPreview";
import { useControllerResturant } from "~~/hooks/useControllerResturants";
import { useGetResturantsNfts } from "~~/hooks/useGetResturantsNfts";

const MyNfts: NextPage = dynamic(
  () =>
    Promise.resolve(() => {
      const resturants = useControllerResturant();
      const { address } = useAccount();
      console.log({ address });
      const resturantAddresses = [...resturants].map(resturant => resturant.address);
      const { nfts } = useGetResturantsNfts({ resturants: resturantAddresses, ownedBy: address });

      return (
        <>
          <MetaHeader />
          {!nfts ? (
            <Spinner height="50px" width="50px" />
          ) : (
            <div className="col-span-5 grid grid-cols-1 lg:grid-cols-5 gap-8 lg:gap-10 p-8">
              {[...nfts.values()]
                ?.sort((nftA, nftB) => nftA.reservationDate - nftB.reservationDate)
                .map(nft => (
                  <OwnedNftPreview nft={nft} key={nft.id.toString()} />
                ))}
            </div>
          )}
        </>
      );
    }),
  { ssr: false },
);

export default MyNfts;
