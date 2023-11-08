// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.18 <=0.9.0;

import { ResturantToken } from "src/ResturantToken.sol";
import { MTSController } from "src/MTSController.sol";
import { Safe, Enum } from "lib/safe-contracts/contracts/Safe.sol";
import { console2 } from "forge-std/console2.sol";
import { SortKeys } from "./utils/SortKeys.sol";

import { BaseScript } from "./Base.s.sol";

contract DeployResturantWithSafeOwner is BaseScript, SortKeys {
    // keccak256(
    //     "SafeTx(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256
    // gasPrice,address gasToken,address refundReceiver,uint256 nonce)"
    // );
    bytes32 private constant SAFE_TX_TYPEHASH = 0xbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8;

    mapping(address publicAddresses => uint256 pk) private wallets;

    function run(
        address resturantOwner,
        string calldata name,
        string calldata symbol,
        address controllerAddress,
        address safeAddress
    )
        public
        returns (ResturantToken)
    {
        uint256[] memory safePrivateKeys = vm.envUint("SAFE_OWNERS_PKS", ",");

        safePrivateKeys = sortPKsByComputedAddress(safePrivateKeys);
        Safe safe = Safe(payable(safeAddress));

        bytes32 txHashData = keccak256(
            encodeTransactionData( // Transaction info
                controllerAddress,
                abi.encodeWithSignature("addNewResturant(address,string,string)", resturantOwner, name, symbol), /* Check
                    if correct */
                safe.nonce(),
                safe
            )
        );

        bytes memory signatures;
        for (uint256 i = 0; i < safePrivateKeys.length; i++) {
            bytes memory newSignature = createSignature(txHashData, safePrivateKeys[i]);
            signatures = abi.encodePacked(signatures, newSignature);
        }

        deployResturant(
            safe,
            controllerAddress,
            abi.encodeWithSignature("addNewResturant(address,string,string)", resturantOwner, name, symbol), /* Check if
                correct */
            signatures
        );

        return ResturantToken(
            MTSController(controllerAddress).getResturantAddress(
                MTSController(controllerAddress).getNumberOfResturants() - 1
            )
        );
    }

    function encodeTransactionData(
        address to,
        bytes memory data,
        uint256 _nonce,
        Safe safe
    )
        internal
        view
        returns (bytes memory)
    {
        bytes32 safeTxHash = keccak256(
            abi.encode(
                SAFE_TX_TYPEHASH, to, 0, keccak256(data), Enum.Operation.Call, 0, 0, 0, address(0), address(0), _nonce
            )
        );
        return abi.encodePacked(bytes1(0x19), bytes1(0x01), safe.domainSeparator(), safeTxHash);
    }

    function deployResturant(
        Safe safe,
        address controllerAddress,
        bytes memory data,
        bytes memory signatures
    )
        internal
        broadcast
    {
        bool success = safe.execTransaction(
            controllerAddress,
            0,
            data,
            //
            Enum.Operation.Call,
            0,
            0,
            0,
            address(0),
            payable(address(0)),
            // Signature info
            signatures
        );
        require(success, "Failed to deploy the new resturant through the Safe");
    }

    function createSignature(bytes32 txHashData, uint256 safePrivateKey) internal pure returns (bytes memory) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(safePrivateKey, txHashData);
        return abi.encodePacked(r, s, v);
    }
}
