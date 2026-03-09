# Vexidus Wallet SDK

Generate keypairs, sign transactions, query balances, and construct bundles for the Vexidus blockchain.

## Features

- **Key Generation**: Ed25519 keypairs with native `Vx` and EVM `0x` address formats
- **Transaction Signing**: Sign bundles with Ed25519 signatures
- **Bundle Builder**: Fluent API for constructing multi-operation transactions
- **RPC Client**: Query balances, transaction history, network stats
- **Address Utilities**: Convert between Vx0 (native), 0x (EVM), and raw bytes

## Quick Start

```rust
use vexidus_sdk::{WalletKeypair, BundleBuilder, WalletClient};

// Generate a new wallet
let wallet = WalletKeypair::generate();
println!("Native:  {}", wallet.vx_address());   // Vx0QmR7...
println!("EVM:     {}", wallet.evm_address());   // 0x3cc75a...

// Connect to testnet
let client = WalletClient::new("https://testnet.vexidus.io");

// Check balance
let balance = client.get_balance(&wallet.address()).await?;
println!("Balance: {} VXS", balance.vxs);

// Transfer VXS
let bundle = BundleBuilder::new()
    .transfer(recipient_address, 1_000_000_000) // 1 VXS (9 decimals)
    .sign(&wallet)?
    .build();

let tx_hash = client.submit_bundle(&bundle).await?;
println!("TX: {}", tx_hash); // Vxh...
```

## Operations

The bundle builder supports all native Vexidus operations:

| Operation | Method |
|-----------|--------|
| Transfer VXS | `.transfer(to, amount)` |
| Transfer Token | `.transfer_token(token, to, amount)` |
| Create Token | `.create_token(name, symbol, supply, decimals)` |
| Mint Token | `.mint_token(token, to, amount)` |
| Burn Token | `.burn_token(token, amount)` |
| Mint NFT | `.mint_nft(collection, metadata)` |
| Swap | `.swap(pool, token_in, amount_in, min_out)` |
| Delegate Stake | `.delegate(validator, amount)` |
| Undelegate | `.undelegate(validator, amount)` |
| Vote | `.vote(proposal_id, vote)` |

## Address Formats

Vexidus uses dual address formats:

| Format | Prefix | Use |
|--------|--------|-----|
| Native | `Vx0` | Wallets, explorers, native RPC |
| EVM | `0x` | MetaMask, eth_* RPC methods |

Both formats map to the same account on-chain.

## RPC Endpoints

The `WalletClient` wraps the Vexidus JSON-RPC API:

```rust
let client = WalletClient::new("https://testnet.vexidus.io");

// Account
client.get_balance(&addr).await?;
client.get_nonce(&addr).await?;
client.get_tokens(&addr).await?;

// Transactions
client.submit_bundle(&bundle).await?;
client.get_transaction(&hash).await?;

// Network
client.get_network_stats().await?;
client.get_block_height().await?;
```
