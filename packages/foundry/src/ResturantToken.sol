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
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC721ReceiverUpgradeable } from
    "@openzeppelin/contracts-upgradeable/token/ERC721/IERC721ReceiverUpgradeable.sol";
import { ERC721EnumerableUpgradeable } from
    "@openzeppelin/contracts-upgradeable/token/ERC721/extensions/ERC721EnumerableUpgradeable.sol";
import { OwnableUpgradeable } from "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import { Counters } from "@openzeppelin/contracts/utils/Counters.sol";
import { PausableUpgradeable } from "@openzeppelin/contracts-upgradeable/security/PausableUpgradeable.sol";
import { IERC5192 } from "./IERC5192.sol";
import { VerifySignature } from "./lib/VerifySignature.sol";
import { IMTSController } from "./IMTSController.sol";
import { ERC721Upgradeable } from "@openzeppelin/contracts-upgradeable/token/ERC721/ERC721Upgradeable.sol";
import { Initializable } from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";

contract ResturantToken is
    Initializable,
    ERC721Upgradeable,
    ERC721EnumerableUpgradeable,
    PausableUpgradeable,
    OwnableUpgradeable,
    IERC721ReceiverUpgradeable,
    IERC5192
{
    using Counters for Counters.Counter;
    using SafeERC20 for IERC20;

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

    event ReviewPosted(uint256 indexed tokenId, string reviewURI);

    error ResturantToken__PriceNotAcceptable();
    error ResturantToken__TokenNotForSale();
    error ResturantToken__TransferFailed();
    error ResturantToken__InvalidSignatureFromClient();
    error ResturantToken__TokenNotSold();
    error ResturantToken__TokenAlreadyUsed();
    error ResturantToken__TokenNotUsed();
    error ResturantToken__SenderIsNotTheController();
    error ResturantToken__TokenNotYetBurnable();
    error ResturantToken__SenderIsNotNftOwner();

    modifier onlyConrtoller() {
        if (_msgSender() != address(s_mtsController)) {
            revert ResturantToken__SenderIsNotTheController();
        }
        _;
    }

    modifier isForSale(uint256 tokenId) {
        if (_ownerOf(tokenId) != address(this)) {
            revert ResturantToken__TokenNotForSale();
        }
        _;
    }

    modifier isSold(uint256 tokenId) {
        if (!_exists(tokenId) || _ownerOf(tokenId) == address(this)) {
            revert ResturantToken__TokenNotSold();
        }
        _;
    }

    modifier isNotUsed(uint256 tokenId) {
        if (s_nfts[tokenId].locked == true) {
            revert ResturantToken__TokenAlreadyUsed();
        }
        _;
    }

    modifier isUsed(uint256 tokenId) {
        if (s_nfts[tokenId].locked == false) {
            revert ResturantToken__TokenNotUsed();
        }
        _;
    }

    modifier isNftOwner(uint256 tokenId, address _nftOwner) {
        if (ownerOf(tokenId) != _nftOwner) {
            revert ResturantToken__SenderIsNotNftOwner();
        }
        _;
    }

    constructor() {
        _disableInitializers();
    }

    function initialize(
        address _resturantOwner,
        address _mtsController,
        string memory _resturantName,
        string memory _nftSymbol
    )
        public
        initializer
    {
        __ERC721_init(_resturantName, _nftSymbol);
        __ERC721Enumerable_init();
        __Pausable_init();
        __Ownable_init();
        s_mtsController = IMTSController(_mtsController);
        transferOwnership(_resturantOwner);
    }

    function safeMint(
        uint256 price,
        address paymentToken,
        uint32 reservationDate,
        string memory uri
    )
        external
        onlyOwner
    {
        // check if price and paymentToken are acceptable by mtsDAO
        if (!s_mtsController.isPriceAcceptable(paymentToken, price)) {
            revert ResturantToken__PriceNotAcceptable();
        }
        uint256 tokenId = s_tokenIdCounter.current();
        s_tokenIdCounter.increment();
        _safeMint(address(this), tokenId);
        s_nfts[tokenId] = NFT({
            price: price,
            paymentToken: paymentToken,
            reservationDate: reservationDate,
            locked: false,
            uri: uri,
            reviewUri: ""
        });
        // ERC-5192
        emit Unlocked(tokenId);
    }

    function buyNFT(uint256 tokenId) external isForSale(tokenId) {
        uint256 salePrice = s_nfts[tokenId].price;
        address paymentToken = s_nfts[tokenId].paymentToken;
        address newOwner = _msgSender();
        _transfer(address(this), newOwner, tokenId);
        IERC20(paymentToken).safeTransferFrom(_msgSender(), address(this), salePrice);
    }

    function useTicket(
        bytes calldata signature,
        uint256 tokenId
    )
        external
        onlyOwner
        isSold(tokenId)
        isNotUsed(tokenId)
    {
        // TODO: think if usefull check if ticket is for a reservation around this time,
        // maybe frontend check is sufficient
        address tokenOwner = _ownerOf(tokenId);
        bool isValidSignature = VerifySignature.verify(tokenOwner, tokenId, address(this), signature);
        if (!isValidSignature) {
            revert ResturantToken__InvalidSignatureFromClient();
        }
        s_nfts[tokenId].locked = true;
        _payoutToken(tokenId);
        // ERC-5192
        emit Locked(tokenId);
    }

    /**
     *
     * @notice reviews can be change by resubmitting
     */
    function sendReview(
        uint256 tokenId,
        string calldata uri
    )
        external
        isUsed(tokenId)
        isNftOwner(tokenId, _msgSender())
    {
        s_nfts[tokenId].reviewUri = uri;
        emit ReviewPosted(tokenId, uri);
    }

    function burnNftNotSold(uint256 tokenId) external onlyOwner isForSale(tokenId) {
        _burn(tokenId);
    }

    /**
     *
     * @notice Can be used if resturant can't serve the experience (es. closed for illness, etc.)
     *         May be exploited to refund NFT that incresed in value/value of the sale dropped => emit a new NFT
     */

    function refund(uint256 tokenId) external onlyOwner isSold(tokenId) {
        uint256 salePrice = s_nfts[tokenId].price;
        address paymentToken = s_nfts[tokenId].paymentToken;
        _burn(tokenId);
        IERC20(paymentToken).safeTransfer(_ownerOf(tokenId), salePrice);
    }

    function burnTicketNotConsumed(uint256 tokenId) external onlyOwner /* Maybe not onlyOwner */ isSold(tokenId) {
        if (s_nfts[tokenId].reservationDate + EXPIRATION_RANGE > block.timestamp) {
            revert ResturantToken__TokenNotYetBurnable();
        }
        _burn(tokenId);
        _payoutToken(tokenId);
    }

    function _payoutToken(uint256 tokenId) internal {
        uint256 price = s_nfts[tokenId].price;
        address paymentToken = s_nfts[tokenId].paymentToken;

        uint256 ppFeeMTSDao = s_mtsController.getFeeInBasePoint(paymentToken);
        require(ppFeeMTSDao < 10_000, "fee can't be more than 100%");
        uint256 feeMTSDao = (price * ppFeeMTSDao) / 10_000;
        IERC20(paymentToken).safeTransfer(address(s_mtsController), feeMTSDao);
        IERC20(paymentToken).safeTransfer(owner(), price - feeMTSDao);
    }

    function tokenURI(uint256 tokenId) public view override returns (string memory) {
        return s_nfts[tokenId].uri;
    }

    /**
     * @dev Whenever an {IERC721} `tokenId` token is transferred to this contract via {IERC721-safeTransferFrom}
     * by `operator` from `from`, this function is called.
     *
     * It must return its Solidity selector to confirm the token transfer.
     * If any other value is returned or the interface is not implemented by the recipient, the transfer will be
     * reverted.
     *
     * The selector can be obtained in Solidity with `IERC721Receiver.onERC721Received.selector`.
     */
    function onERC721Received(
        address, /* operator */
        address, /* from */
        uint256, /* tokenId */
        bytes calldata /* data */
    )
        external
        pure
        returns (bytes4)
    {
        return this.onERC721Received.selector;
    }

    /* ========================================================================== */
    /*                                   GETTERS                                  */
    /* ========================================================================== */

    function getControllerAddress() external view returns (address) {
        return address(s_mtsController);
    }

    function getCounter() external view returns (uint256) {
        return s_tokenIdCounter.current();
    }

    function getNft(uint256 tokenId) external view returns (NFT memory) {
        return s_nfts[tokenId];
    }

    function isTokenUsed(uint256 tokenId) external view returns (bool) {
        return s_nfts[tokenId].locked;
    }

    /* ========================================================================== */
    /*                                   ERC-5192                                  */
    /* ========================================================================== */
    function locked(uint256 tokenId) external view returns (bool) {
        return s_nfts[tokenId].locked;
    }

    /* ========================================================================== */
    /*                                  Pausable                                  */
    /* ========================================================================== */
    function pause() public onlyConrtoller {
        _pause();
    }

    function unpause() public onlyConrtoller {
        _unpause();
    }

    /* ========================================================================== */
    /*         The following functions are overrides required by Solidity.        */
    /*                             From OP Wizzard                                */
    /* ========================================================================== */

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC721Upgradeable, ERC721EnumerableUpgradeable)
        returns (bool)
    {
        // ERC-5192
        if (interfaceId == 0xb45a3c0e) return true;
        return super.supportsInterface(interfaceId);
    }

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId,
        uint256 batchSize
    )
        internal
        override(ERC721Upgradeable, ERC721EnumerableUpgradeable)
    {
        super._beforeTokenTransfer(from, to, tokenId, batchSize);
    }
}
