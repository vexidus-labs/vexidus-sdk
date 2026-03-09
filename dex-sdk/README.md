# Vexidus DEX SDK

Interact with VexiDEX — the native on-chain AMM on Vexidus.

## Features

- **Pool Queries**: List pools, get reserves, calculate prices
- **Swap Quotes**: Preview swap output with price impact and fees
- **Liquidity Management**: Add/remove liquidity from AMM pools
- **Bundle Integration**: Compose swaps with other operations atomically

## Quick Start

```rust
use vexidus_sdk::{DexClient, BundleBuilder, WalletKeypair};

let dex = DexClient::new("https://testnet.vexidus.io");

// List available pools
let pools = dex.get_pools().await?;
for pool in &pools {
    println!("{}/{} — TVL: {}", pool.token_a_symbol, pool.token_b_symbol, pool.tvl);
}

// Get swap quote
let quote = dex.get_swap_quote(pool_id, token_in, 1_000_000_000).await?;
println!("You receive: {} {}", quote.amount_out, quote.token_out_symbol);
println!("Price impact: {}%", quote.price_impact);
println!("Fee: {} {}", quote.fee, quote.token_in_symbol);

// Execute swap
let wallet = WalletKeypair::from_secret_key(my_key)?;
let bundle = BundleBuilder::new()
    .swap(pool_id, token_in, amount_in, quote.min_amount_out)
    .sign(&wallet)?
    .build();

let tx = dex.submit_swap(&bundle).await?;
```

## Live Pools (Testnet)

| Pair | Initial Price |
|------|--------------|
| VXS/USDC | $0.10 |
| USDC/USDT | 1:1 |
| SOL/USDC | $86.97 |
| ETH/USDC | $1,957 |
| BNB/USDC | $629 |
| BTC/USDC | $67,612 |

## AMM Math

VexiDEX uses constant-product AMM (x * y = k) with a 0.3% swap fee.

```
output = (reserve_out * amount_in * 997) / (reserve_in * 1000 + amount_in * 997)
```

## Pool Operations

| Operation | Description |
|-----------|-------------|
| `CreatePool` | Create a new AMM pair (100 VXS creation fee) |
| `AddLiquidity` | Deposit both tokens proportionally |
| `RemoveLiquidity` | Withdraw your share of the pool |
| `Swap` | Trade one token for another |

All pool operations are native — no smart contract deployment needed.
