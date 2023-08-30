import React from "react";
import { useContractWrite } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { useIPFSGateway } from "~~/hooks/useIPFSGateway";
import { NftReview } from "~~/types/nftReview";
import { ResturantNft } from "~~/types/resturantNft";
import { notification } from "~~/utils/scaffold-eth";

type SaveReviewButtonProps = {
  buttonClass: string;
  review?: NftReview;
  nft: ResturantNft;
  onSave: (success: boolean) => void;
};

export default function SaveReviewButton({ buttonClass, review, nft, onSave }: SaveReviewButtonProps) {
  const writeTxn = useTransactor();
  const { uploadFile, isConnecting } = useIPFSGateway();

  const { writeAsync: saveReviewUri, isLoading } = useContractWrite({
    address: nft.resturant,
    abi: contracts.ResturantToken.abi,
    functionName: "sendReview",
  });

  const handleSave = async () => {
    if (isConnecting) {
      notification.loading("Connecting to IPFS");
      return;
    }

    const encoder = new TextEncoder();
    const encodedReview = encoder.encode(JSON.stringify(review));

    const reviewCID = await uploadFile(encodedReview);

    if (!reviewCID) {
      notification.error("Error connecting to IPFS");
      return;
    }
    const result = await writeTxn(() => saveReviewUri({ args: [nft.id, reviewCID.toString()] }));
    onSave(result?.length != 0);
  };
  return (
    <button className={`${buttonClass}  ${isLoading ? "loading" : ""}`} disabled={!review} onClick={handleSave}>
      Save review
    </button>
  );
}
