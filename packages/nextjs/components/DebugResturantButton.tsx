import { Spinner } from "./Spinner";
import { useScaffoldContractRead } from "~~/hooks/scaffold-eth";
import { useResturantContract } from "~~/hooks/useResturantContract";

export const DebugResturantButton = ({
  index,
  selectedContract,
  onClick,
}: {
  index: bigint;
  selectedContract: string;
  onClick: (name: string) => void;
}) => {
  const { data: resturantAddress } = useScaffoldContractRead({
    contractName: "MTSController",
    functionName: "getResturantAddress",
    args: [index],
  });

  const { data: resturantName } = useResturantContract(resturantAddress, "name", undefined);

  console.log(`${resturantAddress} : ${resturantName}`);
  return (
    <button
      className={`btn btn-secondary btn-sm normal-case font-thin ${
        resturantName === selectedContract ? "bg-base-300" : "bg-base-100"
      }`}
      key={resturantName as string}
      onClick={() => onClick(resturantName as string)}
    >
      {resturantName ? (resturantName as string) : <Spinner width="20px" height="20px" />}
    </button>
  );
};
