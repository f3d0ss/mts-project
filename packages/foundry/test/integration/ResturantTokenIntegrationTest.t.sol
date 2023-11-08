// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { GasHelpers } from "test/util/GasHelpers.sol";
import { MTSController } from "src/MTSController.sol";
import { DeployMTSController } from "script/DeployMTSController.s.sol";
import { VerifySignature } from "src/lib/VerifySignature.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/ERC20Mock.sol";

contract ResturantTokenIntegrationTest is PRBTest, StdCheats, GasHelpers {
    address COTNROLLER_OWNER;
    address RESTURANT_OWNER;
    address USER_ADDRESS;
    uint256 USER_KEY;
    ERC20Mock paymentToken;
    string constant RESTURANT_NAME = "Rest Name";
    string constant RESTURANT_SYMBOL = "RN";
    uint256 constant INITIAL_CONTROLLER_FUND = 1 ether;
    uint256 constant INITIAL_ACCEPTABLE_PRICE = 1000;
    uint256 constant NFT_PRICE = 1 ether;
    uint32 constant RESERVATION_IN = 10_000;
    string constant NFT_URI = "ipfs://test";
    string constant REVIEW_URI = "ipfs://review";
    uint32 RESERVATION_DATE_TIMESTAMP;
    uint256 EXPIRATION_RANGE;

    MTSController controller;
    ResturantToken resturant;

    function setUp() public {
        COTNROLLER_OWNER = makeAddr("controller_owner");
        RESTURANT_OWNER = makeAddr("resturant_wallet");
        (USER_ADDRESS, USER_KEY) = makeAddrAndKey("user");
        paymentToken = new ERC20Mock();
        controller = new DeployMTSController().run(COTNROLLER_OWNER);
        vm.startPrank(COTNROLLER_OWNER);
        resturant = controller.addNewResturant(RESTURANT_OWNER, RESTURANT_NAME, RESTURANT_SYMBOL);
        controller.setAcceptableMinPrice(address(paymentToken), INITIAL_ACCEPTABLE_PRICE);
        vm.stopPrank();
        EXPIRATION_RANGE = resturant.EXPIRATION_RANGE();
        RESERVATION_DATE_TIMESTAMP = uint32(block.timestamp) + RESERVATION_IN;
    }

    function test_mintNewTokenOnSale() public {
        vm.startPrank(RESTURANT_OWNER);
        startMeasuringGas("Mint a new token on sale");
        resturant.safeMint(NFT_PRICE, address(paymentToken), RESERVATION_DATE_TIMESTAMP, NFT_URI);
        stopMeasuringGas();
        ResturantToken.NFT memory mintedNft = resturant.getNft(0);

        assertEq(mintedNft.price, 1 ether);
        assertEq(mintedNft.paymentToken, address(paymentToken));
        assertEq(mintedNft.locked, false);

        address mintedNftOwner = resturant.ownerOf(0);

        assertEq(mintedNftOwner, address(resturant));
    }

    modifier mintedTokenForSale() {
        vm.prank(RESTURANT_OWNER);
        resturant.safeMint(NFT_PRICE, address(paymentToken), RESERVATION_DATE_TIMESTAMP, NFT_URI);

        _;
    }

    function test_buyNFTWithApproval() public mintedTokenForSale {
        paymentToken.mint(USER_ADDRESS, NFT_PRICE);
        vm.startPrank(USER_ADDRESS);
        startMeasuringGas("Approve and buy an NFT");
        paymentToken.approve(address(resturant), NFT_PRICE);
        resturant.buyNFT(0);
        stopMeasuringGas();
        vm.stopPrank();
        assertEq(resturant.ownerOf(0), USER_ADDRESS);
    }

    function test_buyNFTWithoutApproval() public mintedTokenForSale {
        paymentToken.mint(USER_ADDRESS, NFT_PRICE);
        vm.startPrank(USER_ADDRESS);
        paymentToken.approve(address(resturant), NFT_PRICE);
        startMeasuringGas("Buy an NFT w/ approved token");
        resturant.buyNFT(0);
        stopMeasuringGas();
        vm.stopPrank();

        assertEq(resturant.ownerOf(0), USER_ADDRESS);
    }

    // function test_onlyApprove() public mintedTokenForSale {
    //     paymentToken.mint(USER_ADDRESS, NFT_PRICE);

    //     startMeasuringGas("Only approve");
    //     paymentToken.approve(address(resturant), NFT_PRICE);
    //     stopMeasuringGas();
    // }

    modifier soldToken() {
        paymentToken.mint(USER_ADDRESS, NFT_PRICE);
        vm.startPrank(USER_ADDRESS);
        paymentToken.approve(address(resturant), NFT_PRICE);
        resturant.buyNFT(0);
        vm.stopPrank();
        _;
    }

    function test_useTicket() public mintedTokenForSale soldToken {
        uint256 balanceResturantOwnerBeforeBurn = paymentToken.balanceOf(RESTURANT_OWNER);
        uint256 controllerFee = NFT_PRICE * controller.getFeeInBasePoint(address(paymentToken)) / 10_000;

        bytes32 messageHash = VerifySignature.getMessageHash(0, address(resturant));
        bytes32 digest = VerifySignature.getEthSignedMessageHash(messageHash);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(USER_KEY, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(RESTURANT_OWNER);
        startMeasuringGas("Use a token");
        resturant.useTicket(signature, 0);
        stopMeasuringGas();

        assertEq(paymentToken.balanceOf(RESTURANT_OWNER), balanceResturantOwnerBeforeBurn + NFT_PRICE - controllerFee);
    }

    modifier usedToken() {
        bytes32 messageHash = VerifySignature.getMessageHash(0, address(resturant));
        bytes32 digest = VerifySignature.getEthSignedMessageHash(messageHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(USER_KEY, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(RESTURANT_OWNER);
        resturant.useTicket(signature, 0);
        _;
    }

    function test_sendReview() public mintedTokenForSale soldToken usedToken {
        vm.startPrank(USER_ADDRESS);
        startMeasuringGas("Post a review for used token");
        resturant.sendReview(0, REVIEW_URI);
        stopMeasuringGas();
    }

    function test_burnNftNotSold() public mintedTokenForSale {
        vm.prank(RESTURANT_OWNER);
        startMeasuringGas("Burn NFT not sold");
        resturant.burnNftNotSold(0);
        stopMeasuringGas();
        assertEq(resturant.balanceOf(address(resturant)), 0);
    }

    function test_refund() public mintedTokenForSale soldToken {
        uint256 userBeforeRefund = paymentToken.balanceOf(USER_ADDRESS);
        vm.prank(RESTURANT_OWNER);
        startMeasuringGas("Refund NFT");
        resturant.refund(0);
        stopMeasuringGas();
        assertEq(paymentToken.balanceOf(USER_ADDRESS), userBeforeRefund + NFT_PRICE);
    }

    function test_burnTicketNotConsumed_sendFundToResturantOwner() public mintedTokenForSale soldToken {
        uint256 balanceResturantOwnerBeforeBurn = paymentToken.balanceOf(RESTURANT_OWNER);
        uint256 controllerFee = NFT_PRICE * controller.getFeeInBasePoint(address(paymentToken)) / 10_000;
        vm.warp(RESERVATION_DATE_TIMESTAMP + EXPIRATION_RANGE);
        vm.roll(block.number + 1);
        vm.prank(RESTURANT_OWNER);
        startMeasuringGas("Burn NFT not consumed");
        resturant.burnTicketNotConsumed(0);
        stopMeasuringGas();
        assertEq(paymentToken.balanceOf(address(resturant)), 0);
        assertEq(paymentToken.balanceOf(RESTURANT_OWNER), balanceResturantOwnerBeforeBurn + NFT_PRICE - controllerFee);
    }
}
