// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { VotingPower } from "./VotingPower.sol";

// Contract used just for testing purposes
contract VotingPowerFreeMint is VotingPower {
    constructor(address _controller) VotingPower(_controller) { }

    function mint(address to, uint256 tokenId) external {
        _safeMint(to, tokenId);
    }
}
