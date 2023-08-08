import React, { useEffect, useState } from "react";
import { AddressInput, IntegerInput, TxReceipt } from "../scaffold-eth";
import { TransactionReceipt } from "viem";
import { useNetwork, useWaitForTransaction } from "wagmi";
import { ArrowDownIcon, ArrowUpIcon } from "@heroicons/react/24/outline";
import { useScaffoldContractWrite } from "~~/hooks/scaffold-eth";
import { getTargetNetwork } from "~~/utils/scaffold-eth";

export default function AddNewAcceptableToken() {
  const [open, setOpen] = useState(false);
  const [newTokenAddress, setTokenAddress] = useState("");
  const [newMinPrice, setMinPrice] = useState<bigint | string>("");
  const { chain } = useNetwork();
  const writeDisabled = !chain || chain?.id !== getTargetNetwork().id || !Number(newMinPrice);

  const {
    writeAsync: writeSetAcceptableMinPrice,
    isLoading,
    data: result,
  } = useScaffoldContractWrite({
    contractName: "MTSController",
    functionName: "setAcceptableMinPrice",
    args: [newTokenAddress, newMinPrice as bigint],
    onBlockConfirmation: txnReceipt => {
      console.log("📦 Transaction blockHash", txnReceipt.blockHash);
    },
  });

  const [displayedTxResult, setDisplayedTxResult] = useState<TransactionReceipt>();
  const { data: txResult } = useWaitForTransaction({
    hash: result?.hash,
  });
  useEffect(() => {
    setDisplayedTxResult(txResult);
  }, [txResult]);

  return (
    <div className="p-5 space-y-3 first:pt-0">
      <button onClick={() => setOpen(!open)}>
        <p className="font-medium my-0 break-words inline-flex">Add A New Token</p>
        {open ? (
          <ArrowUpIcon className="inline ml-4" height="20px" width="20px" />
        ) : (
          <ArrowDownIcon className="inline ml-4" height="20px" width="20px" />
        )}
      </button>

      <div className={`flex gap-3 flex-col ${!open && "hidden"}`}>
        <AddressInput
          value={newTokenAddress}
          onChange={setTokenAddress}
          name="Token Address"
          placeholder="Token Address"
        />
        <IntegerInput
          value={newMinPrice}
          onChange={value => {
            try {
              const bigValue = value as bigint;
              setMinPrice(bigValue);
            } catch {}
          }}
          name="Min Price"
          placeholder="Min Price"
        />
        <div className="flex justify-between gap-2">
          <div className="flex-grow basis-0">
            {displayedTxResult ? <TxReceipt txResult={displayedTxResult} /> : null}
          </div>

          <div
            className={`flex ${
              writeDisabled &&
              "tooltip before:content-[attr(data-tip)] before:right-[-10px] before:left-auto before:transform-none"
            }`}
            data-tip={`${writeDisabled && "Wallet not connected or in the wrong network"}`}
          >
            <button
              className={`btn btn-secondary btn-sm ${isLoading ? "loading" : ""}`}
              disabled={writeDisabled}
              onClick={() => writeSetAcceptableMinPrice()}
            >
              Add
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
