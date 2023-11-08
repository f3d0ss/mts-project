// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.18;

import { BaseScript } from "./Base.s.sol";
import { Safe } from "lib/safe-contracts/contracts/Safe.sol";
import { CompatibilityFallbackHandler } from "lib/safe-contracts/contracts/handler/CompatibilityFallbackHandler.sol";
import { SafeProxyFactory } from "lib/safe-contracts/contracts/proxies/SafeProxyFactory.sol";

contract HelperConfig is BaseScript {
    struct NetworkConfig {
        address safe_v1_4_1;
        address compatibilityFallbackHandler_v1_4_1;
        address safeProxyFactory_v1_4_1;
        address safe_v1_3_0;
        address compatibilityFallbackHandler_v1_3_0;
        address safeProxyFactory_v1_3_0;
    }

    NetworkConfig public activeNetworkConfig;

    constructor() {
        if (block.chainid == 11_155_111) {
            activeNetworkConfig = getSepoliaEthConfig();
        } else if (block.chainid == 5) {
            activeNetworkConfig = getGoerliEthConfig();
        } else {
            activeNetworkConfig = getOrCreateAnvilEthConfig();
        }
    }

    function getSepoliaEthConfig() public pure returns (NetworkConfig memory) {
        return NetworkConfig({
            safe_v1_4_1: 0x41675C099F32341bf84BFc5382aF534df5C7461a,
            compatibilityFallbackHandler_v1_4_1: 0xfd0732Dc9E303f09fCEf3a7388Ad10A83459Ec99,
            safeProxyFactory_v1_4_1: 0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67,
            safe_v1_3_0: 0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552,
            compatibilityFallbackHandler_v1_3_0: 0xf48f2B2d2a534e402487b3ee7C18c33Aec0Fe5e4,
            safeProxyFactory_v1_3_0: 0xa6B71E26C5e0845f74c812102Ca7114b6a896AB2
        });
    }

    function getGoerliEthConfig() public pure returns (NetworkConfig memory) {
        return NetworkConfig({
            safe_v1_4_1: 0x41675C099F32341bf84BFc5382aF534df5C7461a,
            compatibilityFallbackHandler_v1_4_1: 0xfd0732Dc9E303f09fCEf3a7388Ad10A83459Ec99,
            safeProxyFactory_v1_4_1: 0x4e1DCf7AD4e460CfD30791CCC4F9c8a4f820ec67,
            safe_v1_3_0: 0xd9Db270c1B5E3Bd161E8c8503c55cEABeE709552,
            compatibilityFallbackHandler_v1_3_0: 0xf48f2B2d2a534e402487b3ee7C18c33Aec0Fe5e4,
            safeProxyFactory_v1_3_0: 0xa6B71E26C5e0845f74c812102Ca7114b6a896AB2
        });
    }

    function getOrCreateAnvilEthConfig() public returns (NetworkConfig memory) {
        if (activeNetworkConfig.safe_v1_4_1 != address(0)) {
            return activeNetworkConfig;
        }
        return createAnvilEthConfig();
    }

    function createAnvilEthConfig() internal broadcast returns (NetworkConfig memory) {
        Safe safe = new Safe();
        CompatibilityFallbackHandler compatibilityFallbackHandler = new CompatibilityFallbackHandler();
        SafeProxyFactory safeProxyFactory = new SafeProxyFactory();
        return NetworkConfig({
            safe_v1_4_1: address(safe),
            compatibilityFallbackHandler_v1_4_1: address(compatibilityFallbackHandler),
            safeProxyFactory_v1_4_1: address(safeProxyFactory),
            safe_v1_3_0: address(safe),
            compatibilityFallbackHandler_v1_3_0: address(compatibilityFallbackHandler),
            safeProxyFactory_v1_3_0: address(safeProxyFactory)
        });
    }
}
