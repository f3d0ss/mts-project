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
export const useResturantContract = ({
  address,
  functionName,
  args,
}: {
  address: string | undefined;
  functionName: any;
  args?: any;
}) => {
  return useContractRead({
    chainId: getTargetNetwork().id,
    functionName,
    address: address,
    args,
    abi: contracts.ResturantToken.abi,
    watch: true,
  });
};
