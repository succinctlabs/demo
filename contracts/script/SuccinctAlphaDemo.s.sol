// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

import "forge-std/Script.sol";
import "../src/SuccinctAlphaDemo.sol";

contract SuccinctAlphaDemoScript is Script {
    function run() public {
        vm.startBroadcast();
        SuccinctAlphaDemo demo = new SuccinctAlphaDemo();
        vm.stopBroadcast();

        // vm.startBroadcast();
        // bytes32 functionId = 0x9530e5a54d62bbf26f280190d18b5242f716ec2f803f14a2925213908b2abfa9;
        // bytes32 blockRoot = 0xc366a826d730e5e8767a3b13f81cbf8ffa10a269272232b9243337637aef6dc7;
        // demo.requestProof{value: 0.01 ether}(functionId, blockRoot);
        // vm.stopBroadcast();
    }
}
