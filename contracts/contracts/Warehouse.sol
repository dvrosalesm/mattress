// SPDX-License-Identifier: MIT

pragma solidity ^0.8.9;

import "./Mattress.sol";

contract Warehouse {
    
    function salt() private view returns (bytes32) {
        return bytes32(abi.encode(address(this)));
    }

    function ship() external returns (address) {
        Mattress mattress = new Mattress{salt: salt()}(msg.sender);

        return address(mattress);
    }

}
