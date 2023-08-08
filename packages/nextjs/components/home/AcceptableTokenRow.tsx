import React, { useState } from "react";
import { Spinner } from "../Spinner";
import { Address, IntegerInput } from "../scaffold-eth";
import RemoveAcceprableTokenButton from "./RemoveAcceprableTokenButton";
import { formatUnits } from "viem";
import { useToken } from "wagmi";
import { useScaffoldContractWrite } from "~~/hooks/scaffold-eth";

type AcceptableTokenRowProps = {
  tokenAddress: string;
  minPrice: bigint;
};

export default function AcceptableTokenRow({ tokenAddress, minPrice }: AcceptableTokenRowProps) {
  const [editing, setEditing] = useState(false);
  const [stateMinPrice, setStateMinPrice] = useState(minPrice);

  const { data } = useToken({
    address: tokenAddress,
  });

  const { writeAsync: writeSetAcceptableMinPrice, isLoading } = useScaffoldContractWrite({
    contractName: "MTSController",
    functionName: "setAcceptableMinPrice",
    args: [tokenAddress, stateMinPrice],
    onBlockConfirmation: txnReceipt => {
      console.log("📦 Transaction blockHash", txnReceipt.blockHash);
    },
  });

  async function handleEditing() {
    if (!editing) {
      setEditing(true);
    } else {
      if (stateMinPrice !== minPrice) await writeSetAcceptableMinPrice();
      setEditing(false);
    }
  }

  return (
    <div className="flex justify-between">
      <div className="flex flex-1">
        <div className="p-5 flex-2 flex flex-col align-middle">
          <span className="font-bold">{data ? (data.name as string) : "fetching..."}</span>
          <Address address={tokenAddress} />
        </div>
        <div className="p-5 flex flex-col justify-center">
          {editing ? (
            <IntegerInput
              value={stateMinPrice}
              onChange={value => {
                try {
                  const bigValue = value as bigint;
                  setStateMinPrice(bigValue);
                } catch {}
              }}
              name="Min Price"
              placeholder="Min Price"
            />
          ) : (
            <div>
              <span>{formatUnits(minPrice, data?.decimals || 18)}</span>{" "}
              <span className="font-bold">{data ? data.symbol : "fetching..."}</span>
            </div>
          )}
        </div>
      </div>
      <div className="py-5 px-1">
        <button
          className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
            isLoading && "btn-disabled"
          }`}
          onClick={handleEditing}
        >
          {isLoading ? <Spinner /> : editing ? "Confirm" : "Edit"}
        </button>
      </div>
      <div className="py-5 px-1">
        <RemoveAcceprableTokenButton tokenAddress={tokenAddress} />
      </div>
    </div>
  );
}
