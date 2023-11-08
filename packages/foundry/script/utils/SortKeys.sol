// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.20;

import { LibSort } from "lib/solady/src/utils/LibSort.sol";
import { Script } from "forge-std/Script.sol";

contract SortKeys is Script {
    function sortPKsByComputedAddress(uint256[] memory _pks) internal pure returns (uint256[] memory) {
        uint256[] memory sortedPKs = new uint256[](_pks.length);

        address[] memory addresses = new address[](_pks.length);
        bytes32[2][] memory accounts = new bytes32[2][](_pks.length);

        for (uint256 i; i < _pks.length; i++) {
            address signer = vm.addr(_pks[i]);
            addresses[i] = signer;
            accounts[i][0] = bytes32(abi.encode(signer));
            accounts[i][1] = bytes32(_pks[i]);
        }

        LibSort.sort(addresses);

        uint256 found;
        for (uint256 j; j < addresses.length; j++) {
            address signer = addresses[j];
            uint256 pk;
            for (uint256 k; k < accounts.length; k++) {
                if (address(uint160(uint256(accounts[k][0]))) == signer) {
                    pk = uint256(accounts[k][1]);
                    found++;
                }
            }

            sortedPKs[j] = pk;
        }

        if (found < _pks.length) {
            revert("SAFETESTTOOLS: issue with private key sorting, please open a ticket on github");
        }
        return sortedPKs;
    }
}
