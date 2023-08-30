import React, { useEffect, useState } from "react";
import { Spinner } from "../Spinner";
import MarkdownDisplay from "../markdown-editor/MarkdownDisplay";
import MarkdownEditor from "../markdown-editor/MarkdownEditor";
import SaveReviewButton from "../my-nft/SaveReviewButton";
import { useIPFSGateway } from "~~/hooks/useIPFSGateway";
import { NftReview, V1 } from "~~/types/nftReview";
import { ResturantNft } from "~~/types/resturantNft";

type NftPreviewReviewProps = {
  nft: ResturantNft;
  editableReview?: boolean;
};

export default function NftPreviewReview({ nft, editableReview }: NftPreviewReviewProps) {
  const [review, setReview] = useState<NftReview | undefined>();
  const [editingReview, setEditingReview] = useState<NftReview | undefined>();
  const [editing, setEditing] = useState<boolean>(false);

  const { isOnline, isConnecting, getFile } = useIPFSGateway();

  useEffect(() => {
    const init = async () => {
      if (!nft.reviewUri || nft.reviewUri == "") return;

      const jsonFile = await getFile(nft.reviewUri);
      if (!jsonFile) return;
      const rawJson = Buffer.from(jsonFile).toString("utf8");
      const json: NftReview = JSON.parse(rawJson);
      setReview(json);
    };
    init();
  }, [nft.reviewUri, isOnline, getFile]);

  const hasAReview = nft.reviewUri && nft.reviewUri.length != 0;

  if (!nft.locked || (!editableReview && !hasAReview)) {
    return <></>;
  }

  let reviewContainer;
  if (editing) {
    reviewContainer = (
      <MarkdownEditor
        value={review?.reviewPayload}
        onChange={reviewText => setEditingReview({ version: V1, reviewPayload: reviewText })}
      />
    );
  } else {
    if (review) {
      reviewContainer = <MarkdownDisplay>{review?.reviewPayload}</MarkdownDisplay>;
    } else {
      reviewContainer = isConnecting ? <Spinner /> : "Error connecting to IPFS";
    }
  }

  const buttonClass =
    "btn btn-primary rounded-full capitalize font-normal font-white flex items-center gap-1 hover:gap-2 transition-all tracking-widest";

  return (
    <div className="py-2 flex flex-col">
      <div className="flex justify-between items-center pb-4">
        <span className="font-bold">Review: </span>
        {hasAReview && editableReview && !editing && (
          <button
            className={buttonClass}
            onClick={() => {
              setEditing(true);
              setEditingReview(review);
            }}
          >
            Edit the review
          </button>
        )}
        {!hasAReview && editableReview && !editing && (
          <button
            className={buttonClass}
            onClick={() => {
              setEditing(true);
              setEditingReview(review);
            }}
          >
            Add review
          </button>
        )}
        {editableReview && editing && (
          <SaveReviewButton
            buttonClass={buttonClass}
            nft={nft}
            review={editingReview}
            onSave={success => {
              if (success) {
                setReview(editingReview);
              }
              setEditing(false);
            }}
          />
        )}
      </div>
      {reviewContainer}
    </div>
  );
}
