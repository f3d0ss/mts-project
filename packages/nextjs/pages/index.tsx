import type { NextPage } from "next";
import { MetaHeader } from "~~/components/MetaHeader";
import { Spinner } from "~~/components/Spinner";
import ControllerInfo from "~~/components/home/ControllerInfo";
import ControllerResturants from "~~/components/home/ControllerResturants";
import ControllerSettings from "~~/components/home/ControllerSettings";
import { useDeployedContractInfo, useScaffoldContractRead } from "~~/hooks/scaffold-eth";
import { useSafe } from "~~/hooks/useSafeRead";
import { getTargetNetwork } from "~~/utils/scaffold-eth";

const CONTROLLER_CONTRACT_NAME = "MTSController";

const Home: NextPage = () => {
  const { data: controllerContractData, isLoading: deployedContractLoading } =
    useDeployedContractInfo(CONTROLLER_CONTRACT_NAME);

  const configuredNetwork = getTargetNetwork();

  const { data: owner } = useScaffoldContractRead({
    contractName: CONTROLLER_CONTRACT_NAME,
    functionName: "owner",
  });
  const { isSafe } = useSafe(owner);

  if (!controllerContractData) {
    return <p className="text-3xl mt-14">{`No controller found on chain "${configuredNetwork.name}"!`}</p>;
  }
  return (
    <>
      <MetaHeader />
      {deployedContractLoading ? (
        <Spinner height="50px" width="50px" />
      ) : (
        <div className="flex flex-col gap-y-6 lg:gap-y-8 py-8 lg:py-12 justify-center items-center">
          <div className="flex items-center flex-col pt-10 w-full">
            <span>{isSafe ? "Is a Safe" : "Not a Safe"}</span>
          </div>
          <div className="col-span-5 grid grid-cols-1 lg:grid-cols-5 gap-8 lg:gap-10 w-full px-10">
            <ControllerInfo
              controllerName={CONTROLLER_CONTRACT_NAME}
              controllerAddress={controllerContractData.address}
            />
            <ControllerResturants />
            <ControllerSettings />
          </div>
        </div>
      )}
    </>
  );
};

export default Home;
