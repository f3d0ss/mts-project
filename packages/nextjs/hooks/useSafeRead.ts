import { useContractRead } from "wagmi";
import contracts from "~~/generated/usefulAbis";
import { getTargetNetwork } from "~~/utils/scaffold-eth";

/**
 * @dev wrapper for wagmi's useContractRead hook which loads in safe contract abi
 * @param config - The config settings, including extra wagmi configuration
 * @param config.contractName - deployed contract name
 * @param config.functionName - name of the function to be called
 * @param config.args - args to be passed to the function call
 */
export const useSafe = (address: string | undefined) => {
  const { data } = useContractRead({
    chainId: getTargetNetwork().id,
    functionName: "VERSION",
    address: address,
    abi: contracts.Safe.abi,
  });
  return {
    isSafe: data != undefined,
  };
};
