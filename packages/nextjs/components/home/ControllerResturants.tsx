import React from "react";
import AddNewResturant from "./AddNewResturant";
import ResturantRow from "./ResturantRow";
import { useControllerResturant } from "~~/hooks/useControllerResturants";

// type ControllerResturantsProps = {};

export default function ControllerResturants(/* {}: ControllerResturantsProps */) {
  const resturants = useControllerResturant();

  return (
    <div className="col-span-1 lg:col-span-2 flex flex-col gap-6">
      <div className="bg-base-100 p-2 rounded-3xl divide-y divide-base-300  shadow-md shadow-secondary border border-base-300 flex flex-col relative">
        <span className="font-bold p-2 text-3xl sm:text-3xl ">Resturants</span>
        {[...resturants].map(resturant => (
          <ResturantRow
            index={resturant.index}
            address={resturant.address}
            name={resturant.name}
            key={resturant.index.toString()}
          />
        ))}
        <AddNewResturant />
      </div>
    </div>
  );
}
