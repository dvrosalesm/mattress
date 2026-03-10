# Mattress

A hackathon project that leverages Rust + WebAssembly to interact with EVM-compatible blockchains. Mattress enables decentralized authentication and credential management through smart contracts — think of it as "storing your credentials under the mattress," but on-chain.

## Concept

Authenticate to any website using your own private key, with sessions, claims, and credentials managed entirely within Solidity smart contracts. The Rust/WASM layer provides a performant, secure interface to the blockchain.

## Built With

- **Smart Contracts:** Solidity (Hardhat)
- **Backend Logic:** Rust + WebAssembly
- **Deployment:** WasmCloud actors on [Cosmonic](https://cosmonic.com)
- **Tooling:** Make, Cargo, npm

## Architecture

```
mattress/
├── contracts/          # Solidity smart contracts
│   ├── Mattress.sol    # Individual credential store
│   ├── Warehouse.sol   # Factory that deploys Mattress instances
│   └── IMattress.sol   # Interface definition
├── interfaces/         # Rust crate for WASM actors
│   └── src/lib.rs      # WasmCloud actor logic
```

### How It Works

1. **Warehouse** — A factory contract that deploys personal `Mattress` vaults
2. **Mattress** — Your personal on-chain credential store
3. **WASM Actors** — Rust-compiled WebAssembly modules deployed on WasmCloud that handle authentication flows and interact with the smart contracts

## Getting Started

### Smart Contracts

```bash
cd contracts
npm install
npx hardhat compile
npx hardhat test
```

### Rust / WASM

```bash
cd interfaces
cargo build
cargo test
```

## Deployment

Actors and providers are designed to be deployed on [Cosmonic](https://cosmonic.com). See the WasmCloud documentation for deployment instructions.
