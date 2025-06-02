# Xelarius Project Documentation

## Overview

**Xelarius** is a next-generation Rust-native blockchain protocol supporting modular smart contracts, custom validator logic, and WASM execution. This project includes the core chain logic, contract modules, and CLI utilities.

## Directory Structure

* `contracts/` — WASM-compatible smart contracts (e.g., token, staking)
* `xelarius-cli/` — CLI for interacting with the node and contracts
* `src/` — Chain runtime, primitives, modules, and execution engine
* `scripts/` — Deployment, devnet spin-up, and automation helpers
* `COMMANDS.md` — CLI and contract usage
* `README.md` — Project overview and goals
* `DOCS.md` — Developer reference and architecture

## Quickstart

1. Set required environment variables (for devnet or local testing)
2. Build and launch the node:

   ```sh
   cargo run --bin xelarius-node
   ```
3. Deploy core contracts and initialize:

   ```sh
   bash scripts/deploy_and_init.sh
   ```
4. Use the CLI:

   ```sh
   cargo run -p xelarius-cli -- <command> [args]
   ```

## Testing

* Run unit tests:

  ```sh
  cargo test --workspace
  ```

## Advanced Features (Planned)

* Node and contract rewards
* Airdrop and treasury automation
* Web frontend + block explorer
* Cross-chain bridge (XZN to ETH and others)

## Contributing

* Fork, PRs, and issues welcome
* Join the discussion on GitHub or email [ckelley@ghostkellz.sh](mailto:ckelley@ghostkellz.sh)

---

*Xelarius: Forged in Rust. Powered by vision.*
