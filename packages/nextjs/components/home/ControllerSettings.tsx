import React from "react";
import AcceptableTokenRow from "./AcceptableTokenRow";
import AddNewAcceptableToken from "./AddNewAcceptableToken";
import { useControllerMinPrices } from "~~/hooks/useControllerMinPrices";

export default function ControllerSettings() {
  const acceptableMinPrice = useControllerMinPrices();

  return (
    <div className="col-span-1 lg:col-span-2 flex flex-col gap-4">
      <div className="bg-base-100 p-2 rounded-3xl divide-y divide-base-300  shadow-md shadow-secondary border border-base-300 flex flex-col relative">
        <span className="font-bold p-2 text-3xl sm:text-3xl ">Acceptable Tokens</span>
        {[...acceptableMinPrice].map(
          ([tokenAddress, minPrice]) =>
            minPrice > 0 && <AcceptableTokenRow tokenAddress={tokenAddress} minPrice={minPrice} key={tokenAddress} />,
        )}
        <AddNewAcceptableToken />
      </div>
    </div>
  );
}
