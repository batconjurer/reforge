pragma solidity 0.8.30;
/** @notice A Dummy struct
* flag: A boolean
* id: A 32 byte identifier
* #[derive(get_id_or_revert(contract=DummyLibrary))]
*/
struct Dummy {
    bool flag;
    uint32 ID;
}

// #[derive(get_id_or_revert(contract=DummyLibrary))]
struct WrappedBytes {
    bytes inner;
}

library DummyLibrary {
    function sayHello() public pure returns (string memory) {
        string memory name = print_Dummy();
        return string.concat("Hello ", name);
    }
}