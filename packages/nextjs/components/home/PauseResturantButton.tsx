import React from "react";
import { useContractRead } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { useScaffoldContractWrite } from "~~/hooks/scaffold-eth";

type PauseResturantButtonProps = {
  index: bigint;
  resturantAddress: string;
};

export default function PauseResturantButton({ index, resturantAddress }: PauseResturantButtonProps) {
  //   const [functionToCall, setFunctionToCall] = useState<>("pauseResturant");
  const { data, isLoading: isLoadingFetch } = useContractRead({
    address: resturantAddress,
    abi: contracts.ResturantToken.abi,
    functionName: "paused",
    watch: true,
  });

  const functionToCall = data ? "unpauseResturant" : "pauseResturant";

  const { writeAsync: writePauseResturantAsync, isLoading: isLoadingPause } = useScaffoldContractWrite({
    contractName: "MTSController",
    functionName: functionToCall,
    args: [index],
    onBlockConfirmation: txnReceipt => {
      console.log("📦 Transaction blockHash", txnReceipt.blockHash);
    },
  });
  return (
    <button
      className={`btn btn-primary rounded-full capitalize font-normal font-white w-24 flex items-center gap-1 hover:gap-2 transition-all tracking-widest ${
        isLoadingPause || isLoadingFetch ? "loading" : ""
      }`}
      onClick={() => writePauseResturantAsync()}
    >
      {!isLoadingPause ? (data ? "Unpause" : "Pause") : ""}
    </button>
  );
}
