// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { BaseScript } from "../Base.s.sol";
import { DeployVotingPower } from "./DeployVotingPower.s.sol";
import { DeployMTSGovernor } from "./DeployMTSGovernor.s.sol";
import { MTSGovernor } from "src/dao/MTSGovernor.sol";
import { VotingPower } from "src/dao/VotingPower.sol";

contract DeployDAO is BaseScript {
    function run(address controller) public exportDeployments returns (MTSGovernor, VotingPower) {
        DeployVotingPower deployerVotingPower = new DeployVotingPower();
        DeployMTSGovernor deployerMTSGovernor = new DeployMTSGovernor();
        VotingPower votingPower = deployerVotingPower.run(controller);
        MTSGovernor governor = deployerMTSGovernor.run(address(votingPower));
        return (governor, votingPower);
    }
}
