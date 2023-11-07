// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { Ownable } from "@openzeppelin/contracts/access/Ownable.sol";
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IMTSController } from "./IMTSController.sol";
import { ResturantToken } from "./ResturantToken.sol";
import { Clones } from "@openzeppelin/contracts/proxy/Clones.sol";

contract MTSController is IMTSController, Ownable {
    uint256 private constant MAX_BASE_POINT = 10_000;
    mapping(address paymentToken => uint256 minimumPrice) private s_acceptableMinPrices;
    mapping(address paymentToken => uint256 basePointFees) private s_basePointFees;
    address[] private s_resturantAddresses;
    // Can be mutable if want to change resturant implementation
    address immutable s_resturantImplementation;

    event AddNewResturant(uint256 indexed id, address newResturantAddress, string resturantName);
    event RemovedResturant(uint256 indexed id, address resturantAddress, string resturantName);
    event PausedResturant(uint256 indexed id, address resturantAddress, string resturantName);
    event UnpausedResturant(uint256 indexed id, address resturantAddress, string resturantName);
    event SetAcceptableMinPrice(address indexed token, uint256 minPrice);

    error MTSController__UnacceptableToken();
    error MTSController__CannotTransferToken();
    error MTSController__TokensBalancesMismatch();
    error MTSController__FeeCantBeMoreThanOneundredPercent();
    error MTSController__MinimumPriceCannotBeZero();

    constructor(address _controllerOwner, address _resturantImplemetnation) {
        transferOwnership(_controllerOwner);
        s_resturantImplementation = _resturantImplemetnation;
    }

    function addNewResturant(
        address _resturantOwner,
        string memory _name,
        string memory _symbol
    )
        external
        onlyOwner
        returns (ResturantToken)
    {
        bytes32 salt = keccak256(abi.encode(_name, _symbol));
        address newProxy = Clones.cloneDeterministic(s_resturantImplementation, salt);
        ResturantToken newResturant = ResturantToken(newProxy);
        newResturant.initialize(_resturantOwner, address(this), _name, _symbol);
        s_resturantAddresses.push(address(newResturant));
        emit AddNewResturant(s_resturantAddresses.length - 1, address(newResturant), _name);
        return newResturant;
    }

    function pauseResturant(uint256 index) external onlyOwner {
        ResturantToken resturant = ResturantToken(s_resturantAddresses[index]);
        resturant.pause();
        emit PausedResturant(index, address(resturant), resturant.name());
    }

    function unpauseResturant(uint256 index) external onlyOwner {
        ResturantToken resturant = ResturantToken(s_resturantAddresses[index]);
        resturant.unpause();
        emit UnpausedResturant(index, address(resturant), resturant.name());
    }

    function removeResturant(uint256 index) external onlyOwner {
        ResturantToken resturant = ResturantToken(s_resturantAddresses[index]);
        emit RemovedResturant(index, address(resturant), resturant.name());
        s_resturantAddresses[index] = address(0);
    }

    function setAcceptableMinPrice(address paymentToken, uint256 minimumPrice) external onlyOwner {
        if (minimumPrice == 0) revert MTSController__MinimumPriceCannotBeZero();
        s_acceptableMinPrices[paymentToken] = minimumPrice;
        emit SetAcceptableMinPrice(paymentToken, minimumPrice);
    }

    function removeAcceptableToken(address paymentToken) external onlyOwner {
        delete s_acceptableMinPrices[paymentToken];
        emit SetAcceptableMinPrice(paymentToken, 0);
    }

    function setBasePointFees(address paymentToken, uint256 basePointFees) external onlyOwner {
        if (basePointFees >= MAX_BASE_POINT) revert MTSController__FeeCantBeMoreThanOneundredPercent();

        s_basePointFees[paymentToken] = basePointFees;
    }

    function isPriceAcceptable(address paymentToken, uint256 price) external view override returns (bool) {
        if (s_acceptableMinPrices[paymentToken] == 0) return false;
        return price >= s_acceptableMinPrices[paymentToken];
    }

    function getMinPrice(address paymentToken) external view returns (uint256) {
        return s_acceptableMinPrices[paymentToken];
    }

    function getFeeInBasePoint(address paymentToken) external view override returns (uint256) {
        return s_basePointFees[paymentToken];
    }

    function getResturantAddress(uint256 index) public view returns (address) {
        return s_resturantAddresses[index];
    }

    function isResturantAddress(address resturantAddress) public view returns (bool) {
        if (resturantAddress == address(0)) return false;
        uint256 numberOfRestturants = s_resturantAddresses.length;
        for (uint256 i = 0; i < numberOfRestturants; i++) {
            if (s_resturantAddresses[i] == resturantAddress) return true;
        }
        return false;
    }

    function getNumberOfResturants() public view returns (uint256) {
        return s_resturantAddresses.length;
    }

    function withdrawFunds(address[] calldata tokens, uint256[] calldata balances, address to) external onlyOwner {
        if (tokens.length != balances.length) {
            revert MTSController__TokensBalancesMismatch();
        }
        uint256 numberOfTokens = tokens.length;
        for (uint256 i = 0; i < numberOfTokens;) {
            bool success = IERC20(tokens[i]).transfer(to, balances[i]);
            if (!success) {
                revert MTSController__CannotTransferToken();
            }
            unchecked {
                i++;
            }
        }
    }
}
