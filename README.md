# Mattress
This is a hackathon project that tries to leverage the power of rust + wasm to interact with an EVM compatible blockchain.

The idea behind mattress is to be able to authenticate to any website using your own private key and managing sessions, claims and credentials all within a solidity smart contract.


## Code
This repo includes WasmCloud actors for deploying and managing a `Warehouse` which deploys your own `Mattress` to store decentralized credentials.

Actors and providers are meant to be deployed on (Cosmonic)[https://cosmonic.com]
