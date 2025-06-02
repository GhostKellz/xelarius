# Xelarius Whitepaper

## 1. Overview

**Xelarius (\$XZN)** is an open-source, Rust-native blockchain protocol designed for programmable sovereignty, smart contracts, and global peer-to-peer financial applications. Born from the idea that infrastructure should be accessible, composable, and transparent — Xelarius is not just a token, but a modular chain for the builders of the future.

## 2. Why a Custom Chain?

Stellar, Ethereum, and other L1s offered great beginnings — but they are constrained. Xelarius takes the lessons of those networks and reimagines a core layer written in Rust, without the baggage of upstream limitations.

This allows:

* ⚙️ Custom cryptography & storage formats
* 🧱 Native WASM contract execution
* 🌐 Future-proofed bridges to Ethereum & others

The chain is ours to evolve, not patch.

## 3. Vision

**Accessible Crypto. Native Sovereignty. Dev-first DNA.**

We believe in:

* 🌍 A borderless layer to transact, earn, and build
* 🧠 Clean, secure, and modern developer workflows
* 🧑‍🚀 Empowering sysadmins, homelabbers, and crypto engineers
* 🛰️ Enabling testnets, sidechains, and composable modules

## 4. Core Capabilities

| Feature               | Description                                                   |
| --------------------- | ------------------------------------------------------------- |
| **Rust Core**         | Built entirely in Rust for speed and security                 |
| **Smart Contracts**   | WASM-compatible execution (custom runtime, no EVM required)   |
| **Modular Consensus** | Flexible consensus integration (future Tendermint, PoA, etc.) |
| **Node Rewards**      | Planned incentives for running public nodes and relays        |
| **Bridges (Planned)** | Cross-chain support with Ethereum and L2s                     |

## 5. Use Cases

* 🛠️ Build decentralized applications without Solidity
* ⚡ Launch payment systems, governance tools, or games
* 🪙 Issue and control native \$XZN-backed digital assets
* 🧰 Integrate CLI utilities for dev workflows
* 🌐 Future-proof your software by building on Rust + WASM

## 6. Tokenomics (Finalized)

* **Token Name**: Xelarius
* **Ticker**: $XZN
* **Initial Platform**: Native chain
* **Supply**: 42,000,000 (fixed, with burning mechanism)
* **Usage**: Gas fees, staking, contract deployment, governance

## 7. Earning Mechanisms

Xelarius enables multiple ways to earn:

* **Staking**: Lock tokens to secure the network and earn rewards.
* **Compute Contributions**: Use CPU/GPU power for decentralized compute tasks.
* **Home Lab Participation**: Run nodes and relays to earn rewards.

## 8. Wallet Setup

To set up a test wallet:

1. Generate a wallet address:
   ```bash
   xelarius-wallet create --name ckelley.eth
   ```

2. Fund the wallet with test tokens:
   ```bash
   xelarius-wallet fund --address <wallet_address> --amount 1000
   ```

3. Use the wallet to send transactions:
   ```bash
   xelarius-wallet send --to <recipient_address> --amount 100 --gas 1
   ```

---

## 9. License

MIT License

## 10. Final Thoughts

Xelarius is not a fork. It’s not an EVM clone. It’s a chain made from scratch to honor flexibility, elegance, and engineering control. For those who value real infrastructure and self-determined paths, Xelarius is the beginning.

---

*“As above, so below. As within, so without. As the network, so the token.”*

---

*Xelarius: Forged in Rust. Powered by vision.*

