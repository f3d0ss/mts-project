// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";
import { Safe } from "lib/safe-contracts/contracts/Safe.sol";
import { MTSController } from "src/MTSController.sol";
import { ResturantToken } from "src/ResturantToken.sol";

contract GetNonceSafe is Script {
    function run(address safeAddress) external view returns (uint256) {
        Safe safe = Safe(payable(safeAddress));
        return safe.nonce();
    }
}

contract AddAlloedTokenToController is Script {
    function run(address controller, address tokenPayment, uint256 minPrice) public {
        uint256 controllerOwner = vm.envUint("MTS_OWNER");
        vm.broadcast(controllerOwner);
        MTSController(controller).setAcceptableMinPrice(tokenPayment, minPrice);
    }
}

contract MintNft is Script {
    function run(
        address resturantToken,
        uint256 price,
        address tokenPayment,
        uint32 reservationDate,
        string calldata uri
    )
        public
    {
        ResturantToken(resturantToken).safeMint(price, tokenPayment, reservationDate, uri);
    }
}
