// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {console} from "forge-std/console.sol";
import {Bountyscape} from "../src/Bountyscape.sol";
import {BountyscapeTresuary} from "../src/BountyscapeTresuary.sol";

contract BountyscapeTest is Test {
    Bountyscape bountyscape;
    BountyscapeTresuary bountyscapeTresuary;
    address internal employer;
    address internal contractor;

    function setUp() public {
        bountyscape = new Bountyscape();
        bountyscapeTresuary = new BountyscapeTresuary();

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

        // set up contracts
        bountyscape.setContract(address(bountyscapeTresuary));
        bountyscapeTresuary.setBountyscape(address(bountyscape));
    }

    // Empolyer positive testcases
    function testEmployerCanCreateBounty() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
    }

    function testEmployerCanCreatePrivateBounty() public {
        vm.prank(employer);
        bountyscape.createPrivateBounty{value: 1 ether}(
            "QmIPFSIDPrivate", contractor
        );
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
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
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }

    function testContractorCanClaimFunds() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.claimBounty("QmIPFSID");

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);

        vm.prank(contractor);
        bountyscape.claimFunds("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }

    // Contractor negative testcases

    function testFailContractorCanClaimFundsNOCLAIM() public {
        vm.prank(employer);
        bountyscape.createBounty{value: 1 ether}("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(employer);
        bountyscape.approveCompletedBounty("QmIPFSID", contractor);
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 1);
        assertEq(bountyscape.balanceOf(contractor, 0), 0);

        vm.prank(contractor);
        bountyscape.completeBounty("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);

        vm.prank(contractor);
        bountyscape.claimFunds("QmIPFSID");
        assertEq(bountyscape.balanceOf(address(bountyscapeTresuary), 0), 0);
        assertEq(bountyscape.balanceOf(contractor, 0), 1);
    }
}
