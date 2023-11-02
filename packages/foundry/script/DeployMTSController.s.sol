// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.19 <=0.9.0;

import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";

import { BaseScript } from "./Base.s.sol";

contract DeployMTSController is BaseScript {
    function run(address controllerOwner) public broadcast returns (MTSController) {
        ResturantToken resturantTokenImplementation = new ResturantToken();
        MTSController mtsController = new MTSController(controllerOwner, address(resturantTokenImplementation));

        return mtsController;
    }
}
