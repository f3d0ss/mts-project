// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";
import { console2 } from "forge-std/console2.sol";

contract AddAllowedPaymentToken is Script {
    function run(address controller, address tokenPayment, uint256 minPrice) public {
        MTSController(controller).setAcceptableMinPrice(tokenPayment, minPrice);
    }
}

contract AddNewResturant is Script {
    function run(
        address controller,
        address resturantOwner,
        string memory resturantTokenName,
        string memory resturantTokenSymbol
    )
        public
        returns (ResturantToken)
    {
        uint256 controllerOwner = vm.envUint("MTS_OWNER");
        vm.startBroadcast(controllerOwner);
        return MTSController(controller).addNewResturant(resturantOwner, resturantTokenName, resturantTokenSymbol);
    }
}
