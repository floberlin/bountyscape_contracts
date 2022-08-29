// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {Bountyscape} from "../src/Bountyscape.sol";
import {BountyscapeTreasury} from "../src/BountyscapeTreasury.sol";
import {BountyscapeToken} from "../src/BountyscapeToken.sol";

contract BountyscapeScript is Script {

    Bountyscape bountyscape;
    BountyscapeTreasury bountyscapeTreasury;
    BountyscapeToken bountyscapeToken;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        bountyscapeToken = new BountyscapeToken();
        bountyscapeTreasury = new BountyscapeTreasury();
        bountyscape = new Bountyscape(address(bountyscapeTreasury), address(bountyscapeToken));
        bountyscapeTreasury.setBountyscape(address(bountyscape));
        bountyscapeToken.approve(msg.sender, 10000000000000000000000000);
        bountyscapeToken.transferFrom(msg.sender, address(bountyscape), 10000000000000000000000000);

        vm.stopBroadcast();
    }
}
