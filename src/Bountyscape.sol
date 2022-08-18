// SPDX-License-Identifier: MIT
pragma solidity >=0.8.6;

import "openzeppelin-contracts/contracts/token/ERC1155/ERC1155.sol";
import "openzeppelin-contracts/contracts/access/AccessControl.sol";
import "openzeppelin-contracts/contracts/security/ReentrancyGuard.sol";
import "openzeppelin-contracts/contracts/utils/Strings.sol";

interface Itreasury {
    function whitelist(address addr, uint256 tokenId) external;

    function withdraw(address addr, uint256 amount) external;

    function withdrawReward(
        address contractor,
        uint256 id,
        uint256 amount
    ) external;
}

contract Bountyscape is ERC1155, AccessControl, ReentrancyGuard {
    string private _uri; // URI of the token
    address public treasury = address(0); // treasury contract address
    uint256 public tokenID = 0; // token ID of the token to be minted
    bytes32 public constant EMPLOYER_ROLE = keccak256("EMPLOYER_ROLE"); // role of the employer
    bytes32 public constant CONTRACTOR_ROLE = keccak256("CONTRACTOR_ROLE"); // contractor is the role of the employee who is working for the employer.

    mapping(uint256 => string) public tokenIDtoIPFS; // tokenID to IPFS hash
    mapping(uint256 => uint256) public tokenIDtoReward; // tokenID to reward
    mapping(uint256 => address[]) public tokenIDtoClaimers; // array of addresses
    mapping(uint256 => bool) public tokenIDtoDone; // if the contractor has finished the work
    mapping(uint256 => bool) public claimedReward; // if the contractor has claimed the reward
    mapping(uint256 => bool) public completedBounty; //  if the contractor has completed the bounty
    mapping(string => bool) public claimedBounty; // if the contractor has claimed the bounty
    mapping(uint256 => bool) public approvedBounty; // if the employer has approved the bounty

    event BountyCreated(string _ipfsID, uint256 _tokenID, uint256 _reward); // event for when a Bounty is created
    event BountyClaimed(string _ipfsID, uint256 _tokenID, uint256 _reward); // event for when a Bounty is claimed
    event BountyCompleted(string _ipfsID, uint256 _tokenID, uint256 _reward); // event for when a Bounty is completed
    event BountyCancelled(string _ipfsID, uint256 _tokenID, uint256 _reward); // event for when a Bounty is cancelled
    event PermanentURI(string _value, uint256 indexed _id); // event for when a permanent URI is set

    constructor()
        ERC1155(
            "https://ipfs.io/ipns/k51qzi5uqu5diitbr0kyw2jut5z6d06hm923t1mqaygudw4zf2u1ocbip88ieq/{id}.json"
        )
    {
        _setupRole(DEFAULT_ADMIN_ROLE, msg.sender);
    }

    function supportsInterface(bytes4 interfaceId)
        public
        view
        virtual
        override(ERC1155, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }

    /**
     *
     */

    // start of role definitions
    function grantRoleEmployer() public {
        require(
            !hasRole(CONTRACTOR_ROLE, msg.sender),
            "Caller is already a contractor"
        );
        _setupRole(EMPLOYER_ROLE, msg.sender);
    }

    function revokeRoleEmployer(address user) public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "Caller is not an admin"
        );
        revokeRole(EMPLOYER_ROLE, user);
    }

    function grantRoleContractor() public {
        require(
            !hasRole(EMPLOYER_ROLE, msg.sender),
            "Caller is already an employer"
        );
        _setupRole(CONTRACTOR_ROLE, msg.sender);
    }

    function revokeRoleContractor(address user) public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "Caller is not an admin"
        );
        revokeRole(CONTRACTOR_ROLE, user);
    }

    // end of role definitions

    /**
     *
     */

    // start of access control definitions
    function setContract(address addr) public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "Caller is not the admin"
        );
        treasury = addr;
    }

    // end of access control definitions

    /**
     *
     */

    // main functions

    function createBounty(string memory ipfsID) public payable {
        require(
            hasRole(EMPLOYER_ROLE, msg.sender),
            "Caller is not an employer"
        );
        require(msg.value > 0, "Please define a reward for the Bounty");
        uint256 reward = msg.value;
        _mint(treasury, tokenID, 1, "");
        payable(treasury).transfer(reward);
        tokenIDtoIPFS[tokenID] = ipfsID;
        tokenIDtoReward[tokenID] = reward;

        emit BountyCreated(ipfsID, tokenID, reward);
        emit PermanentURI(tokenURI(tokenID), tokenID);
        tokenID++;
    }

    function createPrivateBounty(string memory ipfsID, address contractor)
        public
        payable
    {
        require(
            hasRole(EMPLOYER_ROLE, msg.sender),
            "Caller is not an employer"
        );
        require(msg.value > 0, "Please define a reward for the Bounty");
        uint256 reward = msg.value;
        _mint(treasury, tokenID, 1, "");
        payable(treasury).transfer(reward);
        tokenIDtoIPFS[tokenID] = ipfsID;
        emit BountyCreated(ipfsID, tokenID, reward);
        emit PermanentURI(tokenURI(tokenID), tokenID);
        tokenID++;
        Itreasury(treasury).whitelist(contractor, tokenID);
    }

    function claimBounty(string memory ipfsID) public {
        require(
            hasRole(CONTRACTOR_ROLE, msg.sender),
            "Caller is not a contractor"
        );
        uint256 _tokenID = this.getTokenID(ipfsID);
        require(
            !claimedBounty[
                append(
                    ipfsID,
                    addressToString(msg.sender),
                    ""
                )
            ],
            "Bounty has already been claimed"
        );
        claimedBounty[
                append(
                    ipfsID,
                    addressToString(msg.sender),
                    ""
                )
            ] = true;
        tokenIDtoClaimers[_tokenID].push(msg.sender);
        emit BountyClaimed(
            tokenIDtoIPFS[_tokenID],
            _tokenID,
            tokenIDtoReward[_tokenID]
        );
    }

    function approveCompletedBounty(string memory ipfsID, address contractor)
        public
    {
        require(hasRole(EMPLOYER_ROLE, msg.sender), "Caller is not a employer");
        uint256 _tokenID = this.getTokenID(ipfsID);
        require(!approvedBounty[_tokenID], "Bounty has already been approved");
        approvedBounty[_tokenID] = true;
        tokenIDtoDone[_tokenID] = true;
        Itreasury(treasury).whitelist(contractor, _tokenID);
    }

    function completeBounty(string memory ipfsID) public {
        require(
            hasRole(CONTRACTOR_ROLE, msg.sender),
            "Caller is not a contractor"
        );
        uint256 _tokenID = this.getTokenID(ipfsID);
        require(
            !completedBounty[_tokenID],
            "Bounty has already been completed"
        );
        completedBounty[_tokenID] = true;
        address[] memory claimers = tokenIDtoClaimers[_tokenID];
        bool result = false;
        if (claimers.length >= 0) {
            for (uint256 j = 0; j < claimers.length; j++) {
                if (compareAddr(claimers[j], msg.sender)) {
                    result = true;
                }
            }
        }
        require(result, "Caller is not a bounty claimer");
        require(
            tokenIDtoDone[_tokenID],
            "Bounty completion is not approved by the employer"
        );
        Itreasury(treasury).withdraw(msg.sender, _tokenID);
        emit BountyCompleted(
            tokenIDtoIPFS[_tokenID],
            _tokenID,
            tokenIDtoReward[_tokenID]
        );
    }

    function claimFunds(string memory ipfsID) public {
        uint256 _tokenID = this.getTokenID(ipfsID);
        require(
            !claimedReward[_tokenID],
            "This bounty rewards have already been claimed"
        );
        claimedReward[_tokenID] = true;
        require(
            hasRole(CONTRACTOR_ROLE, msg.sender),
            "Caller is not a contractor"
        );
        require(
            tokenIDtoDone[_tokenID],
            "Bounty completion is not approved by the employer"
        );
        require(
            balanceOf(msg.sender, _tokenID) >= 1,
            "You have not completed the Bounty yet"
        );
        uint256 reward = tokenIDtoReward[_tokenID];
        uint256 fee = 2; // 2% fee
        uint256 feeAmount = ((reward * fee) / 100); // 2% fee
        uint256 claimable = (reward - feeAmount); // reward - fee
        Itreasury(treasury).withdrawReward(msg.sender, _tokenID, claimable);
    }

    function deleteBounty(string memory ipfsID) public {
        require(hasRole(EMPLOYER_ROLE, msg.sender), "Caller is not a employer");
        uint256 _tokenID = this.getTokenID(ipfsID);
        require(balanceOf(treasury, _tokenID) >= 1, "Bounty not available");
        _burn(treasury, _tokenID, 1);
    }

    // Get functions
    function getTokenID(string memory ipfsID) public view returns (uint256) {
        uint256 result;
        for (uint256 j = 0; j <= tokenID; j++) {
            if (compareStrings(tokenIDtoIPFS[j], ipfsID)) {
                result = j;
            }
        }
        return result;
    }

    function getClaimers(string memory ipfsID)
        public
        view
        returns (address[] memory)
    {
        uint256 _tokenID = this.getTokenID(ipfsID);
        return tokenIDtoClaimers[_tokenID];
    }

    function getBounties() public view returns (string[] memory) {
        string[] memory result = new string[](tokenID);
        for (uint256 j = 0; j <= tokenID; j++) {
            if (this.balanceOf(treasury, j) > 0) {
                result[j] = tokenIDtoIPFS[j];
            }
        }
        return result;
    }

    function getStatus(string memory ipfsID) public view returns (bool) {
        uint256 _tokenID = this.getTokenID(ipfsID);
        return tokenIDtoDone[_tokenID];
    }

    // Helper functions
    function compareStrings(string memory a, string memory b)
        internal
        pure
        returns (bool)
    {
        return (keccak256(abi.encodePacked((a))) ==
            keccak256(abi.encodePacked((b))));
    }

    function compareAddr(address a, address b) internal pure returns (bool) {
        return (keccak256(abi.encodePacked((a))) ==
            keccak256(abi.encodePacked((b))));
    }

    function stringToBytes(string memory source)
        internal
        pure
        returns (bytes memory result)
    {
        return abi.encodePacked(source);
    }

    function addressToString(address account)
        public
        view
        returns (string memory)
    {
        uint256 i = uint256(uint160(address(account)));
        return Strings.toString(i);
    }

    function append(
        string memory a,
        string memory b,
        string memory c
    ) internal pure returns (string memory) {
        return string(abi.encodePacked(a, b, c));
    }

    function tokenURI(uint256 tokenId) public pure returns (string memory) {
        return
            append(
                "https://ipfs.io/ipns/k51qzi5uqu5diitbr0kyw2jut5z6d06hm923t1mqaygudw4zf2u1ocbip88ieq",
                Strings.toString(tokenId),
                ".json"
            );
    }
}
