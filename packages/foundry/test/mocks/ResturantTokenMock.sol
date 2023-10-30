// Layout contract elements in the following order:
// Pragma statements
// Import statements
// Interfaces
// Libraries
// Contracts

// Inside each contract, library or interface, use the following order:
// Type declarations
// State variables
// Events
// Errors
// Modifiers
// Functions

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import { IERC721ReceiverUpgradeable } from
    "@openzeppelin/contracts-upgradeable/token/ERC721/IERC721ReceiverUpgradeable.sol";
import { ERC721EnumerableUpgradeable } from
    "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721EnumerableUpgradeable.sol";
import { OwnableUpgradeable } from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { Counters } from "@openzeppelin/contracts/utils/Counters.sol";
import { PausableUpgradeable } from "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import { VerifySignature } from "src/lib/VerifySignature.sol";
import { IMTSController } from "src/IMTSController.sol";
import { ERC721Upgradeable } from "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract ResturantTokenMock is
    Initializable,
    ERC721Upgradeable,
    ERC721EnumerableUpgradeable,
    PausableUpgradeable,
    OwnableUpgradeable,
    IERC721ReceiverUpgradeable
{
    using Counters for Counters.Counter;

    struct NFT {
        uint256 price;
        address paymentToken;
        uint32 reservationDate;
        bool locked;
        string reviewUri;
        string uri;
    }

    uint256 public constant EXPIRATION_RANGE = 24 * 60 * 60; // one day

    Counters.Counter private s_tokenIdCounter;

    mapping(uint256 => NFT) private s_nfts;

    IMTSController private s_mtsController;

    error ResturantToken__SenderIsNotTheController();

    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _owner,
        address _mtsController,
        string memory _name,
        string memory _symbol
    )
        public
        initializer
    {
        __ERC721_init(_name, _symbol);
        __ERC721Enumerable_init();
        __Pausable_init();
        __Ownable_init();
        s_mtsController = IMTSController(_mtsController);
        transferOwnership(_owner);
    }

    /* ========================================================================== */
    /*                                  Pausable                                  */
    /* ========================================================================== */
    function pause() public {
        _pause();
    }

    function unpause() public {
        _unpause();
    }

    /* ========================================================================== */
    /*         The following functions are overrides required by Solidity.        */
    /*                             From OP Wizzard                                */
    /* ========================================================================== */

    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC721EnumerableUpgradeable, ERC721Upgradeable)
        returns (bool)
    { }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 firstTokenId,
        uint256 batchSize
    )
        internal
        virtual
        override(ERC721EnumerableUpgradeable, ERC721Upgradeable)
    { }

    function onERC721Received(
        address operator,
        address from,
        uint256 tokenId,
        bytes calldata data
    )
        external
        override
        returns (bytes4)
    { }
}
