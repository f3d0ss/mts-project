// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { BaseScript } from "./Base.s.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract DeployMockErc20 is BaseScript {
    function run() public broadcast exportDeployments returns (ERC20Mock) {
        ERC20Mock erc20 = new ERC20Mock();
        return erc20;
    }
}
