import React from "react";
import { Address, Balance } from "../scaffold-eth";
import ControllerBalances from "./ControllerBalances";
import { useScaffoldContractRead } from "~~/hooks/scaffold-eth";

type ControllerInfoProps = {
  controllerName: string;
  controllerAddress: string;
};

export default function ControllerInfo({ controllerName, controllerAddress }: ControllerInfoProps) {
  const { data } = useScaffoldContractRead({
    contractName: "MTSController",
    functionName: "owner",
    watch: true,
  });
  return (
    <div className="col-span-1 flex flex-col">
      <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl px-6 lg:px-8 mb-6 space-y-1 py-4">
        <div className="flex">
          <div className="flex flex-col gap-1">
            <span className="font-bold">{controllerName}</span>
            <Address address={controllerAddress} />
            <span className="font-bold">Owner:</span>
            <Address address={data} />
            <div className="flex gap-1 items-center">
              <span className="font-bold text-sm">Balances:</span>
            </div>
            <div className="flex gap-1 items-center">
              <Balance address={controllerAddress} className="px-0 h-1.5 min-h-[0.375rem]" />
            </div>
            <ControllerBalances controllerAddress={controllerAddress}></ControllerBalances>
          </div>
        </div>
      </div>
    </div>
  );
}
