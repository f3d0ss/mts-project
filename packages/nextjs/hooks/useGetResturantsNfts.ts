import { useContractReads } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { FetchedNft } from "~~/types/fetchedNft";
import { ResturantNft } from "~~/types/resturantNft";

type UseGetResturantsNftsProps = {
  resturants: string[];
  ownedBy?: string;
  onlyOnSale?: boolean;
  watch?: boolean;
};

export const useGetResturantsNfts = ({ resturants, onlyOnSale, ownedBy, watch }: UseGetResturantsNftsProps) => {
  const functionTotalName = ownedBy ? "balanceOf" : "totalSupply";
  const argsTotal: [string] | undefined = ownedBy ? [ownedBy] : undefined;

  const numbersOfTokensReads = resturants.map(resturant => {
    return {
      address: resturant,
      abi: contracts.ResturantToken.abi,
      functionName: functionTotalName,
      args: onlyOnSale ? [resturant] : argsTotal,
    };
  });

  const { data: numberOfTokenPerResturantBigInt, isLoading: isLoadingNumberOfToken } = useContractReads({
    contracts: numbersOfTokensReads,
    watch,
  });

  const numberOfTokenPerResturant = numberOfTokenPerResturantBigInt?.map(numberOfTokenBigInt =>
    Number(numberOfTokenBigInt.result),
  );

  const functionByIndexName = ownedBy ? "tokenOfOwnerByIndex" : "tokenByIndex";

  let tokenIdsReadsPerResturant: any[] = [];
  numberOfTokenPerResturant?.forEach((numberOfTokens, i) => {
    const tokenIdsReadsPerResturantI = Array.from({ length: numberOfTokens }).map((_, k) => {
      const argsByIndex: [string, bigint] | undefined = ownedBy ? [ownedBy, BigInt(k)] : undefined;

      return {
        address: resturants[i],
        abi: contracts.ResturantToken.abi,
        functionName: functionByIndexName,
        args: onlyOnSale ? [resturants[i], BigInt(k)] : argsByIndex,
      };
    });
    tokenIdsReadsPerResturant = tokenIdsReadsPerResturant.concat(tokenIdsReadsPerResturantI);
  });

  const { data: tokenIdsPerResturant, isLoading: isLoadingTokenIds } = useContractReads({
    contracts: tokenIdsReadsPerResturant,
    watch,
  });

  const tokenIdsPerResturantTyped =
    tokenIdsPerResturant && tokenIdsPerResturant.map(tokenIdData => tokenIdData.result as bigint);

  const tokenIdsPerResturantSliced = splitDataByContract(
    numberOfTokenPerResturant ?? [],
    tokenIdsPerResturantTyped ?? [],
  );

  const getNftReads: any[] = [];
  const getOwnerReads: any[] = [];
  tokenIdsPerResturantSliced?.forEach((resturantTokenIds, i) =>
    resturantTokenIds.forEach(resturantTokenId => {
      getNftReads.push({
        address: resturants[i],
        abi: contracts.ResturantToken.abi,
        functionName: "getNft",
        args: [resturantTokenId],
      });
      getOwnerReads.push({
        address: resturants[i],
        abi: contracts.ResturantToken.abi,
        functionName: "ownerOf",
        args: [resturantTokenId],
      });
    }),
  );

  const { data: fetchedNfts, isLoading: isLoadingNfts } = useContractReads({ contracts: getNftReads, watch });
  const { data: nftOwners, isLoading: isLoadingOwners } = useContractReads({
    contracts: getOwnerReads,
    watch,
  });
  const nftOwnersTyped = nftOwners && nftOwners.map(nftOwnerData => nftOwnerData.result as string);
  const nftOwnersTypedSplit = splitDataByContract(numberOfTokenPerResturant ?? [], nftOwnersTyped ?? []);

  console.warn({ fetchedNfts });
  const fetchedNftsTyped = fetchedNfts && fetchedNfts.map(fetchedNft => fetchedNft.result as FetchedNft);
  const fetchedNftsTypedSplit = splitDataByContract(numberOfTokenPerResturant ?? [], fetchedNftsTyped ?? []);

  const nfts = new Array<ResturantNft>();
  if (tokenIdsPerResturant) {
    fetchedNftsTypedSplit.forEach((fetchedNftsTyped, i) =>
      fetchedNftsTyped.forEach((fetchedNft, k) => {
        nfts.push({
          ...fetchedNft,
          resturant: resturants[i],
          id: tokenIdsPerResturantSliced[i][k],
          owner: nftOwnersTypedSplit[i][k],
        });
      }),
    );
  }

  return { nfts, isLoading: isLoadingNumberOfToken || isLoadingTokenIds || isLoadingNfts || isLoadingOwners };
};

function splitDataByContract<T>(tokenCounts: number[], tokenIds: T[]): T[][] {
  const result: T[][] = [];
  let startIndex = 0;

  for (let i = 0; i < tokenCounts.length; i++) {
    const endIndex = startIndex + tokenCounts[i];
    const contractIds = tokenIds.slice(startIndex, endIndex);
    result.push(contractIds);
    startIndex = endIndex;
  }

  return result;
}
