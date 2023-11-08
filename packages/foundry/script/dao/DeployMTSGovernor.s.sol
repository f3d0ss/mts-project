// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { BaseScript } from "../Base.s.sol";
import { MTSGovernor, IVotes } from "src/dao/MTSGovernor.sol";

contract DeployMTSGovernor is BaseScript {
    function run(address voingPower) public exportDeployments broadcast returns (MTSGovernor) {
        MTSGovernor mtsGovernor = new MTSGovernor(IVotes(voingPower));
        return mtsGovernor;
    }
}
