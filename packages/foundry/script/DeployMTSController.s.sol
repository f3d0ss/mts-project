// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.19 <=0.9.0;

import { MTSController } from "../src/MTSController.sol";

import { BaseScript } from "./Base.s.sol";

contract DeployMTSController is BaseScript {
    function run(address controllerOwner) public broadcast returns (MTSController) {
        MTSController mtsController = new MTSController(controllerOwner);

        return mtsController;
    }
}
