import React, { useState } from "react";
import { InputImage } from "./InputImage";
import { useContractWrite, useNetwork } from "wagmi";
import DatePicker from "~~/components/DatePicker";
import MarkdownEditor from "~~/components/markdown-editor/MarkdownEditor";
import { AddressInput, InputBase, IntegerInput } from "~~/components/scaffold-eth";
import contracts from "~~/generated/usefulAbis";
import { useTransactor } from "~~/hooks/scaffold-eth";
import { useIPFSGateway } from "~~/hooks/useIPFSGateway";
import { NftMetadata } from "~~/types/nftMetadata";
import { getTargetNetwork, notification } from "~~/utils/scaffold-eth";

type MintNewNftProps = {
  resturantAddress: string;
};

export default function MintNewNft({ resturantAddress }: MintNewNftProps) {
  const [newImage, setImage] = useState<Uint8Array>();
  const [newName, setName] = useState<string>("");
  const [newDescription, setDescription] = useState<string>("");
  const [newPrice, setPrice] = useState<bigint | string>("");
  const [newTokenAddress, setTokenAddress] = useState("");
  const [newReservationDate, setReservationDate] = useState<Date>(new Date());
  const writeTx = useTransactor();

  const { chain } = useNetwork();
  const writeDisabled = !chain || chain?.id !== getTargetNetwork().id || !Number(newPrice);

  const { writeAsync: writeSafeMint, isLoading } = useContractWrite({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "safeMint",
  });

  const { uploadFile, isConnecting } = useIPFSGateway();

  async function submit() {
    if (!newImage) {
      notification.error("Need an image to create a new NFT!");
      return;
    }
    if (isConnecting) {
      notification.loading("Connecting to IPFS");
      return;
    }

    const newImageCID = await uploadFile(newImage);

    if (!newImageCID) {
      notification.error("Error connecting to IPFS");
      return;
    }

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
    if (!nftCID) {
      notification.error("Error connecting to IPFS");
      return;
    }
    writeTx(() =>
      writeSafeMint({
        args: [
          BigInt(newPrice),
          newTokenAddress,
          (newReservationDate.getTime() - newReservationDate.getMilliseconds()) / 1000,
          `ipfs://${nftCID.toString()}`,
        ],
      }),
    );
    console.log(`ipfs://${nftCID.toString()}`);
  }

  console.log(newReservationDate);
  return (
    <div className="bg-base-100 border-base-300 border shadow-md shadow-secondary rounded-3xl lg:px-2 mb-6 space-y-1 py-4 ">
      <InputImage onChange={setImage} />
      <InputBase value={newName} onChange={setName} name="Name" placeholder="Name" />
      <MarkdownEditor value={newDescription} onChange={setDescription} placeholder="Description" />
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
      <DatePicker value={newReservationDate} onChange={setReservationDate} />
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
