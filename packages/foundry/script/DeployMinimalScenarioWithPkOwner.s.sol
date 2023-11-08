// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { BaseScript } from "./Base.s.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { DeployMTSController } from "./DeployMTSController.s.sol";
import { DeployResturantWithPkOwner } from "./DeployResturantWithPkOwner.s.sol";

contract DeployMinimalScenarioWithPkOwner is BaseScript {
    function run(
        address resturantOwner,
        string calldata resturantTokenName,
        string calldata resturantTokenSymbol
    )
        public
        exportDeployments
        returns (MTSController, ResturantToken)
    {
        uint256 ownerPk = vm.envUint("MTS_OWNER");
        address ownersAdddress = vm.addr(ownerPk);
        fundControllerOwner(ownersAdddress);
        MTSController controller = new DeployMTSController().run(ownersAdddress);
        ResturantToken resturantToken = new DeployResturantWithPkOwner().run(
            resturantOwner, resturantTokenName, resturantTokenSymbol, address(controller)
        );
        return (controller, resturantToken);
    }

    function fundControllerOwner(address controllerOwner) internal broadcast {
        (bool sent,) = controllerOwner.call{ value: 0.1 ether }("");
        require(sent, "Failed to send Ether");
    }
}
