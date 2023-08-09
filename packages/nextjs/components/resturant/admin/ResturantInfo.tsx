import React from "react";
import { Address, Balance } from "../../scaffold-eth";
import { useContractReads } from "wagmi";
import AcceptableTokenBalances from "~~/components/home/AcceptableTokenBalances";
import contracts from "~~/generated/usefulAbis";

type ResturantInfoProps = {
  resturantAddress: string;
};

export default function ResturantInfo({ resturantAddress }: ResturantInfoProps) {
  const { data } = useContractReads({
    contracts: [
      {
        address: resturantAddress,
        abi: contracts.ResturantToken.abi,
        functionName: "owner",
      },
      {
        address: resturantAddress,
        abi: contracts.ResturantToken.abi,
        functionName: "getControllerAddress",
      },
      {
        address: resturantAddress,
        abi: contracts.ResturantToken.abi,
        functionName: "name",
      },
    ],
  });
  const owner = data ? (data[0].result as string) : undefined;
  const controllerAddress = data ? (data[1].result as string) : undefined;
  const name = data ? (data[2].result as string) : undefined;

  return (
    <div className="col-span-1 flex flex-col">
      <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl px-6 lg:px-8 mb-6 space-y-1 py-4">
        <div className="flex">
          <div className="flex flex-col gap-1">
            <span className="font-bold">{name}</span>
            <Address address={resturantAddress} />
            <span className="font-bold">Owner:</span>
            <Address address={owner} />
            <div className="flex gap-1 items-center">
              <span className="font-bold text-sm">Balances:</span>
            </div>
            <div className="flex gap-1 items-center py-1">
              <Balance address={resturantAddress} className="px-0 h-1.5 min-h-[0.375rem]" />
            </div>
            {controllerAddress && (
              <AcceptableTokenBalances controllerAddress={controllerAddress} addressToCheck={resturantAddress} />
            )}
          </div>
        </div>
      </div>
    </div>
  );
}
