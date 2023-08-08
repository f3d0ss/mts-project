import React from "react";
import { Spinner } from "../Spinner";
import { useScaffoldContractWrite } from "~~/hooks/scaffold-eth";

type RemoveAcceprableTokenButtonProps = {
  tokenAddress: string;
};

export default function RemoveAcceprableTokenButton({ tokenAddress }: RemoveAcceprableTokenButtonProps) {
  const { writeAsync: writeSetAcceptableMinPrice, isLoading } = useScaffoldContractWrite({
    contractName: "MTSController",
    functionName: "removeAcceptableToken",
    args: [tokenAddress],
    onBlockConfirmation: txnReceipt => {
      console.log("📦 Transaction blockHash", txnReceipt.blockHash);
    },
  });
  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        isLoading && "btn-disabled"
      }`}
      onClick={() => writeSetAcceptableMinPrice()}
    >
      {isLoading ? <Spinner /> : "Remove"}
    </button>
  );
}
