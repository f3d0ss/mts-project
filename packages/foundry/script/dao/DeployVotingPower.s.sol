// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { BaseScript } from "../Base.s.sol";
import { VotingPower } from "src/dao/VotingPower.sol";

contract DeployVotingPower is BaseScript {
    function run(address controller) public exportDeployments broadcast returns (VotingPower) {
        VotingPower votingPower = new VotingPower(controller);
        return votingPower;
    }
}
