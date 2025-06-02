# Xelarius (\$XZN)

**Xelarius** is a next-generation Rust-native blockchain platform built for scalable execution, cryptographic certainty, and self-sovereign infrastructure. Designed as a modular base-layer chain, Xelarius enables high-speed decentralized apps, composable smart contracts, and autonomous financial systems.

> The chain is cosmic, the protocol is elegant. Xelarius is the ledger of luminous order — forged in code, shaped by intent.

<p align="center">
  <img src="assets/xelarius-logo.png" alt="Xelarius Logo" width="200"/>
</p>

---

[![Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/ghostkellz/xelarius/actions/workflows/ci.yml/badge.svg)](https://github.com/ghostkellz/xelarius/actions/workflows/ci.yml)

---

## 🌌 Project Vision

Xelarius aims to provide:

* 🌠 **A programmable sovereign chain** — Rust-first, no upstream dependency
* 🧱 **Modular execution layers** — built-in smart contract runtime
* 🧿 **Predictable cryptography** — secure hashing, deterministic block execution
* 📡 **Interoperable design** — future support for Ethereum + WASM bridges
* 👁️‍🗨️ **Clarity over hype** — a real system, not vapor

---

## 🚀 Core Features

* ✅ Rust-based chain engine (xelarius-core)
* ✅ CLI node binary (xelarius-node)
* 🔐 Modular hashing & block structure
* 📦 Smart contract runtime (WIP)
* 🧠 Developer-focused architecture
* 🔄 Optional EVM + cross-chain bridges

---

## 🧱 Project Layout

```
xelarius/
├── xelarius-core        # Chain logic and state management
├── xelarius-contracts   # Contract execution engine (WASM-ready)
├── xelarius-node        # CLI node binary for testnet/dev
├── assets/              # Branding, logos, architecture
├── README.md
├── LICENSE
├── DOCS.md
└── WHITEPAPER.md
```

---

## 🛠️ Build & Run

### Prerequisites

* Rust (latest stable)
* Git, Bash, UNIX system

### Build the Workspace

```bash
cargo build --workspace
```

### Run the Dev Node

```bash
cargo run -p xelarius-node
```

### Expected Output

```bash
Block #0: { index: 0, data: "Genesis", ... }
Block #1: { index: 1, data: "First XZN tx", ... }
```

---

## 📜 Smart Contracts (WIP)

Contracts will be:

* 🧬 Written in Rust and compiled to WASM
* 🔐 Executed deterministically with resource limits
* 📎 Interacting with native runtime and chain state
* 🚀 Upgradeable and chain-persistent

EVM compatibility may be integrated via a sandbox engine or external bridge module.

---

## 🛰️ Roadmap Ideas

* ☁️ Distributed storage and pinning (IPFS layer?)
* 🔐 Zero-knowledge primitives
* 📡 Decentralized identity for dev access
* 🧪 Snapshot testing + fuzz harness
* 🌍 Public RPC and explorer tooling

---

## 🧠 Community & Contact

Join us in building luminous software:

* GitHub: [ghostkellz/xelarius](https://github.com/ghostkellz/xelarius)
* Email: [ckelley@ghostkellz.sh](mailto:ckelley@ghostkellz.sh)
* Domain: [xelarius.org](https://xelarius.org) *(coming soon)*

---

## 📄 License

MIT © [CK Technology LLC](https://github.com/ghostkellz)

> *“As above, so below. As within, so without. As the network, so the token.”*

---

*Xelarius: Forged in Rust. Powered by vision.*
