# Xelarius CLI & Contract Usage

## 🧰 Prerequisites

* Rust toolchain installed
* [`soroban-cli`](https://github.com/stellar/soroban-tools) installed:

  ```sh
  cargo install --locked soroban-cli
  ```
* Environment variables set:

  * `XZN_ISSUER_SECRET`
  * `XZN_ISSUER_PUB`
  * `XZN_TOKEN_CODE`

---

## 🔧 Build, Deploy, and Initialize

```sh
bash scripts/deploy_and_init.sh
```

---

## 🔀 CLI Usage

```sh
cargo run -p xelarius-cli -- <COMMAND> [ARGS]
```

---

## 🔍 Examples

### ✅ Check balance

```sh
cargo run -p xelarius-cli -- balance --addr <address>
```

### 🔁 Transfer tokens

```sh
cargo run -p xelarius-cli -- transfer --from <from_secret> --to <to_address> --amount 1000
```

### 🪹 Issue tokens

```sh
cargo run -p xelarius-cli -- issue --to <to_address> --amount 10000
```

### 🔥 Burn tokens

```sh
cargo run -p xelarius-cli -- burn --from <from_secret> --amount 500
```

### 📈 Total supply

```sh
cargo run -p xelarius-cli -- total-supply
```

---

## 🧪 Testing

Run contract tests:

```sh
cargo test -p xelarius-core
```

---

## 🚀 Advanced Features (Planned)

* Node rewards & staking (`rewards.rs`)
* Automated airdrops & treasury management
* Web-based interface (future)

