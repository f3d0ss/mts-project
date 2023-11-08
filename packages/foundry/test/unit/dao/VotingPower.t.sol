// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";
import { StdCheats } from "forge-std/StdCheats.sol";
import { VotingPower } from "src/dao/VotingPower.sol";
import { MTSControllerMock } from "../../mocks/MTSControllerMock.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { IERC721 } from "@openzeppelin/contracts/token/ERC721/IERC721.sol";

contract VotingPowerTest is PRBTest, StdCheats {
    address fakeResturantAddress;
    address resturantAddress;
    address tokenOwner;
    uint256 constant TOKEN_ID_NOT_USED = 1;
    uint256 constant TOKEN_ID_USED = 0;
    MTSControllerMock controller;
    VotingPower votingPower;

    function setUp() public {
        resturantAddress = makeAddr("resturant_address");
        fakeResturantAddress = makeAddr("fake_resturant_address");
        tokenOwner = makeAddr("token_owner");
        controller = new MTSControllerMock();

        vm.mockCall(
            address(controller),
            abi.encodeWithSelector(MTSController.isResturantAddress.selector, fakeResturantAddress),
            abi.encode(false)
        );
        vm.mockCall(
            address(controller),
            abi.encodeWithSelector(MTSController.isResturantAddress.selector, resturantAddress),
            abi.encode(true)
        );
        vm.mockCall(
            resturantAddress,
            abi.encodeWithSelector(ResturantToken.isTokenUsed.selector, TOKEN_ID_NOT_USED),
            abi.encode(false)
        );
        vm.mockCall(
            resturantAddress,
            abi.encodeWithSelector(ResturantToken.isTokenUsed.selector, TOKEN_ID_USED),
            abi.encode(true)
        );
        vm.mockCall(
            resturantAddress, abi.encodeWithSelector(IERC721.ownerOf.selector, TOKEN_ID_USED), abi.encode(tokenOwner)
        );

        votingPower = new VotingPower(address(controller));
    }

    function test_constuctor() public {
        new VotingPower(address(controller));
    }

    /* ========================================================================== */
    /*                                gimmiMyPower                               */
    /* ========================================================================== */
    function test_gimmiMyPower_mintNewVotingToken() public {
        vm.prank(tokenOwner);
        votingPower.gimmiMyPower(resturantAddress, TOKEN_ID_USED);
        assertEq(votingPower.balanceOf(tokenOwner), 1);
    }

    function test_gimmiMyPower_RevertWhen_notAResturantToken() public {
        vm.expectRevert(VotingPower.VotingPower__NotValidResturantAddress.selector);
        votingPower.gimmiMyPower(fakeResturantAddress, TOKEN_ID_USED);
    }

    function test_gimmiMyPower_RevertWhen_tokenNotYetUsed() public {
        vm.expectRevert(VotingPower.VotingPower__TokenNotYetUsed.selector);
        votingPower.gimmiMyPower(resturantAddress, TOKEN_ID_NOT_USED);
    }

    function test_gimmiMyPower_RevertWhen_senderNotTheTokenOwner() public {
        vm.expectRevert(VotingPower.VotingPower__NotTheOwnerOfTheNFT.selector);
        votingPower.gimmiMyPower(resturantAddress, TOKEN_ID_USED);
    }

    modifier claimToken(address _resturantAddress, uint256 _tokenId) {
        vm.prank(tokenOwner);
        votingPower.gimmiMyPower(resturantAddress, TOKEN_ID_USED);
        _;
    }

    function test_gimmiMyPower_RevertWhen_tokenAlreadyClaimed() public claimToken(resturantAddress, TOKEN_ID_USED) {
        vm.expectRevert(VotingPower.VotingPower__AlreadyRedeemed.selector);
        vm.prank(tokenOwner);
        votingPower.gimmiMyPower(resturantAddress, TOKEN_ID_USED);
    }

    /* ========================================================================== */
    /*                                gimmiMyPowers                               */
    /* ========================================================================== */
    function test_gimmiMyPowers_RevertWhen_resturantAndTokenIdsDoNotMatch() public {
        address[] memory resturantAddresses = new address[](2);
        resturantAddresses[0] = resturantAddress;
        resturantAddresses[1] = fakeResturantAddress;
        uint256[][] memory tokenIds = new uint256[][](1);
        tokenIds[0] = new uint256[](1);
        tokenIds[0][0] = TOKEN_ID_USED;

        vm.prank(tokenOwner);
        vm.expectRevert(VotingPower.VotingPower__AddressesAndTokenIdsPerResturantDoNotMatch.selector);
        votingPower.gimmiMyPowers(resturantAddresses, tokenIds);
    }

    function test_gimmiMyPowers_mintNewVotingTokens() public {
        address[] memory resturantAddresses = new address[](1);
        resturantAddresses[0] = resturantAddress;
        uint256[][] memory tokenIds = new uint256[][](1);
        tokenIds[0] = new uint256[](1);
        tokenIds[0][0] = TOKEN_ID_USED;
        vm.prank(tokenOwner);
        votingPower.gimmiMyPowers(resturantAddresses, tokenIds);
        assertEq(votingPower.balanceOf(tokenOwner), 1);
    }

    /* ========================================================================== */
    /*                                    clock                                   */
    /* ========================================================================== */
    function test_clock_returnClock() public {
        assertEq(votingPower.clock(), uint48(block.timestamp));
    }

    /* ========================================================================== */
    /*                                 CLOCK_MODE                                 */
    /* ========================================================================== */
    function test_CLOCK_MODE_returnCLOCKMODE() public {
        assertEq(votingPower.CLOCK_MODE(), "mode=timestamp");
    }

    /* ========================================================================== */
    /*                                   locked                                   */
    /* ========================================================================== */
    function test_lock_returnTrueAllTokenLocked() public {
        assertEq(votingPower.locked(0), true);
    }

    /* ========================================================================== */
    /*                              supportsInterface                             */
    /* ========================================================================== */
    function test_supportsInterface_supportERC5192() public {
        assertEq(votingPower.supportsInterface(0xb45a3c0e), true);
    }

    function test_supportsInterface_forwardCall() public {
        assertEq(votingPower.supportsInterface(0xffffffff), false);
    }
}
