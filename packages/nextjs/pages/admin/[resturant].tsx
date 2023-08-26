import React from "react";
import dynamic from "next/dynamic";
import { useRouter } from "next/router";
import { NextPage } from "next";
import { MetaHeader } from "~~/components/MetaHeader";
import { Spinner } from "~~/components/Spinner";
import AcceptUserNft from "~~/components/resturant/admin/AcceptUserNft";
import AdminNftPreview from "~~/components/resturant/admin/AdminNftPreview";
import MintNewNft from "~~/components/resturant/admin/MintNewNft";
import ResturantInfo from "~~/components/resturant/admin/ResturantInfo";
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
              <div className="col-span-1">
                <div>
                  <div className="col-span-1 h-12 text-center font-bold">Info</div>
                  <ResturantInfo resturantAddress={resturant} />
                  <div className="col-span-1 h-12 text-center font-bold">Mint</div>
                  <MintNewNft resturantAddress={resturant} />
                  <div className="col-span-1 h-12 text-center font-bold">Consume</div>
                  <AcceptUserNft resturantAddress={resturant} />
                </div>
              </div>
              <div className="col-span-2 grid grid-cols-2 gap-2 content-start">
                <div className="col-span-2 h-12 text-center font-bold">On Sale</div>
                {nfts
                  ?.filter(nft => nft.owner === nft.resturant)
                  .filter(nft => nft.id)
                  .sort((nftA, nftB) => nftA.reservationDate - nftB.reservationDate)
                  .map(nft => (
                    <AdminNftPreview nft={nft} key={nft.id.toString()} />
                  ))}
              </div>
              <div className="col-span-2 grid grid-cols-2 gap-2 content-start">
                <div className="col-span-2 h-12 text-center font-bold">Sold</div>

                {nfts
                  ?.filter(nft => nft.owner !== nft.resturant)
                  .filter(nft => nft.id)
                  .sort((nftA, nftB) => nftA.reservationDate - nftB.reservationDate)
                  .map(nft => (
                    <AdminNftPreview nft={nft} showOwner={true} key={nft.id.toString()} />
                  ))}
              </div>
              <div className="col-span-1"></div>
            </div>
          )}
        </>
      );
    }),
  { ssr: false },
);

export default ResturantAdminPage;
