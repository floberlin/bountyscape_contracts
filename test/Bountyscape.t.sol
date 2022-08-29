// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {Bountyscape} from "../src/Bountyscape.sol";
import {BountyscapeTreasury} from "../src/BountyscapeTreasury.sol";
import {BountyscapeToken} from "../src/BountyscapeToken.sol";

contract BountyscapeTest is Test {
    Bountyscape bountyscape;
    BountyscapeTreasury bountyscapeTreasury;
    BountyscapeToken bountyscapeToken;
    address internal employer;
    address internal deployer;
    address internal contractor;
    address internal contractor2;
    address internal contractor3;

    function setUp() public {
       
        bountyscapeTreasury = new BountyscapeTreasury();
        deployer = vm.addr(0xDe);
        vm.deal(deployer, 100 ether);
        vm.label(deployer, "deployer");
        vm.prank(deployer);
        bountyscapeToken = new BountyscapeToken();

        bountyscape = new Bountyscape(address(bountyscapeTreasury), address(bountyscapeToken));

        // set up employer
        employer = vm.addr(0xA11CE);
        vm.deal(employer, 100 ether);
        vm.label(employer, "employer");
        vm.prank(employer);
        bountyscape.grantRoleEmployer();

        // set up contractor
        contractor = vm.addr(0xB0B);
        vm.deal(contractor, 100 ether);
        vm.label(contractor, "contractor");
        vm.prank(contractor);
        bountyscape.grantRoleContractor();

        // set up contractor 2
        contractor2 = vm.addr(0xB0B2);
        vm.deal(contractor2, 100 ether);
        vm.label(contractor2, "contractor2");
        vm.prank(contractor2);
        bountyscape.grantRoleContractor();

        // set up contractor 3
        contractor3 = vm.addr(0xB0B3);
        vm.deal(contractor3, 100 ether);
        vm.label(contractor3, "contractor3");
        vm.prank(contractor3);
        bountyscape.grantRoleContractor();

        // set up contracts
        vm.prank(deployer);
        bountyscapeToken.approve(deployer, 10000000000000000000000000);
        vm.prank(deployer);
        bountyscapeToken.transferFrom(deployer, address(bountyscape), 10000000000000000000000000);

        bountyscapeTreasury.setBountyscape(address(bountyscape));
    }

    // Empolyer positive testcases
    function testEmployerCanCreateBounty() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
    }

    function testEmployerCanCreatePrivateBounty() public {
        vm.prank(employer);
        bountyscape.createPrivateBounty{value: 1 ether}(
            "QmIPFSIDPrivate",
            contractor
        );
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
    }

    function testEmployerCanApproveCompletedBounty() public {
        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
    }

    // Empolyer negative testcases
    function testFailEmployerCanCreateBountyNOVALUE() public {
        vm.prank(employer);
        bountyscape.createBounty("QmIPFSID");
    }

    function testFailEmployerCanCreatePrivateBountyNOVALUE() public {
        vm.prank(employer);
        bountyscape.createPrivateBounty("QmIPFSIDPrivate", contractor);
    }

    function testFailEmployerCanClaimBounty() public {
        vm.prank(employer);
        bountyscape.claimBounty("QmIPFSID");
        address[] memory claimers = bountyscape.getClaimers("QmIPFSID");
        assertEq(claimers[0], contractor);
    }

    // Contractor positive testcases
    function testContractorCanClaimBounty() public {
        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");
    }

    function testContractorCompleteBounty() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }

    function testContractorCanClaimFunds() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);

        vm.prank(contractor);
        bountyscape.claimFunds("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }

    function testContractorCanClaimFundsMultiple() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSI2");

        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID3");

        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(contractor2);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(contractor3);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(contractor3);
        bountyscape.claimBounty("QmIPFSID2");

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);

        vm.prank(contractor);
        bountyscape.claimFunds("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }

    function testContractorCanClaimFundsEmpolyerCanBurnMultiple() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(contractor2);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(employer);
        bountyscape.deleteBounty("QmIPFSID");
    }

    // Contractor negative testcases

    function testFailContractorCanClaimFundsNOCLAIM() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);

        vm.prank(contractor);
        bountyscape.claimFunds("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTreasury), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }

    // positive testcases - no roles required

    function testAnyoneCanListBounties() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID2");
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID3");
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID4");
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID5");

        bountyscape.getBounties();
    }

    function testAnyoneCanListBountiesAndGetStatus() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID2");
        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID2", contractor);
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID3");
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID4");
        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID4", contractor);
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID5");

        bountyscape.getBounties();

        assertTrue(!bountyscape.getStatus("QmIPFSID"));
        assertTrue(bountyscape.getStatus("QmIPFSID2"));
        assertTrue(!bountyscape.getStatus("QmIPFSID3"));
        assertTrue(bountyscape.getStatus("QmIPFSID4"));
        assertTrue(!bountyscape.getStatus("QmIPFSID5"));
    }
}
