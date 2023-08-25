import React, { useState } from "react";
import { InputImage } from "./InputImage";
import { useContractWrite, useNetwork } from "wagmi";
import { TextAreaInput } from "~~/components/TextAreaInput";
import { AddressInput, InputBase, IntegerInput } from "~~/components/scaffold-eth";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { useIPFSGateway } from "~~/hooks/useIPFSGateway";
import { NftMetadata } from "~~/types/nftMetadata";
import { getTargetNetwork } from "~~/utils/scaffold-eth";

type MintNewNftProps = {
  resturantAddress: string;
};

export default function MintNewNft({ resturantAddress }: MintNewNftProps) {
  const [newImage, setImage] = useState<Uint8Array>();
  const [newName, setName] = useState<string>("");
  const [newDescription, setDescription] = useState<string>("");
  const [newPrice, setPrice] = useState<bigint | string>("");
  const [newTokenAddress, setTokenAddress] = useState("");
  const [newReservationDate, setReservationDate] = useState<bigint | string>("");
  const writeTx = useTransactor();

  const { chain } = useNetwork();
  const writeDisabled = !chain || chain?.id !== getTargetNetwork().id || !Number(newPrice);

  const { writeAsync: writeSafeMint, isLoading } = useContractWrite({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "safeMint",
  });

  const { uploadFile } = useIPFSGateway();

  async function submit() {
    if (!newImage) {
      console.error("need an image to create a new NFT!");
      return;
    }
    if (!uploadFile) {
      console.error("ipfs not yet ready");
      return;
    }
    const newImageCID = await uploadFile(newImage);

    const metadata: NftMetadata = {
      description: newDescription,
      external_url: `http://localhost:3000/${resturantAddress}`,
      image: `ipfs://${newImageCID}`,
      name: newName,
      attributes: [],
    };
    const jsonMetadata = JSON.stringify(metadata);
    const encoder = new TextEncoder();
    const encodedString = encoder.encode(jsonMetadata);

    const nftCID = await uploadFile(encodedString);
    writeTx(() =>
      writeSafeMint({
        args: [BigInt(newPrice), newTokenAddress, Number(newReservationDate), `ipfs://${nftCID.toString()}`],
      }),
    );
    console.log(`ipfs://${nftCID.toString()}`);
  }

  return (
    <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl lg:px-2 mb-6 space-y-1 py-4 ">
      <InputImage onChange={setImage} />
      <InputBase value={newName} onChange={setName} name="Name" placeholder="Name" />
      <TextAreaInput
        value={newDescription}
        onChange={setDescription}
        name="Description"
        placeholder="Description"
        rows={4}
      />
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
            onClick={submit}
          >
            Add
          </button>
        </div>
      </div>
    </div>
  );
}
