// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { BaseScript } from "./Base.s.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { DeployMTSController } from "./DeployMTSController.s.sol";
import { DeployMinimalScenarioWithPkOwner } from "./DeployMinimalScenarioWithPkOwner.s.sol";
import { DeployResturantWithPkOwner } from "./DeployResturantWithPkOwner.s.sol";
import { DeployMockErc20 } from "./DeployMockErc20.s.sol";
import { AddAlloedTokenToController, MintNft } from "./Interactions.s.sol";
import { ERC20Mock } from "@openzeppelin/contracts/mocks/ERC20Mock.sol";
import { console2 } from "forge-std/console2.sol";
import { Strings } from "@openzeppelin/contracts/utils/Strings.sol";

contract SetUpTestScenario is BaseScript {
    uint256 constant NUMBER_OF_RESTURANT = 3;
    uint256 constant NUMBER_OF_NFT_PER_RESTURANT = 10;
    address constant RESTURANT_OWNER = 0x6C1b125e0D951396C0256ab083615a558C615648;
    string constant RESTURANT_TOKEN_NAME = "Resturant";
    string constant RESTURANT_TOKEN_SYMBOL = "RT";
    string constant IPFS_METADATA_BASE = "ipfs://bafybeibc5sgo2plmjkq2tzmhrn54bk3crhnc23zd2msg4ea7a4pxrkgfna/";

    function run() public exportDeployments returns (MTSController, ResturantToken[NUMBER_OF_RESTURANT] memory) {
        uint256 ownersPk = vm.envUint("MTS_OWNER");
        address ownersAdddress = vm.addr(ownersPk);
        fundAddress(ownersAdddress);

        MTSController controller = new DeployMTSController().run(ownersAdddress);

        ResturantToken[NUMBER_OF_RESTURANT] memory resturantTokens;
        vm.startBroadcast(ownersPk);
        for (uint256 i = 0; i < NUMBER_OF_RESTURANT; i++) {
            resturantTokens[i] = controller.addNewResturant(
                RESTURANT_OWNER,
                string.concat(RESTURANT_TOKEN_NAME, Strings.toString(i)),
                string.concat(RESTURANT_TOKEN_SYMBOL, Strings.toString(i))
            );
            // vm.stopBroadcast();
            console2.log("Owner of resturant %d: %s", i, resturantTokens[i].owner());
            console2.log("MTSController resturants: %d", controller.getNumberOfResturants());
        }
        vm.stopBroadcast();

        ERC20Mock erc20 = new DeployMockErc20().run();

        new AddAlloedTokenToController().run(address(controller), address(erc20), 1);

        fundAddress(RESTURANT_OWNER);
        vm.startBroadcast(RESTURANT_OWNER);
        for (uint256 i = 0; i < resturantTokens.length; i++) {
            for (uint256 k = 0; k < NUMBER_OF_NFT_PER_RESTURANT; k++) {
                uint32 reservationDate = uint32(block.timestamp);
                if (k % 3 != 0) {
                    reservationDate += 10 * 60 * 60 * 24;
                }
                resturantTokens[i].safeMint(
                    (k + 1) * 1 ether,
                    address(erc20),
                    reservationDate,
                    string.concat(IPFS_METADATA_BASE, Strings.toString(i * resturantTokens.length + k))
                );
            }
        }
        vm.stopBroadcast();

        return (controller, resturantTokens);
    }

    function fundAddress(address funded) internal broadcast {
        (bool sent,) = funded.call{ value: 1 ether }("");
        require(sent, "Failed to send Ether");
    }
}
