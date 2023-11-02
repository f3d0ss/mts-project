// SPDX-License-Identifier: SEE LICENSE IN LICENSE
pragma solidity ^0.8.19;

import { PRBTest } from "@prb/test/PRBTest.sol";
import { console2 } from "forge-std/console2.sol";

contract GasHelpers is PRBTest {
    string private checkpointLabel;
    uint256 private checkpointGasLeft = 1; // Start the slot warm.

    function startMeasuringGas(string memory label) internal virtual {
        checkpointLabel = label;

        checkpointGasLeft = gasleft();
    }

    function stopMeasuringGas() internal virtual {
        uint256 checkpointGasLeft2 = gasleft();

        // Subtract 100 to account for the warm SLOAD in startMeasuringGas.
        uint256 gasDelta = checkpointGasLeft - checkpointGasLeft2 - 100;

        emit LogNamedUint256(string(abi.encodePacked(checkpointLabel, " Gas")), gasDelta);
        if (bytes(checkpointLabel).length < 20) {
            checkpointLabel = string(abi.encodePacked(checkpointLabel, "\t"));
        }
        console2.log(string(abi.encodePacked(checkpointLabel, "\t Gas")), gasDelta);
    }
}
