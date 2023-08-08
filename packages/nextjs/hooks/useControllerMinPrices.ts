import { useEffect, useState } from "react";
import { useScaffoldEventHistory, useScaffoldEventSubscriber } from "./scaffold-eth";

type AcceptableTokenAddress = string;

export const useControllerMinPrices = () => {
  const [acceptableToken, setAcceptableTokens] = useState<Map<AcceptableTokenAddress, bigint>>(new Map());
  const [newAcceptableTokens, setNewAcceptableTokens] = useState<Map<AcceptableTokenAddress, bigint>>(new Map());

  useScaffoldEventSubscriber({
    contractName: "MTSController",
    eventName: "SetAcceptableMinPrice",
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    listener: logs => {
      const tmpNewAcceptableTokens = new Map(newAcceptableTokens);
      logs.forEach(log => {
        const { token, minPrice } = log.args;
        if (token && minPrice) {
          tmpNewAcceptableTokens.set(token, minPrice);
        }
      });
      setNewAcceptableTokens(tmpNewAcceptableTokens);
    },
  });

  const { data: addNewAcceptableTokenEvents } = useScaffoldEventHistory({
    contractName: "MTSController",
    eventName: "SetAcceptableMinPrice",
    fromBlock: process.env.NEXT_PUBLIC_DEPLOY_BLOCK ? BigInt(process.env.NEXT_PUBLIC_DEPLOY_BLOCK) : 0n,
    filters: {},
    blockData: true,
  });

  useEffect(() => {
    const acceptableTokenFromHistoryEvent = new Map<AcceptableTokenAddress, bigint>();
    for (const addNewAcceptableTokenEvent of addNewAcceptableTokenEvents?.reverse() || []) {
      const tokenAddress = addNewAcceptableTokenEvent["args"][0] as AcceptableTokenAddress;
      const minPrice = addNewAcceptableTokenEvent["args"][1] as bigint;
      acceptableTokenFromHistoryEvent.set(tokenAddress, minPrice);
    }
    setAcceptableTokens(acceptableTokenFromHistoryEvent);
  }, [addNewAcceptableTokenEvents]);

  const updatedAcceptableTokens = mergeMaps(acceptableToken, newAcceptableTokens);

  return updatedAcceptableTokens;
};

function mergeMaps<K, V>(map1: Map<K, V>, map2: Map<K, V>): Map<K, V> {
  const mergedMap = new Map(map1);
  for (const [key, value] of map2) {
    mergedMap.set(key, value);
  }
  return mergedMap;
}
