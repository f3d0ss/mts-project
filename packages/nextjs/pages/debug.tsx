import { useEffect, useState } from "react";
import type { NextPage } from "next";
import { useLocalStorage } from "usehooks-ts";
import { DebugResturantButton } from "~~/components/DebugResturantButton";
import { MetaHeader } from "~~/components/MetaHeader";
import { ResturantContractUI } from "~~/components/ResturantContractUI";
import { ContractUI } from "~~/components/scaffold-eth";
import { useScaffoldEventHistory, useScaffoldEventSubscriber } from "~~/hooks/scaffold-eth";
import { getContractNames } from "~~/utils/scaffold-eth/contractNames";

const selectedContractStorageKey = "scaffoldEth2.selectedContract";
const contractNames = getContractNames();

const Debug: NextPage = () => {
  const [resturants, setResturants] = useState<Set<bigint>>(new Set());
  const [newResturants, setNewResturants] = useState<Set<bigint>>(new Set());
  const [selectedContract, setSelectedContract] = useLocalStorage<string>(selectedContractStorageKey, contractNames[0]);

  useScaffoldEventSubscriber({
    contractName: "MTSController",
    eventName: "AddNewResturant",
    listener: logs => {
      logs.map(log => {
        const { id, resturantName } = log.args;
        if (id) {
          console.log(`AddedNewResturant: ${id}:${resturantName}`);
          setNewResturants(new Set(newResturants).add(id));
        }
      });
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
    const oldEventResturants = new Set<bigint>();
    for (const addNewResturantEvent of addNewResturantEvents || []) {
      console.log(addNewResturantEvent);
      const index = addNewResturantEvent.args.id as bigint;
      console.log("ADD NEW RESTURANT EVENT");
      console.log(addNewResturantEvent);

      oldEventResturants.add(index);
    }
    for (const removedResturantEvent of removedResturantEvents || []) {
      const index = removedResturantEvent.args.id as bigint;
      oldEventResturants.delete(index);
    }
    setResturants(oldEventResturants);
    console.log("USED EFFECT");
    console.log(oldEventResturants);
  }, [addNewResturantEvents, removedResturantEvents]);

  return (
    <>
      <MetaHeader
        title="Debug Contracts | Scaffold-ETH 2"
        description="Debug your deployed 🏗 Scaffold-ETH 2 contracts in an easy way"
      />
      <div className="flex flex-col gap-y-6 lg:gap-y-8 py-8 lg:py-12 justify-center items-center">
        {contractNames.length === 0 ? (
          <p className="text-3xl mt-14">No contracts found!</p>
        ) : (
          <>
            {contractNames.length >= 1 && (
              <div className="flex flex-row gap-2 w-full max-w-7xl pb-1 px-6 lg:px-10 flex-wrap">
                {contractNames.map(contractName => (
                  <button
                    className={`btn btn-secondary btn-sm normal-case font-thin ${
                      contractName === selectedContract ? "bg-base-300" : "bg-base-100"
                    }`}
                    key={contractName}
                    onClick={() => setSelectedContract(contractName)}
                  >
                    {contractName}
                  </button>
                ))}
                {[...new Set<bigint>([...resturants, ...newResturants])].map(resturantId => (
                  <DebugResturantButton
                    index={resturantId}
                    key={resturantId.toString()}
                    selectedContract={selectedContract}
                    onClick={(contractName: string) => setSelectedContract(contractName)}
                  />
                ))}
              </div>
            )}
            {contractNames.map(contractName => (
              <ContractUI
                key={contractName}
                contractName={contractName}
                className={contractName === selectedContract ? "" : "hidden"}
              />
            ))}
            {[...new Set<bigint>([...resturants, ...newResturants])].map(resturantId => (
              <ResturantContractUI
                index={resturantId}
                key={resturantId.toString()}
                selectedContract={selectedContract}
              />
            ))}
          </>
        )}
      </div>
      <div className="text-center mt-8 bg-secondary p-10">
        <h1 className="text-4xl my-0">Debug Contracts</h1>
        <p className="text-neutral">
          You can debug & interact with your deployed contracts here.
          <br /> Check{" "}
          <code className="italic bg-base-300 text-base font-bold [word-spacing:-0.5rem] px-1">
            packages / nextjs / pages / debug.tsx
          </code>{" "}
        </p>
      </div>
    </>
  );
};

export default Debug;
