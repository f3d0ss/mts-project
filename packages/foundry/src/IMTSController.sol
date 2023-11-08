// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

interface IMTSController {
    function isPriceAcceptable(address paymentToken, uint256 price) external returns (bool);
    function getFeeInBasePoint(address paymentToken) external view returns (uint256);
}
