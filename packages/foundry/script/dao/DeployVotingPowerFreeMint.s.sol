// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { BaseScript } from "../Base.s.sol";
import { VotingPowerFreeMint } from "src/dao/VotingPowerFreeMint.sol";

contract DeployVotingPowerFreeMint is BaseScript {
    function run(address controller) public exportDeployments broadcast returns (VotingPowerFreeMint) {
        VotingPowerFreeMint votingPowerFreeMint = new VotingPowerFreeMint(controller);
        return votingPowerFreeMint;
    }
}
