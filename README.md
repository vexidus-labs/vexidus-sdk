# Vexidus SDK

Official SDKs for building on the Vexidus blockchain.

Vexidus is a next-generation Layer 1 blockchain with native token standards, an on-chain DEX, post-quantum security, and intent-based transactions — no smart contracts required.

## SDKs

| SDK | Description | Status |
|-----|-------------|--------|
| [Wallet SDK](./wallet-sdk/) | Key generation, transaction signing, balance queries, bundle construction | Live |
| [DEX SDK](./dex-sdk/) | AMM pool interaction, swap quotes, liquidity management | Live |
| [Validator SDK](./validator-sdk/) | Node setup, staking, delegation, validator management | Live |
| [IntentVM](./intentvm/) | Natural language intent parsing, atomic multi-operation transactions | Live |

## Quick Start

### Install (Rust)

Add to your `Cargo.toml`:

```toml
[dependencies]
vexidus-sdk = { git = "https://github.com/vexidus-labs/vexidus-sdk" }
```

### Create a Wallet

```rust
use vexidus_sdk::WalletKeypair;

let wallet = WalletKeypair::generate();
println!("Address: {}", wallet.vx_address());    // Vx0...
println!("EVM:     {}", wallet.evm_address());    // 0x...
```

### Build & Submit a Transaction

```rust
use vexidus_sdk::{BundleBuilder, WalletKeypair};

let wallet = WalletKeypair::generate();
let bundle = BundleBuilder::new()
    .transfer(recipient, amount)
    .sign(&wallet)?
    .build();
```

### Swap on VexiDEX

```rust
use vexidus_sdk::DexClient;

let dex = DexClient::new("https://testnet.vexidus.io");
let quote = dex.get_swap_quote(pool_id, token_in, amount_in).await?;
println!("Output: {} (price impact: {}%)", quote.amount_out, quote.price_impact);
```

### Submit an Intent

```rust
use vexidus_sdk::{parse_intent, IntentBuilder};

let intent = parse_intent("transfer 100 VXS to Vx0abc...")?;
let bundle = IntentBuilder::new()
    .with_intent(intent)
    .sign(&wallet)?
    .build();
```

## Network

| Resource | URL |
|----------|-----|
| Testnet RPC | `https://testnet.vexidus.io` |
| Block Explorer | [vexscan.io](https://vexscan.io) |
| Wallet | [wallet.vexspark.com](https://wallet.vexspark.com) |
| DEX | [vexidex.com](https://vexidex.com) |
| Faucet | [vexswap.xyz](https://vexswap.xyz) |
| Documentation | [docs.vexidus.io](https://docs.vexidus.io) |
| Developer Studio | [vexforge.xyz](https://vexforge.xyz) |

## Key Features

- **Native Addresses**: `Vx` prefix (base58), EVM-compatible `0x` for MetaMask
- **Native Hashes**: `Vxh` prefix (BLAKE3), `0x` hex for EVM tools
- **Gas**: ~$0.0002 per transfer (10 nanoVXS/gas)
- **Finality**: Adaptive block time (500ms–12s)
- **Token Standards**: VSC-7 (fungible), VSC-21 (NFT), VSC-55 (multi-token), VSC-8 (stablecoin)
- **Post-Quantum**: Ed25519 + Dilithium3 hybrid keypairs

## Architecture

Vexidus is **not** an EVM chain. It's a purpose-built L1 with native operations (no smart contract deployment needed):

- Token creation, transfer, minting, burning
- NFT minting, metadata, collections
- AMM pool creation, swaps, liquidity
- Staking, delegation, governance voting
- Cross-chain bridge deposits/withdrawals
- Intent-based atomic multi-operation bundles

All operations are submitted as **bundles** — signed containers of one or more operations executed atomically.

## License

Apache 2.0 — see [LICENSE](./LICENSE)

## Links

- [Vexidus Documentation](https://docs.vexidus.io)
- [GitHub Organization](https://github.com/vexidus-labs)
- [X (Twitter)](https://x.com/vexiduslabs)
