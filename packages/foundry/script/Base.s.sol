// SPDX-License-Identifier: MIT
pragma solidity >=0.8.19 <=0.9.0;

import { Script } from "forge-std/Script.sol";

abstract contract BaseScript is Script {
    /// @dev Included to enable compilation of the script without a $MNEMONIC environment variable.
    string internal constant TEST_MNEMONIC = "test test test test test test test test test test test junk";

    /// @dev Needed for the deterministic deployments.
    bytes32 internal constant ZERO_SALT = bytes32(0);

    /// @dev The address of the transaction broadcaster.
    address internal broadcaster;

    /// @dev Used to derive the broadcaster's address if $ETH_FROM is not defined.
    string internal mnemonic;

    /// @dev Initializes the transaction broadcaster like this:
    ///
    /// - If $ETH_FROM is defined, use it.
    /// - Otherwise, derive the broadcaster address from $MNEMONIC.
    /// - If $MNEMONIC is not defined, default to a test mnemonic.
    ///
    /// The use case for $ETH_FROM is to specify the broadcaster key and its address via the command line.
    constructor() {
        address from = vm.envOr({ name: "ETH_FROM", defaultValue: address(0) });
        if (from != address(0)) {
            broadcaster = from;
        } else {
            uint256 pk = getPrivateKeyFromEnv();
            if (pk != 0) {
                broadcaster = vm.rememberKey(pk);
            } else {
                mnemonic = vm.envOr({ name: "MNEMONIC", defaultValue: TEST_MNEMONIC });
                (broadcaster,) = deriveRememberKey({ mnemonic: mnemonic, index: 0 });
            }
        }
    }

    function getPrivateKeyFromEnv() internal returns (uint256) {
        if (block.chainid == 11_155_111) {
            return vm.envOr({ name: "SEPOLIA_PRIVATE_KEY", defaultValue: uint256(0) });
        } else if (block.chainid == 5) {
            return vm.envOr({ name: "GOERLI_PRIVATE_KEY", defaultValue: uint256(0) });
        } else {
            return vm.envOr({ name: "ANVIL_PRIVATE_KEY", defaultValue: uint256(0) });
        }
    }

    modifier broadcast() {
        vm.startBroadcast(broadcaster);
        _;
        vm.stopBroadcast();
    }

    /* ========================================================================== */
    /*                          FROM SCAFFOLD-ETH-FOUDRY                          */
    /* ========================================================================== */

    struct Deployment {
        string name;
        address addr;
    }

    string root;
    string path;
    Deployment[] public deployments;

    modifier exportDeployments() {
        _;
        // fetch already existing contracts
        root = vm.projectRoot();
        path = string.concat(root, "/deployments/");
        string memory chainIdStr = vm.toString(block.chainid);
        path = string.concat(path, string.concat(chainIdStr, ".json"));

        string memory jsonWrite;

        uint256 len = deployments.length;

        for (uint256 i = 0; i < len; i++) {
            vm.serializeString(jsonWrite, vm.toString(deployments[i].addr), deployments[i].name);
        }

        Chain memory chain = getChain(block.chainid);
        jsonWrite = vm.serializeString(jsonWrite, "networkName", chain.name);
        vm.writeJson(jsonWrite, path);
    }
}
