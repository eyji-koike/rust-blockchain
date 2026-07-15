# TinyL1 — A Minimal Layer 1 Blockchain in Rust

A from-scratch L1 blockchain implementing:

- **Stack-based VM** with 16 opcodes and gas metering
- **Merkle tree** for transaction roots and state commitment
- **BFT consensus** (Tendermint-style: Propose → PreVote → PreCommit → Commit)
- **Account-based state** with nonce replay protection

## Architecture

┌─────────────────────────────────┐ │ CLI / RPC Layer │ ├─────────────────────────────────┤ │ BFT Consensus Engine │ │ Propose → PreVote → PreCommit │ ├─────────────────────────────────┤ │ Execution Engine (VM) │ │ Stack machine + gas metering │ ├─────────────────────────────────┤ │ State (Merkle-Patricia Trie) │ ├─────────────────────────────────┤ │ Crypto (SHA3, Ed25519) │ └─────────────────────────────────┘

## Quick Start

```bash
cargo run
# Watch blocks get produced and finalized
``` 