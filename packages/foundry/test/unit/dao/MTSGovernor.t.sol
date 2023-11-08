// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { MTSGovernor } from "src/dao/MTSGovernor.sol";
import { IVotes } from "@openzeppelin/contracts/governance/extensions/GovernorVotes.sol";

contract MTSGovernorTest is PRBTest, StdCheats {
    uint256 constant INITIAL_VOTING_DELAY = 86_400;
    uint256 constant INITIAL_VOTING_PERIOD = 604_800;
    uint256 constant INITIAL_QUORUM = 0;
    uint256 constant INITIAL_PROPOSAL_THRESHOLD = 1;
    address votingToken;
    MTSGovernor governor;

    function setUp() public {
        votingToken = makeAddr("voting_token");
        vm.mockCall(votingToken, abi.encodeWithSignature("clock()"), abi.encode(uint48(block.timestamp)));
        governor = new MTSGovernor(IVotes(votingToken));
    }

    function test_constuctor() public {
        new MTSGovernor(IVotes(votingToken));
    }

    /* ========================================================================== */
    /*                               votingDelay                              */
    /* ========================================================================== */
    function test_votingDelay_returnVotingDelay() public {
        assertEq(governor.votingDelay(), INITIAL_VOTING_DELAY);
    }
    /* ========================================================================== */
    /*                                votingPeriod                                */
    /* ========================================================================== */

    function test_votingPeriod_returnVotingPeriod() public {
        assertEq(governor.votingPeriod(), INITIAL_VOTING_PERIOD);
    }
    /* ========================================================================== */
    /*                                   quorum                                   */
    /* ========================================================================== */

    function test_quorum_returnQuorum() public {
        vm.mockCall(votingToken, abi.encodeWithSelector(IVotes.getPastTotalSupply.selector, 0), abi.encode(100));
        vm.mockCall(votingToken, abi.encodeWithSelector(IVotes.getPastTotalSupply.selector, 0), abi.encode(100));
        assertEq(governor.quorum(0), INITIAL_QUORUM);
    }

    /* ========================================================================== */
    /*                              proposalThreshold                             */
    /* ========================================================================== */
    function test_proposalThreshold_returnProposalThreshold() public {
        assertEq(governor.proposalThreshold(), INITIAL_PROPOSAL_THRESHOLD);
    }
}
