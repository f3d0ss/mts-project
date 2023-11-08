// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { GasHelpers } from "test/util/GasHelpers.sol";
import { MTSController } from "src/MTSController.sol";
import { DeployMTSController } from "script/DeployMTSController.s.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/token/ERC20Mock.sol";

contract MTSControllerIntegrationTest is PRBTest, StdCheats, GasHelpers {
    address controllerOwner;
    address resturantOwner;
    ERC20Mock paymentToken;
    string constant RESTURANT_NAME = "Rest Name";
    string constant RESTURANT_SYMBOL = "RN";
    uint256 constant INITIAL_CONTROLLER_FUND = 1 ether;
    uint256 constant FUNDS_TO_WITHDRAW = 100;
    MTSController controller;
    DeployMTSController controllerDeployer;

    function setUp() public {
        controllerOwner = makeAddr("controller_owner");
        resturantOwner = makeAddr("resturant_owner");
        paymentToken = new ERC20Mock();
        controllerDeployer = new DeployMTSController();
        controller = controllerDeployer.run(controllerOwner);

        paymentToken.mint(address(controller), INITIAL_CONTROLLER_FUND);
    }

    function test_deploy() public {
        startMeasuringGas("Deploy controller");
        ResturantToken resturantTokenImplementation = new ResturantToken();
        new MTSController(controllerOwner, address(resturantTokenImplementation));
        stopMeasuringGas();
    }

    function test_addNewResturant() public {
        vm.startPrank(controllerOwner);
        startMeasuringGas("Add new resturant");
        controller.addNewResturant(resturantOwner, RESTURANT_NAME, RESTURANT_SYMBOL);
        stopMeasuringGas();
        assertEq(controller.getNumberOfResturants(), 1);
    }

    modifier withAResturantDeployed() {
        vm.prank(controllerOwner);
        controller.addNewResturant(resturantOwner, RESTURANT_NAME, RESTURANT_SYMBOL);
        _;
    }

    function test_pauseResturant() public withAResturantDeployed {
        vm.prank(controllerOwner);
        startMeasuringGas("Pause a resturant");
        controller.pauseResturant(0);
        stopMeasuringGas();
    }

    modifier withAResturantPaused(uint256 resturantId) {
        vm.prank(controllerOwner);
        controller.pauseResturant(resturantId);
        _;
    }

    function test_unpauseResturant() public withAResturantDeployed withAResturantPaused(0) {
        vm.prank(controllerOwner);
        startMeasuringGas("Unpause a resturant");
        controller.unpauseResturant(0);
        stopMeasuringGas();
    }

    function test_removeResturant() public withAResturantDeployed {
        vm.prank(controllerOwner);
        startMeasuringGas("Pause a resturant");
        controller.removeResturant(0);
        stopMeasuringGas();
        assertEq(controller.getResturantAddress(0), address(0));
    }

    function test_setAcceptableMinPrice() public {
        address paymentTokenAddress = address(paymentToken);
        vm.prank(controllerOwner);
        startMeasuringGas("Set acceptable min price");
        controller.setAcceptableMinPrice(paymentTokenAddress, 1 ether);
        stopMeasuringGas();
        assertEq(controller.getMinPrice(paymentTokenAddress), 1 ether);
        assertEq(controller.isPriceAcceptable(paymentTokenAddress, 1 ether), true);
    }

    modifier withAddedPaymentToken(address token) {
        vm.prank(controllerOwner);
        controller.setAcceptableMinPrice(token, 1 ether);
        _;
    }

    function test_removeAcceptableToken() public withAddedPaymentToken(address(paymentToken)) {
        vm.prank(controllerOwner);
        startMeasuringGas("Remove acceptable token");
        controller.removeAcceptableToken(address(paymentToken));
        stopMeasuringGas();
        assertEq(controller.isPriceAcceptable(address(paymentToken), 1), false);
    }

    function test_setBasePointFees() public {
        vm.prank(controllerOwner);
        startMeasuringGas("Set fee for a token");
        controller.setBasePointFees(address(paymentToken), 100);
        stopMeasuringGas();
        assertEq(controller.getFeeInBasePoint(address(paymentToken)), 100);
    }

    function test_withdrawOneTokenFunds() public {
        address[] memory tokensToWithdraw = new address[](1);
        tokensToWithdraw[0] = address(paymentToken);
        uint256[] memory amountOfTokens = new uint256[](1);
        amountOfTokens[0] = FUNDS_TO_WITHDRAW;
        vm.prank(controllerOwner);
        startMeasuringGas("Withdraw one token from funds");
        controller.withdrawFunds(tokensToWithdraw, amountOfTokens, controllerOwner);
        stopMeasuringGas();
        assertEq(paymentToken.balanceOf(controllerOwner), FUNDS_TO_WITHDRAW);
        assertEq(paymentToken.balanceOf(address(controller)), INITIAL_CONTROLLER_FUND - FUNDS_TO_WITHDRAW);
    }
}
