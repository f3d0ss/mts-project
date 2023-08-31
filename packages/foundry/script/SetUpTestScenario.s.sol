// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { BaseScript } from "./Base.s.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { DeployMTSController } from "./DeployMTSController.s.sol";
import { DeployMinimalScenarioWithPkOwner } from "./DeployMinimalScenarioWithPkOwner.s.sol";
import { DeployResturantWithPkOwner } from "./DeployResturantWithPkOwner.s.sol";
import { DeployMockErc20 } from "./DeployMockErc20.s.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import { console2 } from "forge-std/console2.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

contract SetUpTestScenario is BaseScript {
    uint32 constant TEN_DAYS = 10 * 60 * 60 * 24;

    function run() public exportDeployments returns (MTSController, ResturantToken[] memory) {
        uint256 NUMBER_OF_RESTURANT = vm.envUint("NUMBER_OF_RESTURANT");
        uint256 NUMBER_OF_NFT_PER_RESTURANT = vm.envUint("NUMBER_OF_NFT_PER_RESTURANT");
        address[] memory RESTURANT_OWNERS = vm.envAddress("RESTURANT_OWNERS", ",");
        string memory BASE_RESTURANT_TOKEN_NAME = vm.envString("RESTURANT_TOKEN_NAME");
        string memory BASE_RESTURANT_TOKEN_SYMBOL = vm.envString("RESTURANT_TOKEN_SYMBOL");
        string memory IPFS_METADATA_BASE = vm.envString("IPFS_METADATA_BASE");
        uint256 CONTROLLER_OWNER_PK = vm.envUint("MTS_OWNER");
        uint256[] memory BUYERS_PKS = vm.envUint("BUYERS_PKS", ",");
        address controllerOwnersAdddress = vm.addr(CONTROLLER_OWNER_PK);
        fundAddress(controllerOwnersAdddress);

        MTSController controller = new DeployMTSController().run(controllerOwnersAdddress);

        ResturantToken[] memory resturantTokens = addNewResturants(
            controller,
            CONTROLLER_OWNER_PK,
            NUMBER_OF_RESTURANT,
            RESTURANT_OWNERS,
            BASE_RESTURANT_TOKEN_NAME,
            BASE_RESTURANT_TOKEN_SYMBOL
        );

        ERC20Mock erc20 = new DeployMockErc20().run();

        vm.broadcast(CONTROLLER_OWNER_PK);
        controller.setAcceptableMinPrice(address(erc20), 1);

        for (uint256 i = 0; i < RESTURANT_OWNERS.length; i++) {
            fundAddress(RESTURANT_OWNERS[i]);
        }
        mintExperiencesForResturants(
            resturantTokens, address(erc20), RESTURANT_OWNERS, NUMBER_OF_NFT_PER_RESTURANT, IPFS_METADATA_BASE
        );

        for (uint256 i = 0; i < BUYERS_PKS.length; i++) {
            uint256 buyerPk = BUYERS_PKS[i];
            address buyerAddress = vm.addr(buyerPk);
            fundAddress(buyerAddress);
            fundAddress(buyerAddress, erc20);
            buyNftFromRestuants(resturantTokens, buyerPk);
        }

        return (controller, resturantTokens);
    }

    function buyNftFromRestuants(ResturantToken[] memory resturants, uint256 buyerPk) internal {
        for (uint256 j = 0; j < resturants.length; j++) {
            ResturantToken resturant = resturants[j];
            buyNftFromRestuant(resturant, buyerPk);
        }
    }

    function buyNftFromRestuant(ResturantToken resturant, uint256 buyerPk) internal {
        uint256 numberOfNft = resturant.getCounter();
        for (uint256 k = buyerPk % 3; k < numberOfNft; k += 3) {
            if (resturant.ownerOf(k) == address(resturant)) {
                ResturantToken.NFT memory nft = resturant.getNft(k);
                vm.startBroadcast(buyerPk);
                ERC20Mock(nft.paymentToken).approve(address(resturant), nft.price);
                resturant.buyNFT(k);
                vm.stopBroadcast();
            }
        }
    }

    function addNewResturants(
        MTSController controller,
        uint256 controllerOwnerPk,
        uint256 numberOfResturant,
        address[] memory resturantOwners,
        string memory baseResturantTokenName,
        string memory baseResturantTokenSymbol
    )
        internal
        returns (ResturantToken[] memory)
    {
        ResturantToken[] memory resturantTokens = new ResturantToken[](numberOfResturant);
        for (uint256 i = 0; i < numberOfResturant; i++) {
            resturantTokens[i] = addNewResturant(
                controller,
                controllerOwnerPk,
                resturantOwners[i % resturantOwners.length],
                string.concat(baseResturantTokenName, Strings.toString(i)),
                string.concat(baseResturantTokenSymbol, Strings.toString(i))
            );
        }
        return resturantTokens;
    }

    function addNewResturant(
        MTSController controller,
        uint256 controllerOwnerPk,
        address resturantOwner,
        string memory resturantTokenName,
        string memory resturantTokenSymbol
    )
        internal
        returns (ResturantToken)
    {
        vm.startBroadcast(controllerOwnerPk);
        ResturantToken resturantToken =
            controller.addNewResturant(resturantOwner, resturantTokenName, resturantTokenSymbol);
        vm.stopBroadcast();
        return resturantToken;
    }

    function mintExperiencesForResturants(
        ResturantToken[] memory resturantTokens,
        address erc20,
        address[] memory resturantOwners,
        uint256 numberOfNftPerResturant,
        string memory ipfsMetadataBase
    )
        internal
    {
        for (uint256 i = 0; i < resturantTokens.length; i++) {
            mintExperiencesForSingleResturant(
                resturantTokens[i],
                erc20,
                resturantOwners[i % resturantOwners.length],
                numberOfNftPerResturant,
                ipfsMetadataBase
            );
        }
    }

    function mintExperiencesForSingleResturant(
        ResturantToken resturant,
        address paymentToken,
        address resturantOwner,
        uint256 numberOfNft,
        string memory ipfsMetadataBase
    )
        internal
    {
        vm.startBroadcast(resturantOwner);
        for (uint256 k = 0; k < numberOfNft; k++) {
            uint32 reservationDate = uint32(block.timestamp);
            if (k % 2 != 0) {
                reservationDate += TEN_DAYS;
            }
            resturant.safeMint(
                (k + 1) * 1 ether,
                paymentToken,
                reservationDate,
                string.concat(ipfsMetadataBase, Strings.toString(uint160(address(resturant)) % 1000 + k))
            );
        }
        vm.stopBroadcast();
    }

    function fundAddress(address funded) internal {
        if (broadcaster != funded) {
            _fundAddress(funded);
        }
    }

    function _fundAddress(address funded) internal broadcast {
        (bool sent,) = funded.call{ value: 0.1 ether }("");
        require(sent, "Failed to send Ether");
    }

    function fundAddress(address funded, ERC20Mock erc20) internal broadcast {
        erc20.mint(funded, 1000 ether);
    }
}
