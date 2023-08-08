import { useContractRead, useContractReads } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { ResturantNft } from "~~/types/resturantNft";

type FetchedNft = {
  price: bigint;
  paymentToken: string;
  reservationDate: number;
  locked: boolean;
  uri: string;
};

type UseGetResturantNftsProps = {
  resturant: string;
  onlyOnSale?: boolean;
};

export const useGetResturantNfts = ({ resturant, onlyOnSale }: UseGetResturantNftsProps) => {
  const functionName = onlyOnSale ? "balanceOf" : "totalSupply";
  const args: [string] | undefined = onlyOnSale ? [resturant] : undefined;
  const { data: numberOfTokenBigInt, isLoading: isLoadingNumberOfToken } = useContractRead({
    address: resturant,
    abi: contracts.ResturantToken.abi,
    functionName: functionName,
    args: args,
    watch: true,
  });

  const numberOfToken = Number(numberOfTokenBigInt);

  const tokenIdsReads = Array.from({ length: numberOfToken }).map((_, i) => {
    return {
      address: resturant,
      abi: contracts.ResturantToken.abi,
      functionName: "tokenByIndex",
      args: [i],
    };
  });

  const { data: tokenIds, isLoading: isLoadingTokenIds } = useContractReads({
    contracts: tokenIdsReads,
    watch: true,
  });

  const getNftReads = tokenIds?.map(tokenId => {
    return {
      address: resturant,
      abi: contracts.ResturantToken.abi,
      functionName: "getNft",
      args: [tokenId],
    };
  });

  const getOwnerReads = tokenIds?.map(tokenId => {
    return {
      address: resturant,
      abi: contracts.ResturantToken.abi,
      functionName: "ownerOf",
      args: [tokenId],
    };
  });

  const { data: fetchedNfts, isLoading: isLoadingNfts } = useContractReads({ contracts: getNftReads, watch: true });
  const { data: nftOwners, isLoading: isLoadingOwners } = useContractReads({
    contracts: getOwnerReads,
    watch: true,
  });
  const nftOwnersTyped = nftOwners && nftOwners.map(nftOwnerData => nftOwnerData.result as string);

  const fetchedNftsTyped = fetchedNfts && fetchedNfts.map(fetchedNft => fetchedNft.result as FetchedNft);

  let nfts;
  if (tokenIds) {
    nfts = fetchedNftsTyped?.map<ResturantNft>((fetchedNft: FetchedNft, i) => {
      return {
        ...fetchedNft,
        resturant,
        id: tokenIds[i].result as bigint,
        owner: nftOwnersTyped ? nftOwnersTyped[i] : "",
      };
    });
  }

  return { nfts, isLoading: isLoadingNumberOfToken || isLoadingTokenIds || isLoadingNfts || isLoadingOwners };
};
