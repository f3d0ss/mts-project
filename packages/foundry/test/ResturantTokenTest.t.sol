// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { MTSControllerMock } from "./mocks/MTSControllerMock.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import { VerifySignature } from "src/lib/VerifySignature.sol";
import { Clones } from "@openzeppelin/contracts/proxy/Clones.sol";

contract ResturantTokenTest is PRBTest, StdCheats {
    string constant TOKEN_NAME = "Token Name";
    string constant TOKEN_SYMBOL = "TN";
    string constant NFT_URI = "ipfs://test";
    string constant REVIEW_URI = "ipfs://review";
    uint32 constant RESERVATION_IN = 10_000;
    uint32 RESERVATION_DATE_TIMESTAMP;
    uint256 EXPIRATION_RANGE;
    address RESTURANT_OWNER_ADDRESS;
    uint256 RESTURANT_OWNER_KEY;
    address USER_ADDRESS;
    uint256 USER_KEY;
    uint256 constant NFT_PRICE = 1 ether;
    bytes4 constant erc5192Interface = 0xb45a3c0e;
    ERC20Mock nftPriceToken;
    ResturantToken resturantToken;
    MTSControllerMock controller;

    function setUp() public {
        (RESTURANT_OWNER_ADDRESS, RESTURANT_OWNER_KEY) = makeAddrAndKey("resturant_wallet");
        (USER_ADDRESS, USER_KEY) = makeAddrAndKey("user");
        controller = new MTSControllerMock();
        address resturantTokenImplementation = address(new ResturantToken());
        resturantToken = ResturantToken(Clones.clone(resturantTokenImplementation));
        resturantToken.initialize(RESTURANT_OWNER_ADDRESS, address(controller), TOKEN_NAME, TOKEN_SYMBOL);
        nftPriceToken = new ERC20Mock();
        EXPIRATION_RANGE = resturantToken.EXPIRATION_RANGE();
        RESERVATION_DATE_TIMESTAMP = uint32(block.timestamp) + RESERVATION_IN;
    }

    function test_initialize_canInitialize() public {
        address resturantTokenImplementation = address(new ResturantToken());
        resturantToken = ResturantToken(Clones.clone(resturantTokenImplementation));
        resturantToken.initialize(RESTURANT_OWNER_ADDRESS, address(controller), TOKEN_NAME, TOKEN_SYMBOL);
    }

    function test_safeMint_RevertWhen_calledByNotResturantOwner() public {
        vm.expectRevert();
        resturantToken.safeMint(NFT_PRICE, address(nftPriceToken), RESERVATION_DATE_TIMESTAMP, NFT_URI);
    }

    function test_safeMint_RevertWhen_mintWithNotApprovedToken() public {
        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__PriceNotAcceptable.selector);
        resturantToken.safeMint(NFT_PRICE, address(nftPriceToken), RESERVATION_DATE_TIMESTAMP, NFT_URI);
    }

    function test_safeMint_canMintToken() public {
        controller.setAcceptableMinPrice(address(nftPriceToken), NFT_PRICE);

        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.safeMint(NFT_PRICE, address(nftPriceToken), RESERVATION_DATE_TIMESTAMP, NFT_URI);

        ResturantToken.NFT memory mintedNft = resturantToken.getNft(0);

        assertEq(mintedNft.price, 1 ether);
        assertEq(mintedNft.paymentToken, address(nftPriceToken));
        assertEq(mintedNft.locked, false);

        address mintedNftOwner = resturantToken.ownerOf(0);

        assertEq(mintedNftOwner, address(resturantToken));
    }

    /* ========================================================================== */
    /*                                   buyNFT                                   */
    /* ========================================================================== */

    function test_buyNFT_RevertWhen_buyANotExistingToken() public {
        vm.expectRevert(ResturantToken.ResturantToken__TokenNotForSale.selector);
        resturantToken.buyNFT(0);
    }

    modifier mintedTokenForSale() {
        controller.setAcceptableMinPrice(address(nftPriceToken), NFT_PRICE);

        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.safeMint(NFT_PRICE, address(nftPriceToken), RESERVATION_DATE_TIMESTAMP, NFT_URI);

        _;
    }

    function test_buyNFT_RevertWhen_userDoNotHaveFundsToPay() public mintedTokenForSale {
        nftPriceToken.approve(address(resturantToken), NFT_PRICE);
        vm.expectRevert();
        resturantToken.buyNFT(0);
    }

    function test_buyNFT_canBuyTheNft() public mintedTokenForSale {
        nftPriceToken.mint(USER_ADDRESS, NFT_PRICE);
        vm.startPrank(USER_ADDRESS);
        nftPriceToken.approve(address(resturantToken), NFT_PRICE);

        resturantToken.buyNFT(0);
        vm.stopPrank();

        assertEq(resturantToken.ownerOf(0), USER_ADDRESS);
    }

    /* ========================================================================== */
    /*                                  useTicket                                 */
    /* ========================================================================== */

    function test_useTicket_RevertWhen_calledByNotResturantOwner() public {
        vm.expectRevert();
        resturantToken.useTicket("", 0);
    }

    function test_useTicket_RevertWhen_ticketForATokenNotSold() public {
        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__TokenNotSold.selector);
        resturantToken.useTicket("", 0);
    }

    modifier soldToken() {
        nftPriceToken.mint(USER_ADDRESS, NFT_PRICE);
        vm.startPrank(USER_ADDRESS);
        nftPriceToken.approve(address(resturantToken), NFT_PRICE);
        resturantToken.buyNFT(0);
        vm.stopPrank();
        _;
    }

    function test_useTicket_RevertWhen_invalidSignature() public mintedTokenForSale soldToken {
        bytes32 messageHash = VerifySignature.getMessageHash(0, RESTURANT_OWNER_ADDRESS);

        bytes32 digest = keccak256(abi.encodePacked("\x19Ethereum Signed Message:\n32", messageHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(RESTURANT_OWNER_KEY, digest);
        bytes memory falseSignature = abi.encodePacked(r, s, v);
        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__InvalidSignatureFromClient.selector);
        resturantToken.useTicket(falseSignature, 0);
    }

    function test_useTicket_canUseTicketWithValidSignature() public mintedTokenForSale soldToken {
        uint256 balanceResturantOwnerBeforeBurn = nftPriceToken.balanceOf(RESTURANT_OWNER_ADDRESS);
        uint256 controllerFee = NFT_PRICE * controller.getFeeInBasePoint(address(nftPriceToken)) / 10_000;

        // bytes32 tmpMessageHash = VerifySignature.getMessageHash(6, 0xBE6566b6d79dF8EA95b2619478019333aA758BE8);
        // console2.logBytes32(tmpMessageHash);
        // bytes32 tmpDigest = VerifySignature.getEthSignedMessageHash(tmpMessageHash);
        // console2.logBytes32(tmpDigest);
        // (uint8 tmpV, bytes32 tmpR, bytes32 tmpS) = vm.sign(
        //     114_655_236_757_964_383_808_236_220_317_196_289_954_524_722_153_601_011_776_311_909_915_248_731_575_136,
        //     tmpDigest
        // );
        // bytes memory tmpSignature = abi.encodePacked(tmpR, tmpS, tmpV);
        // console2.logBytes(tmpSignature);

        bytes32 messageHash = VerifySignature.getMessageHash(0, address(resturantToken));
        bytes32 digest = VerifySignature.getEthSignedMessageHash(messageHash);

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(USER_KEY, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.useTicket(signature, 0);
        assertEq(
            nftPriceToken.balanceOf(RESTURANT_OWNER_ADDRESS),
            balanceResturantOwnerBeforeBurn + NFT_PRICE - controllerFee
        );
    }

    modifier usedToken() {
        bytes32 messageHash = VerifySignature.getMessageHash(0, address(resturantToken));
        bytes32 digest = VerifySignature.getEthSignedMessageHash(messageHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(USER_KEY, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.useTicket(signature, 0);
        _;
    }

    function test_useTicket_RevertWhen_tokenAlreadyUsed() public mintedTokenForSale soldToken usedToken {
        bytes32 messageHash = VerifySignature.getMessageHash(0, address(resturantToken));
        bytes32 digest = VerifySignature.getEthSignedMessageHash(messageHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(USER_KEY, digest);
        bytes memory signature = abi.encodePacked(r, s, v);

        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__TokenAlreadyUsed.selector);
        resturantToken.useTicket(signature, 0);
    }

    /* ========================================================================== */
    /*                                 sendReview                                 */
    /* ========================================================================== */

    function test_sendReview_RevertWhen_isNotOwner() public mintedTokenForSale soldToken usedToken {
        vm.expectRevert();
        resturantToken.sendReview(0, REVIEW_URI);
    }

    function test_sendReview_canAddReview() public mintedTokenForSale soldToken usedToken {
        vm.startPrank(USER_ADDRESS);
        resturantToken.sendReview(0, REVIEW_URI);
    }

    /* ========================================================================== */
    /*                               burnNftNotSold                               */
    /* ========================================================================== */
    function test_burnNftNotSold_RevertWhen_isNotOwner() public mintedTokenForSale {
        vm.expectRevert();
        resturantToken.burnNftNotSold(0);
    }

    function test_burnNftNotSold_RevertWhen_tokenAlreadySold() public mintedTokenForSale soldToken {
        vm.expectRevert();
        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.burnNftNotSold(0);
    }

    function test_burnNftNotSold_canBurnTokenNotSold() public mintedTokenForSale {
        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.burnNftNotSold(0);
        assertEq(resturantToken.balanceOf(address(resturantToken)), 0);
    }

    /* ========================================================================== */
    /*                                   refund                                   */
    /* ========================================================================== */
    function test_refund_RevertWhen_isNotOwner() public mintedTokenForSale soldToken {
        vm.expectRevert();
        resturantToken.refund(0);
    }

    function test_refund_RevertWhen_tokenIsNotSold() public mintedTokenForSale {
        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__TokenNotSold.selector);
        resturantToken.refund(0);
    }

    function test_refund_canRefund() public mintedTokenForSale soldToken {
        uint256 userBeforeRefund = nftPriceToken.balanceOf(USER_ADDRESS);
        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.refund(0);
        assertEq(nftPriceToken.balanceOf(USER_ADDRESS), userBeforeRefund + NFT_PRICE);
    }

    /* ========================================================================== */
    /*                            burnTicketNotConsumed                           */
    /* ========================================================================== */

    function test_burnTicketNotConsumed_RevertWhen_isNotOwner() public mintedTokenForSale soldToken {
        vm.expectRevert();
        resturantToken.burnTicketNotConsumed(0);
    }

    function test_burnTicketNotConsumed_RevertWhen_isNotSold() public mintedTokenForSale {
        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__TokenNotSold.selector);
        resturantToken.burnTicketNotConsumed(0);
    }

    function test_burnTicketNotConsumed_RevertWhen_tokenNotExpired() public mintedTokenForSale soldToken {
        vm.warp(RESERVATION_DATE_TIMESTAMP);
        vm.roll(block.number + 1);
        vm.prank(RESTURANT_OWNER_ADDRESS);
        vm.expectRevert(ResturantToken.ResturantToken__TokenNotYetBurnable.selector);
        resturantToken.burnTicketNotConsumed(0);
    }

    function test_burnTicketNotConsumed_sendFundToResturantOwner() public mintedTokenForSale soldToken {
        uint256 balanceResturantOwnerBeforeBurn = nftPriceToken.balanceOf(RESTURANT_OWNER_ADDRESS);
        uint256 controllerFee = NFT_PRICE * controller.getFeeInBasePoint(address(nftPriceToken)) / 10_000;
        vm.warp(RESERVATION_DATE_TIMESTAMP + EXPIRATION_RANGE);
        vm.roll(block.number + 1);
        vm.prank(RESTURANT_OWNER_ADDRESS);
        resturantToken.burnTicketNotConsumed(0);
        assertEq(nftPriceToken.balanceOf(address(resturantToken)), 0);
        assertEq(
            nftPriceToken.balanceOf(RESTURANT_OWNER_ADDRESS),
            balanceResturantOwnerBeforeBurn + NFT_PRICE - controllerFee
        );
    }

    /* ========================================================================== */
    /*                              getControllerAddress                              */
    /* ========================================================================== */
    function test_getControllerAddress_retrunTheAddressSetInTheConstructor() public {
        address returnGetControllerAddress = resturantToken.getControllerAddress();
        assertEq(returnGetControllerAddress, address(controller));
    }

    /* ========================================================================== */
    /*                              getCounter                              */
    /* ========================================================================== */
    function test_getCounter_retrunTheAddressSetInTheConstructor() public {
        uint256 returnGetCounter = resturantToken.getCounter();
        assertEq(returnGetCounter, 0);
    }

    /* ========================================================================== */
    /*                              isTokenUsed                              */
    /* ========================================================================== */
    function test_isTokenUsed_retrunTheAddressSetInTheConstructor() public mintedTokenForSale {
        bool returnIsTokenUsed = resturantToken.isTokenUsed(0);
        assertEq(returnIsTokenUsed, false);
    }

    /* ========================================================================== */
    /*                                   locked                                   */
    /* ========================================================================== */
    function test_locked_returnFalseIfTokenIsNotUsed() public mintedTokenForSale {
        assertEq(resturantToken.locked(0), false);
    }

    function test_locked_returnTrueIfTokenIsUsed() public mintedTokenForSale soldToken usedToken {
        assertEq(resturantToken.locked(0), true);
    }

    /* ========================================================================== */
    /*                                    pause                                   */
    /* ========================================================================== */
    function test_pause_RevertWhen_notTheController() public {
        vm.expectRevert(ResturantToken.ResturantToken__SenderIsNotTheController.selector);
        resturantToken.pause();
    }

    function test_pause_canBePaused() public {
        vm.prank(address(controller));
        resturantToken.pause();
    }

    /* ========================================================================== */
    /*                                    unpause                                   */
    /* ========================================================================== */
    function test_unpause_RevertWhen_notTheController() public {
        vm.expectRevert(ResturantToken.ResturantToken__SenderIsNotTheController.selector);
        resturantToken.unpause();
    }

    function test_unpause_canBeUnpaused() public {
        vm.startPrank(address(controller));
        resturantToken.pause();
        resturantToken.unpause();
        vm.stopPrank();
    }

    /* ========================================================================== */
    /*                                  tokenURI                                  */
    /* ========================================================================== */
    function test_tokenURI_returnTheTokenURISetInMint() public mintedTokenForSale {
        assertEq(resturantToken.tokenURI(0), NFT_URI);
    }

    /* ========================================================================== */
    /*                              supportsInterface                             */
    /* ========================================================================== */
    function test_supportsInterface_supportERC5192() public mintedTokenForSale {
        assertEq(resturantToken.supportsInterface(erc5192Interface), true);
    }

    function test_supportsInterface_forwardCall() public mintedTokenForSale {
        assertEq(resturantToken.supportsInterface(0xffffffff), false);
    }
}
