pragma solidity 0.8.30;

struct Test {
    bool flag;
    uint32 ID;
}

library LibraryTest {
    function sayHello() public pure returns (string memory) {
        return print_Test();
    }
}