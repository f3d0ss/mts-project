// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { ResturantTokenMock } from "./mocks/ResturantTokenMock.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/ERC20Mock.sol";

import { Clones } from "@openzeppelin/contracts/proxy/Clones.sol";

contract MTSControllerTest is PRBTest, StdCheats {
    address CONTROLLER_OWNER_ADDRESS;
    uint256 CONTROLLER_OWNER_KEY;
    address RESTURANT_OWNER_ADDRESS;
    address SECOND_RESTURANT_OWNER_ADDRESS;
    address PAYMENT_TOKEN;
    uint256 constant MINIMUM_PAYMENT_TOKEN = 1 ether;
    uint256 constant PAYMENT_TOKEN_FEE = 100;
    uint256 constant MAX_BASE_POINT = 10_000;
    string constant RESTURANT_NAME = "RESTURANT Name";
    string constant RESTURANT_SYMBOL = "RN";
    address resturantOne;
    ResturantTokenMock resturantToken;
    MTSController controller;
    ERC20Mock nftPriceToken;

    function setUp() public {
        (CONTROLLER_OWNER_ADDRESS, CONTROLLER_OWNER_KEY) = makeAddrAndKey("controller_wallet");
        RESTURANT_OWNER_ADDRESS = makeAddr("resturant_owner");
        SECOND_RESTURANT_OWNER_ADDRESS = makeAddr("second_resturant_owner");
        PAYMENT_TOKEN = makeAddr("payment_token");
        resturantToken = new ResturantTokenMock();
        controller = new MTSController(CONTROLLER_OWNER_ADDRESS, address(resturantToken));
        nftPriceToken = new ERC20Mock();
    }

    function test_constuctor() public {
        new MTSController(CONTROLLER_OWNER_ADDRESS, address(resturantToken));
    }

    /* ========================================================================== */
    /*                               addNewResturant                              */
    /* ========================================================================== */
    function test_addNewResturant_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.addNewResturant(RESTURANT_OWNER_ADDRESS, RESTURANT_NAME, RESTURANT_SYMBOL);
    }

    function test_addNewResturant_canAdResturant() public {
        ResturantToken newResturantAddress;
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        newResturantAddress = controller.addNewResturant(RESTURANT_OWNER_ADDRESS, RESTURANT_NAME, RESTURANT_SYMBOL);
        assertEq(controller.isResturantAddress(address(newResturantAddress)), true);
    }

    function test_addNewResturant_RevertWhen_addMultipleResturantWithSameNameAndSymbol() public {
        vm.startPrank(CONTROLLER_OWNER_ADDRESS);
        controller.addNewResturant(RESTURANT_OWNER_ADDRESS, RESTURANT_NAME, RESTURANT_SYMBOL);
        vm.expectRevert();
        controller.addNewResturant(SECOND_RESTURANT_OWNER_ADDRESS, RESTURANT_NAME, RESTURANT_SYMBOL);
    }

    /* ========================================================================== */
    /*                               pauseResturant                               */
    /* ========================================================================== */

    function test_pauseResturant_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.pauseResturant(0);
    }

    function test_pauseResturant_RevertWhen_notExistingResturant() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectRevert();
        controller.pauseResturant(0);
    }

    modifier withAResturantDeployed() {
        ResturantToken newResturantAddress;
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        newResturantAddress = controller.addNewResturant(RESTURANT_OWNER_ADDRESS, RESTURANT_NAME, RESTURANT_SYMBOL);
        resturantOne = address(newResturantAddress);
        _;
    }

    function test_pauseResturant_pauseAResturant() public withAResturantDeployed {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectCall(resturantOne, abi.encodeCall(ResturantToken.pause, ()));
        controller.pauseResturant(0);
    }

    /* ========================================================================== */
    /*                              unpauseResturant                              */
    /* ========================================================================== */

    function test_unpauseResturant_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.unpauseResturant(0);
    }

    function test_unpauseResturant_RevertWhen_notExistingResturant() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectRevert();
        controller.unpauseResturant(0);
    }

    modifier withAResturantPaused(uint256 resturantId) {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        controller.pauseResturant(resturantId);
        _;
    }

    function test_unpauseResturant_pauseAResturant() public withAResturantDeployed withAResturantPaused(0) {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectCall(resturantOne, abi.encodeCall(ResturantToken.unpause, ()));
        controller.unpauseResturant(0);
    }

    /* ========================================================================== */
    /*                               removeResturant                              */
    /* ========================================================================== */
    function test_removeResturant_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.removeResturant(0);
    }

    function test_removeResturant_RevertWhen_notExistingResturant() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectRevert();
        controller.removeResturant(0);
    }

    function test_removeResturant_removeAResturant() public withAResturantDeployed {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        controller.removeResturant(0);
        assertEq(controller.getResturantAddress(0), address(0));
    }

    /* ========================================================================== */
    /*                            setAcceptableMinPrice                           */
    /* ========================================================================== */
    function test_setAcceptableMinPrice_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.setAcceptableMinPrice(PAYMENT_TOKEN, MINIMUM_PAYMENT_TOKEN);
    }

    function test_setAcceptableMinPrice_addAcceptableMinPrice() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        controller.setAcceptableMinPrice(PAYMENT_TOKEN, MINIMUM_PAYMENT_TOKEN);
        assertEq(controller.getMinPrice(PAYMENT_TOKEN), MINIMUM_PAYMENT_TOKEN);
        assertEq(controller.isPriceAcceptable(PAYMENT_TOKEN, MINIMUM_PAYMENT_TOKEN), true);
    }

    /* ========================================================================== */
    /*                            removeAcceptableToken                           */
    /* ========================================================================== */
    function test_removeAcceptableToken_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.removeAcceptableToken(PAYMENT_TOKEN);
    }

    modifier withAddedPaymentToken(address token) {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        controller.setAcceptableMinPrice(PAYMENT_TOKEN, MINIMUM_PAYMENT_TOKEN);
        _;
    }

    function test_removeAcceptableToken_removePaymentToken() public withAddedPaymentToken(PAYMENT_TOKEN) {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        controller.removeAcceptableToken(PAYMENT_TOKEN);
        assertEq(controller.isPriceAcceptable(PAYMENT_TOKEN, 1), false);
    }

    /* ========================================================================== */
    /*                               setBasePointFees                              */
    /* ========================================================================== */
    function test_setBasePointFees_RevertWhen_notTheOwner() public {
        vm.expectRevert("Ownable: caller is not the owner");
        controller.setBasePointFees(PAYMENT_TOKEN, PAYMENT_TOKEN_FEE);
    }

    function test_setBasePointFees_RevertWhen_feeGreaterThanOneundredPercent() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectRevert(MTSController.MTSController__FeeCantBeMoreThanOneundredPercent.selector);
        controller.setBasePointFees(PAYMENT_TOKEN, MAX_BASE_POINT);
    }

    function test_setBasePointFees_setTheBaseFee() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        controller.setBasePointFees(PAYMENT_TOKEN, PAYMENT_TOKEN_FEE);
        assertEq(controller.getFeeInBasePoint(PAYMENT_TOKEN), PAYMENT_TOKEN_FEE);
    }

    /* ========================================================================== */
    /*                             isResturantAddress                             */
    /* ========================================================================== */
    function test_isResturantAddress_returnFalseIfNotAResturant() public {
        assertEq(controller.isResturantAddress(CONTROLLER_OWNER_ADDRESS), false);
    }

    function test_isResturantAddress_returnFalseIfAddressZero() public {
        assertEq(controller.isResturantAddress(address(0)), false);
    }

    /* ========================================================================== */
    /*                            getNumberOfResturants                           */
    /* ========================================================================== */

    function test_getNumberOfResturants_returnNumberOfResturants() public {
        assertEq(controller.getNumberOfResturants(), 0);
    }

    /* ========================================================================== */
    /*                                withdrawFunds                               */
    /* ========================================================================== */
    function test_withdrawFunds_RevertWhen_notTheOwner() public {
        address[] memory tokensToWithdraw = new address[](1);
        tokensToWithdraw[0] = PAYMENT_TOKEN;
        uint256[] memory amountOfTokens = new uint256[](1);
        amountOfTokens[0] = PAYMENT_TOKEN_FEE;
        vm.expectRevert("Ownable: caller is not the owner");
        controller.withdrawFunds(tokensToWithdraw, amountOfTokens, CONTROLLER_OWNER_ADDRESS);
    }

    function test_withdrawFunds_RevertWhen_tokenAndAmountsDoNotMatch() public {
        address[] memory tokensToWithdraw = new address[](1);
        tokensToWithdraw[0] = PAYMENT_TOKEN;
        uint256[] memory amountOfTokens = new uint256[](0);
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        vm.expectRevert(MTSController.MTSController__TokensBalancesMismatch.selector);
        controller.withdrawFunds(tokensToWithdraw, amountOfTokens, CONTROLLER_OWNER_ADDRESS);
    }

    function test_withdrawFunds_canWithdrawFunds() public {
        vm.prank(CONTROLLER_OWNER_ADDRESS);
        address[] memory tokensToWithdraw = new address[](1);
        tokensToWithdraw[0] = address(nftPriceToken);
        uint256[] memory amountOfTokens = new uint256[](1);
        amountOfTokens[0] = 0;

        vm.expectCall(address(nftPriceToken), abi.encodeCall(nftPriceToken.transfer, (CONTROLLER_OWNER_ADDRESS, 0)));
        controller.withdrawFunds(tokensToWithdraw, amountOfTokens, CONTROLLER_OWNER_ADDRESS);
    }
}
