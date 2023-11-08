// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { IMTSController } from "../../src/IMTSController.sol";

contract MTSControllerMock is IMTSController {
    mapping(address paymentToken => uint256 minimumPrice) private s_acceptableMinPrices;
    mapping(address paymentToken => uint256 basePointFees) s_basePointFees;

    constructor() { }

    function setAcceptableMinPrice(address paymentToken, uint256 minimumPrice) external {
        s_acceptableMinPrices[paymentToken] = minimumPrice;
    }

    function setBasePintFees(address paymentToken, uint256 basePointFees) external {
        s_basePointFees[paymentToken] = basePointFees;
    }

    function isPriceAcceptable(address paymentToken, uint256 price) external view override returns (bool) {
        if (s_acceptableMinPrices[paymentToken] == 0) return false;
        return price >= s_acceptableMinPrices[paymentToken];
    }

    function getFeeInBasePoint(address paymentToken) external view override returns (uint256) {
        return s_basePointFees[paymentToken];
    }
}
