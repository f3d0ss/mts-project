import React from "react";
import dynamic from "next/dynamic";
import { useRouter } from "next/router";
import { NextPage } from "next";
import { MetaHeader } from "~~/components/MetaHeader";
import { Spinner } from "~~/components/Spinner";
import UserNftPreview from "~~/components/resturant/UserNftPreview";
import { useGetResturantNfts } from "~~/hooks/useGetResturantNfts";

const ResturantPage: NextPage = dynamic(
  () =>
    Promise.resolve(() => {
      const router = useRouter();
      const resturant = router.query.resturant as string;

      const { nfts, isLoading } = useGetResturantNfts({ resturant, onlyOnSale: true });

      return (
        <>
          <MetaHeader />
          {isLoading ? (
            <Spinner height="50px" width="50px" />
          ) : (
            <div className="col-span-5 grid grid-cols-1 lg:grid-cols-5 gap-8 lg:gap-10 p-8">
              {nfts?.map(nft => (
                <UserNftPreview nft={nft} key={nft.id.toString()} />
              ))}
            </div>
          )}
        </>
      );
    }),
  { ssr: false },
);

export default ResturantPage;
