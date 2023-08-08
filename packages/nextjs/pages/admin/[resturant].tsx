import React from "react";
import dynamic from "next/dynamic";
import { useRouter } from "next/router";
import { NextPage } from "next";
import { MetaHeader } from "~~/components/MetaHeader";
import { Spinner } from "~~/components/Spinner";
import AdminNftPreview from "~~/components/resturant/admin/AdminNftPreview";
import { useGetResturantNfts } from "~~/hooks/useGetResturantNfts";

const ResturantAdminPage: NextPage = dynamic(
  () =>
    Promise.resolve(() => {
      const router = useRouter();
      const resturant = router.query.resturant as string;

      const { nfts, isLoading } = useGetResturantNfts({ resturant });

      return (
        <>
          <MetaHeader />
          {isLoading ? (
            <Spinner height="50px" width="50px" />
          ) : (
            <div className="col-span-5 grid grid-cols-1 lg:grid-cols-5 gap-8 lg:gap-10 p-8">
              <div className="col-span-1"></div>
              <div className="col-span-2 grid grid-cols-2 gap-x-2">
                {nfts
                  ?.filter(nft => nft.owner === nft.resturant)
                  .map(nft => (
                    <AdminNftPreview nft={nft} key={nft.id.toString()} />
                  ))}
              </div>
              <div className="col-span-2 grid grid-cols-2 gap-x-2">
                {nfts
                  ?.filter(nft => nft.owner !== nft.resturant)
                  .map(nft => (
                    <AdminNftPreview nft={nft} key={nft.id.toString()} />
                  ))}
              </div>
            </div>
          )}
        </>
      );
    }),
  { ssr: false },
);

export default ResturantAdminPage;
