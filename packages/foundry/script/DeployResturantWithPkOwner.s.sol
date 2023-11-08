// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.18 <=0.9.0;

import { ResturantToken } from "src/ResturantToken.sol";
import { MTSController } from "src/MTSController.sol";
import { Safe, Enum } from "lib/safe-contracts/contracts/Safe.sol";
import { console2 } from "forge-std/console2.sol";
import { SortKeys } from "./utils/SortKeys.sol";

import { BaseScript } from "./Base.s.sol";

contract DeployResturantWithPkOwner is BaseScript, SortKeys {
    function run(
        address resturantOwner,
        string calldata name,
        string calldata symbol,
        address controllerAddress
    )
        public
        broadcast
        returns (ResturantToken)
    {
        // uint256 controllerOwner = vm.envUint("MTS_OWNER");
        // vm.startBroadcast(controllerOwner);
        return MTSController(controllerAddress).addNewResturant(resturantOwner, name, symbol);
    }
}
