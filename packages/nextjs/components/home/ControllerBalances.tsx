import { useEffect, useState } from "react";
import ControllerBalanceInToken from "./ControllerBalanceInToken";
import { useScaffoldEventHistory, useScaffoldEventSubscriber } from "~~/hooks/scaffold-eth";

type ControllerBalancesProps = {
  controllerAddress: string;
};

export default function ControllerBalances({ controllerAddress }: ControllerBalancesProps) {
  const [newTokens, setNewTokens] = useState<Set<string>>(new Set());
  const [oldTokens, setOldTokens] = useState<Set<string>>(new Set());
  useScaffoldEventSubscriber({
    contractName: "MTSController",
    eventName: "SetAcceptableMinPrice",
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    listener: logs => {
      logs.forEach(log => {
        const { token, minPrice } = log.args;
        if (token) {
          console.log(`Set new acceptable min price: ${token}=${minPrice}`);
          setNewTokens(new Set(newTokens).add(token));
        }
      });
    },
  });

  const { data: setAcceptableMinPriceEvents } = useScaffoldEventHistory({
    contractName: "MTSController",
    eventName: "SetAcceptableMinPrice",
    fromBlock: process.env.NEXT_PUBLIC_DEPLOY_BLOCK ? BigInt(process.env.NEXT_PUBLIC_DEPLOY_BLOCK) : 0n,
    filters: {},
    blockData: true,
  });

  useEffect(() => {
    const oldAcceptableTokens = new Set<string>();

    for (const setAcceptableMinPriceEvent of setAcceptableMinPriceEvents || []) {
      console.log(setAcceptableMinPriceEvent);
      const tokenAddress = setAcceptableMinPriceEvent["args"][0] as string;

      oldAcceptableTokens.add(tokenAddress);
    }

    // for (const removedResturantEvent of removedResturantEvents || []) {
    //   const index = removedResturantEvent["args"][0].toNumber() as number;
    //   oldSetAcceptableMinPrice.delete(index);
    // }
    setOldTokens(oldAcceptableTokens);
  }, [setAcceptableMinPriceEvents]);

  const tokens = [...new Set<string>([...newTokens, ...oldTokens])];
  return (
    <>
      {tokens.map(tokenAddress => (
        <div className="flex gap-1 items-center" key={tokenAddress}>
          <ControllerBalanceInToken controllerAddress={controllerAddress} tokenAddress={tokenAddress} />
        </div>
      ))}
    </>
  );
}
