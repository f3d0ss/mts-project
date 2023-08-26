import React from "react";
import Link from "next/link";
import { Address } from "../scaffold-eth";
import PauseResturantButton from "./PauseResturantButton";
import RemoveResturantButton from "./RemoveResturantButton";

type ResturantRowProps = {
  index: bigint;
  name: string;
  address: string;
};

export default function ResturantRow({ index, name, address }: ResturantRowProps) {
  return (
    <div className="flex justify-between">
      <div className="p-5  flex-1">
        <span className="font-bold">{name}</span>
        <Address address={address} />
      </div>
      <div className="py-5 px-1">
        <Link
          className="btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest"
          href={`resturant/${address}`}
        >
          Open
        </Link>
      </div>
      <div className="py-5 px-1">
        <Link
          className="btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest"
          href={`admin/${address}`}
        >
          Admin
        </Link>
      </div>
      <div className="py-5 px-1">
        <PauseResturantButton index={index} resturantAddress={address} />
      </div>
      <div className="py-5 px-1">
        <RemoveResturantButton index={index} />
      </div>
    </div>
  );
}
