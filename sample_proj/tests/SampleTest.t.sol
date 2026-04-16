pragma solidity ^0.8.30;

import {Test} from "forge-std/Test.sol";
import {Dummy, WrappedBytes, DummyLibrary} from "../Sample.sol";

contract TestDummyLibrary is Test {
    function testSayHello() public pure {
        string memory output = DummyLibrary.sayHello();
        assertEq(output, "Hello Dummy");
    }

    function testGetIDOrRevert() public {
        Dummy memory obj1 = Dummy ({flag: true, ID: uint32(10) });
        assertEq(DummyLibrary.get_id_dummy(obj1), uint32(10));
        WrappedBytes memory obj2 = WrappedBytes({inner: "0xDEADBEEF"});
        vm.expectRevert("WrappedBytes has no field ID");
        DummyLibrary.get_id_wrappedbytes(obj2);
    }
}