// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { BaseScript } from "../Base.s.sol";
import { DeployVotingPowerFreeMint } from "./DeployVotingPowerFreeMint.s.sol";
import { DeployMTSGovernor } from "./DeployMTSGovernor.s.sol";
import { MTSGovernor } from "src/dao/MTSGovernor.sol";
import { VotingPowerFreeMint } from "src/dao/VotingPowerFreeMint.sol";

contract DeployDAO is BaseScript {
    function run(address controller) public exportDeployments returns (MTSGovernor, VotingPowerFreeMint) {
        DeployVotingPowerFreeMint deployerVotingPowerFreeMint = new DeployVotingPowerFreeMint();
        DeployMTSGovernor deployerMTSGovernor = new DeployMTSGovernor();
        VotingPowerFreeMint votingPowerFreeMint = deployerVotingPowerFreeMint.run(controller);
        MTSGovernor governor = deployerMTSGovernor.run(address(votingPowerFreeMint));
        return (governor, votingPowerFreeMint);
    }
}
