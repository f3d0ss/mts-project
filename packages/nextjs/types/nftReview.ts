export const V1 = "v1";
export type NftReview = {
  version: string;
  reviewPayload: ReviewPayloadV1;
};

export type ReviewPayloadV1 = string;
