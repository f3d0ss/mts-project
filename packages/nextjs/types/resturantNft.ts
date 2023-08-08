export type ResturantNft = {
  id: bigint;
  price: bigint;
  paymentToken: string;
  reservationDate: number;
  locked: boolean;
  uri: string;
  resturant: string;
  owner: string;
};
