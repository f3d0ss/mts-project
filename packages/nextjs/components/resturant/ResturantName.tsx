import React, { useState } from "react";
import { Spinner } from "../Spinner";
import { Address } from "../scaffold-eth";
import { useContractRead } from "wagmi";
import contracts from "~~/generated/usefulAbis";

type ResturantNameProps = {
  resturantAddress: string;
};

export default function ResturantName({ resturantAddress }: ResturantNameProps) {
  const [showAddress, setShowAddress] = useState(false);
  const { data: resturantName } = useContractRead({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "name",
  });
  return (
    <>
      <button onClick={() => setShowAddress(!showAddress)}>
        <span className="font-bold">Resturant</span>:
      </button>
      {showAddress ? (
        <Address address={resturantAddress} />
      ) : resturantName ? (
        <div className="flex items-center">{resturantName}</div>
      ) : (
        <Spinner />
      )}
    </>
  );
}
