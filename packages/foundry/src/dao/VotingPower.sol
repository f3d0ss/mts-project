// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import { ERC721 } from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import { EIP712 } from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import { ERC721Votes } from "@openzeppelin/contracts/token/ERC721/extensions/ERC721Votes.sol";
import { ResturantToken } from "../ResturantToken.sol";
import { MTSController } from "../MTSController.sol";
import { IERC5192 } from "../IERC5192.sol";

contract VotingPower is ERC721, EIP712, ERC721Votes, IERC5192 {
    MTSController immutable CONTROLLER;

    mapping(address resturantAddress => mapping(uint256 resturantTokenId => uint256 votingTokenId))
        s_resturantTokenToVotingPower;

    uint256 private s_tokenIdCounter;

    error VotingPower__NotValidResturantAddress();
    error VotingPower__TokenNotYetUsed();
    error VotingPower__NotTheOwnerOfTheNFT();
    error VotingPower__AlreadyRedeemed();
    error VotingPower__VotingPowerTokenIsSoulbounded();
    error VotingPower__AddressesAndTokenIdsPerResturantDoNotMatch();

    modifier isTokenUsed(address resturant, uint256 resturantTokenId) {
        if (!ResturantToken(resturant).isTokenUsed(resturantTokenId)) revert VotingPower__TokenNotYetUsed();
        _;
    }

    modifier senderIsTokenOwner(address resturant, uint256 resturantTokenId) {
        if (ResturantToken(resturant).ownerOf(resturantTokenId) != _msgSender()) {
            revert VotingPower__NotTheOwnerOfTheNFT();
        }
        _;
    }

    modifier notAlreadyClaimed(address resturant, uint256 resturantTokenId) {
        if (s_resturantTokenToVotingPower[resturant][resturantTokenId] != 0) revert VotingPower__AlreadyRedeemed();
        _;
    }

    constructor(address _controller) ERC721("VotingPower", "VP") EIP712("VotingPower", "1") {
        // Start counter from one
        s_tokenIdCounter++;
        CONTROLLER = MTSController(_controller);
    }

    function gimmiMyPowers(address[] memory resturants, uint256[][] memory resturantTokenIdsPerResturant) external {
        if (resturants.length != resturantTokenIdsPerResturant.length) {
            revert VotingPower__AddressesAndTokenIdsPerResturantDoNotMatch();
        }
        uint256 numberOfResturants = resturantTokenIdsPerResturant.length;
        for (uint256 i = 0; i < numberOfResturants; i++) {
            address resturant = resturants[i];
            uint256[] memory resturantTokenIds = resturantTokenIdsPerResturant[i];
            if (!CONTROLLER.isResturantAddress(resturant)) revert VotingPower__NotValidResturantAddress();
            uint256 numberOfToken = resturantTokenIds.length;
            for (uint256 k = 0; k < numberOfToken; k++) {
                uint256 resturantTokenId = resturantTokenIds[i];
                _gimmiMyPower(resturant, resturantTokenId);
            }
        }
    }

    function gimmiMyPower(address resturant, uint256 resturantTokenId) external {
        if (!CONTROLLER.isResturantAddress(resturant)) revert VotingPower__NotValidResturantAddress();
        _gimmiMyPower(resturant, resturantTokenId);
    }

    function _gimmiMyPower(
        address resturant,
        uint256 resturantTokenId
    )
        internal
        isTokenUsed(resturant, resturantTokenId)
        senderIsTokenOwner(resturant, resturantTokenId)
        notAlreadyClaimed(resturant, resturantTokenId)
    {
        uint256 votingTokenId = s_tokenIdCounter++;
        s_resturantTokenToVotingPower[resturant][resturantTokenId] = votingTokenId;
        emit Locked(votingTokenId);
        _safeMint(_msgSender(), votingTokenId);
    }

    function _update(
        address to,
        uint256 tokenId,
        address auth
    )
        internal
        override(ERC721, ERC721Votes)
        returns (address)
    {
        // Revert if not minting
        if (_ownerOf(tokenId) != address(0) && to != address(0)) revert VotingPower__VotingPowerTokenIsSoulbounded();
        return super._update(to, tokenId, auth);
    }

    function _increaseBalance(address account, uint128 value) internal override(ERC721, ERC721Votes) {
        super._increaseBalance(account, value);
    }

    // Overrides IERC6372 functions to make the token & governor timestamp-based
    // https://docs.openzeppelin.com/contracts/4.x/governance#timestamp_based_governance

    function clock() public view override returns (uint48) {
        return uint48(block.timestamp);
    }

    // solhint-disable-next-line func-name-mixedcase
    function CLOCK_MODE() public pure override returns (string memory) {
        return "mode=timestamp";
    }

    /* ========================================================================== */
    /*                                   ERC-5192                                  */
    /* ========================================================================== */
    function locked(uint256 /* tokenId */ ) external pure returns (bool) {
        return true;
    }

    function supportsInterface(bytes4 interfaceId) public view override returns (bool) {
        // ERC-5192
        if (interfaceId == 0xb45a3c0e) return true;
        return super.supportsInterface(interfaceId);
    }
}
