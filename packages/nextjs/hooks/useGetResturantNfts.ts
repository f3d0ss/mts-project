import { useContractRead, useContractReads } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { FetchedNft } from "~~/types/fetchedNft";
import { ResturantNft } from "~~/types/resturantNft";

type UseGetResturantNftsProps = {
  resturant: string;
  ownedBy?: string;
  onlyOnSale?: boolean;
};

export const useGetResturantNfts = ({ resturant, onlyOnSale, ownedBy }: UseGetResturantNftsProps) => {
  if (onlyOnSale) {
    ownedBy = resturant;
  }
  const functionTotalName = ownedBy ? "balanceOf" : "totalSupply";
  const args: [string] | undefined = ownedBy ? [ownedBy] : undefined;

  const { data: numberOfTokenBigInt, isLoading: isLoadingNumberOfToken } = useContractRead({
    address: resturant,
    abi: contracts.ResturantToken.abi,
    functionName: functionTotalName,
    args: args,
    watch: true,
  });

  const numberOfToken = Number(numberOfTokenBigInt);

  const functionByIndexName = ownedBy ? "tokenOfOwnerByIndex" : "tokenByIndex";
  const tokenIdsReads = Array.from({ length: numberOfToken }).map((_, i) => {
    const argsByIndex: [string, bigint] | [bigint] = ownedBy ? [ownedBy, BigInt(i)] : [BigInt(i)];

    return {
      address: resturant,
      abi: contracts.ResturantToken.abi,
      functionName: functionByIndexName,
      args: onlyOnSale ? [resturant, BigInt(i)] : argsByIndex,
    };
  });

  const { data: tokenIds, isLoading: isLoadingTokenIds } = useContractReads({
    contracts: tokenIdsReads,
    watch: true,
  });

  const tokenIdsTyped = tokenIds && tokenIds.map(tokenIdData => tokenIdData.result as bigint);

  const getNftReads = tokenIdsTyped?.map(tokenId => {
    return {
      address: resturant,
      abi: contracts.ResturantToken.abi,
      functionName: "getNft",
      args: [tokenId],
    };
  });

  const getOwnerReads = tokenIdsTyped?.map(tokenId => {
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

  const nfts = new Array<ResturantNft>();
  if (tokenIds) {
    fetchedNftsTyped?.forEach((fetchedNft: FetchedNft, i) => {
      if (tokenIds[i].result) {
        nfts.push({
          ...fetchedNft,
          resturant,
          id: tokenIds[i].result as bigint,
          owner: nftOwnersTyped ? nftOwnersTyped[i] : "",
        });
      }
    });
  }

  return { nfts, isLoading: isLoadingNumberOfToken || isLoadingTokenIds || isLoadingNfts || isLoadingOwners };
};
