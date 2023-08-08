import { useEffect, useState } from "react";
import { useScaffoldEventHistory, useScaffoldEventSubscriber } from "./scaffold-eth";

type Resturant = {
  index: bigint;
  name: string;
  address: string;
};

export const useControllerResturant = () => {
  const [resturants, setResturants] = useState<Map<bigint, Resturant>>(new Map());
  const [newResturants, setNewResturants] = useState<Map<bigint, Resturant>>(new Map());
  const [newRemovedResturants, setNewRemovedResturants] = useState<Set<bigint>>(new Set());

  useScaffoldEventSubscriber({
    contractName: "MTSController",
    eventName: "AddNewResturant",
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    listener: logs => {
      const tmpNewResturants = new Map(newResturants);
      logs.forEach(log => {
        const { id, newResturantAddress, resturantName } = log.args;
        if (id && newResturantAddress && resturantName) {
          const newResturant: Resturant = { index: id, name: resturantName, address: newResturantAddress };
          tmpNewResturants.set(id, newResturant);
        }
      });
      setNewResturants(tmpNewResturants);
    },
  });
  useScaffoldEventSubscriber({
    contractName: "MTSController",
    eventName: "RemovedResturant",
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    listener: logs => {
      const tmpRemovedResturants = new Set(newRemovedResturants);
      logs.forEach(log => {
        const { id } = log.args;
        if (id) {
          tmpRemovedResturants.add(id);
        }
      });
      setNewRemovedResturants(tmpRemovedResturants);
    },
  });

  const { data: addNewResturantEvents } = useScaffoldEventHistory({
    contractName: "MTSController",
    eventName: "AddNewResturant",
    fromBlock: process.env.NEXT_PUBLIC_DEPLOY_BLOCK ? BigInt(process.env.NEXT_PUBLIC_DEPLOY_BLOCK) : 0n,
    filters: {},
    blockData: true,
  });

  const { data: removedResturantEvents } = useScaffoldEventHistory({
    contractName: "MTSController",
    eventName: "RemovedResturant",
    fromBlock: process.env.NEXT_PUBLIC_DEPLOY_BLOCK ? BigInt(process.env.NEXT_PUBLIC_DEPLOY_BLOCK) : 0n,
    filters: {},
    blockData: true,
  });

  useEffect(() => {
    const resturantsFromHistoryEvent = new Map<bigint, Resturant>();
    for (const addNewResturantEvent of addNewResturantEvents || []) {
      const index = addNewResturantEvent.args.id as bigint;
      const address = addNewResturantEvent.args.newResturantAddress as string;
      const name = addNewResturantEvent.args.resturantName as string;
      const resturant = { index, name, address };
      resturantsFromHistoryEvent.set(index, resturant);
    }
    for (const removedResturantEvent of removedResturantEvents || []) {
      const index = removedResturantEvent.args.id as bigint;
      resturantsFromHistoryEvent.delete(index);
    }
    setResturants(resturantsFromHistoryEvent);
  }, [addNewResturantEvents, removedResturantEvents]);
  //   const resturants = resturantsFromHistoryEvent;
  const updatedResturants = mergeMaps(resturants, newResturants);
  newRemovedResturants.forEach(index => updatedResturants.delete(index));
  return updatedResturants.values();
};

function mergeMaps<K, V>(map1: Map<K, V>, map2: Map<K, V>): Map<K, V> {
  const mergedMap = new Map(map1);
  for (const [key, value] of map2) {
    mergedMap.set(key, value);
  }
  return mergedMap;
}
