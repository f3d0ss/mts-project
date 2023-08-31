// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { Script } from "forge-std/Script.sol";
import { ResturantToken } from "src/ResturantToken.sol";

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

contract BuyNft is Script {
    function run(address resturant, uint256 tokenId) public {
        ResturantToken(resturant).buyNFT(tokenId);
    }
}
