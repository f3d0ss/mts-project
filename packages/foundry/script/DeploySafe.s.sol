// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.19 <=0.9.0;

import { Safe } from "lib/safe-contracts/contracts/Safe.sol";
import { SafeProxyFactory } from "lib/safe-contracts/contracts/proxies/SafeProxyFactory.sol";
import { CompatibilityFallbackHandler } from "lib/safe-contracts/contracts/handler/CompatibilityFallbackHandler.sol";
import { HelperConfig } from "./HelperConfig.s.sol";
import { BaseScript } from "./Base.s.sol";

contract DeploySafe is BaseScript {
    string public constant safe_v_1_4_1 = "1.4.1";
    string public constant safe_v_1_3_0 = "1.3.0";

    function run(address[] calldata owners, uint256 threshold) public returns (Safe) {
        HelperConfig config = new HelperConfig();
        address safeAddress;
        address compatibilityFallbackHandler;
        address safeProxyFactoryAddress;

        string memory safeVersion = vm.envOr({ name: "SAFE_VERSION", defaultValue: safe_v_1_3_0 });
        if (keccak256(abi.encodePacked(safeVersion)) == keccak256(abi.encodePacked(safe_v_1_3_0))) {
            (,,, safeAddress, compatibilityFallbackHandler, safeProxyFactoryAddress) = config.activeNetworkConfig();
        } else if (keccak256(abi.encodePacked(safeVersion)) == keccak256(abi.encodePacked(safe_v_1_4_1))) {
            (safeAddress, compatibilityFallbackHandler, safeProxyFactoryAddress,,,) = config.activeNetworkConfig();
        }

        Safe singleton = Safe(payable(safeAddress));
        SafeProxyFactory safeProxyFactory = SafeProxyFactory(safeProxyFactoryAddress);
        CompatibilityFallbackHandler handler = CompatibilityFallbackHandler(compatibilityFallbackHandler);
        bytes memory initializer = abi.encodeWithSelector(
            Safe.setup.selector,
            owners,
            threshold,
            address(0), /* setupModulesCall_to */
            "", /* setupModulesCall_data */
            handler,
            address(0), /* refundToken */
            0, /* refundAmount */
            address(0) /* refundReceiver */
        );
        Safe safe = deplySafe(safeProxyFactory, singleton, initializer, 3 /* TODO: think about salt */ );
        return safe;
    }

    function deplySafe(
        SafeProxyFactory safeProxyFactory,
        Safe singleton,
        bytes memory initializer,
        uint256 saltNonce
    )
        public
        broadcast
        returns (Safe)
    {
        return Safe(payable(safeProxyFactory.createProxyWithNonce(address(singleton), initializer, saltNonce)));
    }
}
