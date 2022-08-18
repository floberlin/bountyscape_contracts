// SPDX-License-Identifier: MIT
pragma solidity >= 0.8.6;

import "openzeppelin-contracts/contracts/token/ERC1155/utils/ERC1155Holder.sol";
import "openzeppelin-contracts/contracts/access/Ownable.sol";

interface Bountyscape {
    function safeTransferFrom(
        address from,
        address to,
        uint256 id,
        uint256 amount,
        bytes memory data
    )
        external;
}

contract BountyscapeTreasury is ERC1155Holder, Ownable {
    // address public BountyscapeAddr;
    address public immutable BountyscapeAddr = 0x8cB3c3931365d372Ef6bD1297658d433d7A198e4;


    mapping(uint256 => address) public addressIdMapping;

    constructor() {
        _transferOwnership(msg.sender);
    }

    receive() external payable {}

    // function setBountyscape(address addr) public onlyOwner {
    //     BountyscapeAddr = addr;
    // }

    function withdraw(address addr, uint256 id) public payable {
        require(addr == addressIdMapping[id], "Caller is not whitelisted!");
        Bountyscape(BountyscapeAddr).safeTransferFrom(
            address(this), addr, id, 1, ""
        );
    }

    function whitelist(address addr, uint256 tokenId) public {
        require(
            msg.sender == BountyscapeAddr,
            "Caller is not Bountyscape smart contract!"
        );
        addressIdMapping[tokenId] = addr;
    }

    function withdrawReward(address contractor, uint256 id, uint256 amount)
        public
        payable
    {
        require(
            contractor == addressIdMapping[id], "Caller is not whitelisted!"
        );
        payable(contractor).transfer(amount);
    }
}
