import React, { useState } from "react";
import { useContractWrite, useNetwork } from "wagmi";
import { AddressInput, InputBase, IntegerInput } from "~~/components/scaffold-eth";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { getTargetNetwork } from "~~/utils/scaffold-eth";

type MintNewNftProps = {
  resturantAddress: string;
};

export default function MintNewNft({ resturantAddress }: MintNewNftProps) {
  const [newPrice, setPrice] = useState<bigint | string>("");
  const [newTokenAddress, setTokenAddress] = useState("");
  const [newReservationDate, setReservationDate] = useState<bigint | string>("");
  const [newUri, setUri] = useState<string>("");
  const writeTx = useTransactor();

  const { chain } = useNetwork();
  const writeDisabled = !chain || chain?.id !== getTargetNetwork().id || !Number(newPrice);

  const { writeAsync: writeSafeMint, isLoading } = useContractWrite({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "safeMint",
    args: [BigInt(newPrice), newTokenAddress, Number(newReservationDate), newUri],
  });

  return (
    <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl lg:px-2 mb-6 space-y-1 py-4 ">
      <AddressInput value={newTokenAddress} onChange={setTokenAddress} name="PaymentToken" placeholder="PaymentToken" />
      <IntegerInput
        value={newPrice}
        onChange={value => {
          try {
            const bigValue = BigInt(value);
            setPrice(bigValue);
          } catch {}
        }}
        name="Price"
        placeholder="Price"
      />
      <IntegerInput
        value={newReservationDate}
        onChange={value => {
          try {
            const bigValue = BigInt(value);
            setReservationDate(bigValue);
          } catch {}
        }}
        name="Reservation Date"
        placeholder="Reservation Date"
      />
      <InputBase value={newUri} onChange={setUri} name="Uri" placeholder="Uri" />
      <div className="flex justify-between gap-2">
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
            onClick={() => writeTx(() => writeSafeMint())}
          >
            Add
          </button>
        </div>
      </div>
    </div>
  );
}
