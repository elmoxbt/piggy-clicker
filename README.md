# Solana Counter Game

A decentralized clicker game built with **Anchor** on **Solana**, where players contribute to a shared, verifiable counter stored on-chain. The counter has a configurable maximum value, supports increment/decrement, and can be reset (closing the account to reclaim rent). All operations are secured via PDAs and authority checks.

---

## Overview

This program turns your original Rust CLI counter into a **persistent, tamper-proof, on-chain game**. Every increment is a transaction — globally verifiable, low-cost, and high-throughput thanks to Solana.

### Key Features
- **Configurable Max Count**: Set a limit (e.g., 20) at initialization.
- **Safe Arithmetic**: Prevents overflow/underflow.
- **Authority Control**: Only the creator can reset.
- **Events & Logs**: Real-time feedback on milestones.
- **Rent Reclamation**: `reset` closes the account and returns SOL.

---

## Use Case

> **"Piggy Clicker"** — A fun, decentralized game where players worldwide tap to fill a shared piggy bank. When it hits the max, it "breaks" (resets), and the next player starts a new one.

Perfect for:
- Learning Solana + Anchor
- Demonstrating PDAs
- Building social/gamified dApps

---

## User Stories

| As a...       | I want to...                                   | So that...                                   |
|---------------|------------------------------------------------|----------------------------------------------|
| Player        | Initialize a counter with a max limit (e.g., 20) | I can start a new game session               |
| Player        | Increment the counter by 1                     | I contribute to the shared total             |
| Player        | Decrement the counter                          | I can correct mistakes or reduce the count   |
| Game Owner    | Reset and close the counter                    | I end the round and reclaim SOL rent         |
| Anyone        | View the current count on-chain                | The state is transparent and verifiable      |

---

## Architectural Diagram

```text
+-------------------+          +-------------------+          +-------------------+
|     Client        |          |     Solana RPC    |          |  Solana Program   |
| (Web/JS App)      |          | (Validator Node)  |          |   (Anchor-based)  |
+-------------------+          +-------------------+          +-------------------+
|                           |                              |
| 1. Build Tx (init/inc)    |                              |
| 2. Sign with Wallet       |                              |
|                           |                              |
| ───────────────────────►  | 3. Route Instruction         |
|                           | ──────────────────────────►  | • Validate PDA Seeds
|                           |                              | • Check Authority
|                           |                              | • Update Count
|                           |                              | • Enforce Max/Zero
| ◄──────────────────────   | ◄──────────────────────────  |
| 4. Confirmation + Events  |    (Logs + State Update)     |
|                           |                              |
+-------------------+          +-------------------+          +-------------------+
                                      ▲
                                      │
                                      ▼
                              +-------------------+
                              |   Counter PDA     |
                              | count, max, auth  |
                              +-------------------+
```
> *PDA = Program-Derived Address (owned by program, not wallet)*

---

## How It Works

| Instruction   | Action                                      | Constraints |
|---------------|---------------------------------------------|-----------|
| `initialize(max_count)` | Creates PDA, sets count=0, max=20         | `max_count > 0`, PDA seeds |
| `increment()`           | `count += 1`                                | `count < max_count` |
| `decrement()`           | `count -= 1`                                | `count > 0` |
| `reset()`               | Closes PDA, returns rent to authority       | Only original authority |

---

## Prerequisites

```bash
# Required
rustc --version          # ≥1.68
cargo --version
solana --version         # ≥1.18
anchor --version         # ≥0.30
yarn --version
```

---

## Getting Started

### 1. Clone & Install
```bash
git clone https://github.com/yourname/counter-game.git
cd counter-game
yarn install
```

### 2. Build
```bash
anchor build
```

### 3. Run Tests
```bash
anchor test
```

> Starts local validator automatically.

### 4. Deploy to Devnet
```toml
# Anchor.toml
[programs.devnet]
counter = "YOUR_PROGRAM_ID"
```

```bash
anchor deploy --provider.cluster devnet
```

---

## Client Usage (JavaScript)

```ts
const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("counter"), user.publicKey.toBuffer()],
  program.programId
);

await program.methods
  .initialize(new anchor.BN(20))
  .accounts({ counter: counterPDA, authority: user.publicKey })
  .signers([user])
  .rpc();
```

---

## Troubleshooting

| Issue                      | Fix |
|---------------------------|-----|
| `Unauthorized` error      | Use the same wallet that initialized |
| `CountExceeded`            | Reset or start new counter |
| `Account does not exist`   | Initialize first |
| Test fails on `reset`      | Ensure `close = authority` in Rust |

---

## Resources

- [Anchor Docs](https://www.anchor-lang.com)
- [Solana Cookbook](https://solanacookbook.com)
- [QuickNode Guide](https://www.quicknode.com/guides/solana-development/anchor)

---

**Built with ❤️ on Solana**  
```


