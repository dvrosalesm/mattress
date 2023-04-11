// SPDX-License-Identifier: MIT

pragma solidity ^0.8.9;

import "./IMattress.sol";

contract Mattress is IMattress {
    address private _owner;
    mapping(address => mapping(string => Credential)) private _credentials;
    string[] public providers;

    modifier onlyOwner() {
        require(_owner == msg.sender, "Unauthorized");
        _;
    }

    constructor(address owner) {
        _owner = owner;
    }
        
    function grantClaim(string memory provider, Credential memory credential) external onlyOwner override {
        _credentials[_owner][provider] = credential;         
    }

    function revokeClaim(string memory provider) external onlyOwner {
        delete _credentials[_owner][provider];
    }
}
