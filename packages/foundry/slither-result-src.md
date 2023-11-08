**THIS CHECKLIST IS NOT COMPLETE**. Use `--show-ignored-findings` to show all the results.
Summary
 - [reentrancy-benign](#reentrancy-benign) (4 results) (Low)
 - [reentrancy-events](#reentrancy-events) (2 results) (Low)
 - [timestamp](#timestamp) (2 results) (Low)
 - [solc-version](#solc-version) (2 results) (Informational)
 - [calls-loop](#calls-loop) (1 results) (Low)
 - [naming-convention](#naming-convention) (3 results) (Informational)
 - [too-many-digits](#too-many-digits) (2 results) (Informational)
## reentrancy-benign
Impact: Low
Confidence: Medium
 - [ ] ID-0
Reentrancy in [ResturantToken.safeMint(uint256,address,uint32,string)](src/ResturantToken.sol#L140-L165):
	External calls:
	- [! s_mtsController.isPriceAcceptable(paymentToken,price)](src/ResturantToken.sol#L150)
	State variables written after the call(s):
	- [tokenId = s_tokenIdCounter ++](src/ResturantToken.sol#L153)

src/ResturantToken.sol#L140-L165

****
 - [ ] ID-1
Reentrancy in [ResturantToken.safeMint(uint256,address,uint32,string)](src/ResturantToken.sol#L140-L165):
	External calls:
	- [! s_mtsController.isPriceAcceptable(paymentToken,price)](src/ResturantToken.sol#L150)
	- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
		- [retval = IERC721Receiver(to).onERC721Received(_msgSender(),from,tokenId,data)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L496-L509)
	State variables written after the call(s):
	- [s_nfts[tokenId] = NFT({price:price,paymentToken:paymentToken,reservationDate:reservationDate,locked:false,uri:uri,reviewUri:})](src/ResturantToken.sol#L155-L162)

src/ResturantToken.sol#L140-L165


 - [ ] ID-2
Reentrancy in [ResturantToken.safeMint(uint256,address,uint32,string)](src/ResturantToken.sol#L140-L165):
	External calls:
	- [! s_mtsController.isPriceAcceptable(paymentToken,price)](src/ResturantToken.sol#L150)
	State variables written after the call(s):
	- [tokenId = s_tokenIdCounter ++](src/ResturantToken.sol#L153)

src/ResturantToken.sol#L140-L165


 - [ ] ID-3
Reentrancy in [ResturantToken.safeMint(uint256,address,uint32,string)](src/ResturantToken.sol#L140-L165):
	External calls:
	- [! s_mtsController.isPriceAcceptable(paymentToken,price)](src/ResturantToken.sol#L150)
	- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
		- [retval = IERC721Receiver(to).onERC721Received(_msgSender(),from,tokenId,data)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L496-L509)
	State variables written after the call(s):
	- [s_nfts[tokenId] = NFT({price:price,paymentToken:paymentToken,reservationDate:reservationDate,locked:false,uri:uri,reviewUri:})](src/ResturantToken.sol#L155-L162)

src/ResturantToken.sol#L140-L165


## reentrancy-events
Impact: Low
Confidence: Medium
 - [ ] ID-4
Reentrancy in [ResturantToken.safeMint(uint256,address,uint32,string)](src/ResturantToken.sol#L140-L165):
	External calls:
	- [! s_mtsController.isPriceAcceptable(paymentToken,price)](src/ResturantToken.sol#L150)
	- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
		- [retval = IERC721Receiver(to).onERC721Received(_msgSender(),from,tokenId,data)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L496-L509)
	Event emitted after the call(s):
	- [Approval(owner,to,tokenId)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L447)
		- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
	- [Transfer(from,to,tokenId)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L294)
		- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
	- [Unlocked(tokenId)](src/ResturantToken.sol#L164)

src/ResturantToken.sol#L140-L165


 - [ ] ID-5
Reentrancy in [ResturantToken.safeMint(uint256,address,uint32,string)](src/ResturantToken.sol#L140-L165):
	External calls:
	- [! s_mtsController.isPriceAcceptable(paymentToken,price)](src/ResturantToken.sol#L150)
	- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
		- [retval = IERC721Receiver(to).onERC721Received(_msgSender(),from,tokenId,data)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L496-L509)
	Event emitted after the call(s):
	- [Approval(owner,to,tokenId)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L447)
		- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
	- [Transfer(from,to,tokenId)](lib/openzeppelin-contracts-upgradeable/contracts/token/ERC721/ERC721Upgradeable.sol#L294)
		- [_safeMint(address(this),tokenId)](src/ResturantToken.sol#L154)
	- [Unlocked(tokenId)](src/ResturantToken.sol#L164)

src/ResturantToken.sol#L140-L165


## timestamp
Impact: Low
Confidence: Medium
 - [ ] ID-6
[ResturantToken.burnTicketNotConsumed(uint256)](src/ResturantToken.sol#L231-L237) uses timestamp for comparisons
	Dangerous comparisons:
	- [s_nfts[tokenId].reservationDate + EXPIRATION_RANGE > block.timestamp](src/ResturantToken.sol#L232)

src/ResturantToken.sol#L231-L237


 - [ ] ID-7
[ResturantToken.burnTicketNotConsumed(uint256)](src/ResturantToken.sol#L231-L237) uses timestamp for comparisons
	Dangerous comparisons:
	- [s_nfts[tokenId].reservationDate + EXPIRATION_RANGE > block.timestamp](src/ResturantToken.sol#L232)

src/ResturantToken.sol#L231-L237


## solc-version
Impact: Informational
Confidence: High
 - [ ] ID-8
Pragma version[^0.8.20](lib/openzeppelin-contracts/contracts/interfaces/IERC721Receiver.sol#L4) necessitates a version too recent to be trusted. Consider deploying with 0.8.18.

lib/openzeppelin-contracts/contracts/interfaces/IERC721Receiver.sol#L4


 - [ ] ID-9
Pragma version[^0.8.20](lib/openzeppelin-contracts/contracts/interfaces/IERC721Receiver.sol#L4) necessitates a version too recent to be trusted. Consider deploying with 0.8.18.

lib/openzeppelin-contracts/contracts/interfaces/IERC721Receiver.sol#L4


## calls-loop
Impact: Low
Confidence: Medium
 - [ ] ID-10
[MTSController.withdrawFunds(address[],uint256[],address)](src/MTSController.sol#L117-L131) has external calls inside a loop: [success = IERC20(tokens[i]).transfer(to,balances[i])](src/MTSController.sol#L123)

src/MTSController.sol#L117-L131


## naming-convention
Impact: Informational
Confidence: High
 - [ ] ID-11
Parameter [MTSController.addNewResturant(address,string,string)._symbol](src/MTSController.sol#L37) is not in mixedCase

src/MTSController.sol#L37


 - [ ] ID-12
Parameter [MTSController.addNewResturant(address,string,string)._resturantOwner](src/MTSController.sol#L35) is not in mixedCase

src/MTSController.sol#L35


 - [ ] ID-13
Parameter [MTSController.addNewResturant(address,string,string)._name](src/MTSController.sol#L36) is not in mixedCase

src/MTSController.sol#L36


## too-many-digits
Impact: Informational
Confidence: Medium
 - [ ] ID-14
[Clones.cloneDeterministic(address,bytes32)](lib/openzeppelin-contracts/contracts/proxy/Clones.sol#L50-L63) uses literals with too many digits:
	- [mstore(uint256,uint256)(0x00,implementation << 0x60 >> 0xe8 | 0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000)](lib/openzeppelin-contracts/contracts/proxy/Clones.sol#L55)

lib/openzeppelin-contracts/contracts/proxy/Clones.sol#L50-L63


 - [ ] ID-15
[Clones.clone(address)](lib/openzeppelin-contracts/contracts/proxy/Clones.sol#L28-L41) uses literals with too many digits:
	- [mstore(uint256,uint256)(0x00,implementation << 0x60 >> 0xe8 | 0x3d602d80600a3d3981f3363d3d373d3d3d363d73000000)](lib/openzeppelin-contracts/contracts/proxy/Clones.sol#L33)

lib/openzeppelin-contracts/contracts/proxy/Clones.sol#L28-L41


