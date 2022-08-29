// SPDX-License-Identifier: MIT
pragma solidity ^0.8.6;

import "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

contract BountyscapeToken is ERC20 {
    constructor() ERC20("Bountyscape Token", "BST") {
        _mint(msg.sender, 20000000000000000000000000);
    }
}