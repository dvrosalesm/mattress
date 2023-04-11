// SPDX-License-Identifier: MIT

pragma solidity ^0.8.9;

interface IMattress {

    enum Claims {
        Name,
        DOB,
        Email
    }

    struct Credential {
        string url;
        uint256 expiration;
        Claims[] claims;
        bytes metadata;
    }
    

    function grantClaim(string memory provider, Credential memory credential) external;
}
