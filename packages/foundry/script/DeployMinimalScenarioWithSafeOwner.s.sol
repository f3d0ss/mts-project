// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { BaseScript } from "./Base.s.sol";
import { Safe } from "lib/safe-contracts/contracts/Safe.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { DeploySafe } from "./DeploySafe.s.sol";
import { DeployMTSController } from "./DeployMTSController.s.sol";
import { DeployResturantWithSafeOwner } from "./DeployResturantWithSafeOwner.s.sol";
import { SortKeys } from "./utils/SortKeys.sol";

contract DeployMinimalScenarioWithSafeOwner is BaseScript, SortKeys {
    function run(
        uint256 threshold,
        address resturantOwner,
        string calldata resturantTokenName,
        string calldata resturantTokenSymbol
    )
        public
        exportDeployments
        returns (Safe, MTSController, ResturantToken)
    {
        // function envOr(string calldata name, string calldata delim, uint256[] calldata defaultValue)

        uint256[] memory ownersPks = vm.envOr("SAFE_OWNERS_PKS", ",", new uint256[](0));
        address[] memory ownersAdddresses = new address[](ownersPks.length);
        for (uint256 i = 0; i < ownersPks.length; i++) {
            ownersAdddresses[i] = vm.addr(ownersPks[i]);
            fundControllerOwner(ownersAdddresses[i]);
        }
        Safe safe = new DeploySafe().run(ownersAdddresses, threshold);
        MTSController controller = new DeployMTSController().run(address(safe));
        ResturantToken resturantToken = new DeployResturantWithSafeOwner().run(
            resturantOwner, resturantTokenName, resturantTokenSymbol, address(controller), address(safe)
        );
        return (safe, controller, resturantToken);
    }

    function fundControllerOwner(address controllerOwner) internal broadcast {
        (bool sent,) = controllerOwner.call{ value: 0.1 ether }("");
        require(sent, "Failed to send Ether");
    }
}
