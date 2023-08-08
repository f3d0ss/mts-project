import React from "react";
import { useScaffoldContractWrite } from "~~/hooks/scaffold-eth";

type RemoveResturantButtonProps = {
  index: bigint;
};

export default function RemoveResturantButton({ index }: RemoveResturantButtonProps) {
  const { writeAsync: writeRemoveResturantAsync, isLoading: isLoadingRemove } = useScaffoldContractWrite({
    contractName: "MTSController",
    functionName: "removeResturant",
    args: [index],
    onBlockConfirmation: txnReceipt => {
      console.log("📦 Transaction blockHash", txnReceipt.blockHash);
    },
  });
  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        isLoadingRemove ? "loading" : ""
      }`}
      onClick={() => writeRemoveResturantAsync()}
    >
      Remove
    </button>
  );
}
