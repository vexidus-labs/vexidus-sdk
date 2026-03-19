# Vexidus IntentVM

Express what you want. IntentVM handles the rest.

**U.S. Patent App. 19/571,463** — Atomic intent-based transaction execution.

## What is IntentVM?

Instead of constructing individual blockchain operations, describe your **goal** in plain language:

```
"swap 100 USDC for VXS"
"get me 10 SOL, max 200 USDC"
"rebalance to 50% VXS, 30% USDC, 20% ETH"
"bridge 10 SOL from solana and swap to VXS"
"sell everything for USDC except VXS and ETH"
"swap 10 USDC and 10 USDT for VXS"
```

IntentVM parses the intent, plans the optimal execution route, and executes all operations atomically — if any step fails, everything reverts. One intent. One signature. N operations.

## Supported Goals

| Goal | Example | What Happens |
|------|---------|--------------|
| Swap | "swap 50 USDC for VXS" | Direct pool or auto-routed multi-hop |
| Transfer | "send 100 VXS to Vx0..." | Point-to-point token transfer |
| Stake | "stake 1000 VXS" | Validator staking (auto-selects if unspecified) |
| Provide Liquidity | "add 100 VXS and 500 USDC liquidity" | AMM pool provision or auto-creation |
| Bridge | "bridge 10 SOL from solana" | Cross-chain deposit with proof verification |
| Bridge + Action | "bridge 10 SOL from solana and swap to VXS" | Atomic cross-chain + swap |
| Acquire | "get me 10 SOL, max 200 USDC" | Reverse AMM — specify the outcome, not the input |
| Liquidate | "sell everything for USDC except VXS" | Portfolio scan + multi-token conversion |
| Rebalance | "rebalance to 50% VXS, 30% USDC, 20% ETH" | Automated portfolio rebalancing |
| Allocate | "split 100 VXS 60% stake 40% liquidity" | Split funds across operation types |
| Conditional | "swap 100 USDC for VXS if VXS below 0.15" | Price-triggered limit orders |
| Multi-Input | "swap 10 USDC and 10 USDT for VXS" | Multiple tokens merged atomically |

## Multi-Language Support

IntentVM parses intents in 8+ languages:

```
"comprar 10 VXS con USDC"       (Spanish)
"trocar 50 USDC para VXS"       (Portuguese)
"acheter 100 VXS avec USDC"     (French)
"kaufen 100 VXS mit USDC"       (German)
"comprare 50 VXS con USDC"      (Italian)
```

## Constraints

Control execution with inline constraints:

```
"swap 100 USDC for VXS with 2% slippage"
"swap 100 USDC for VXS within 5 minutes"
"swap 100 USDC for VXS max fee 50000"
```

## Quick Start

```rust
use vexidus_sdk::{parse_intent, IntentBuilder, WalletKeypair};

let wallet = WalletKeypair::generate();

// Parse natural language intent
let intent = parse_intent("swap 100 USDC for VXS with 1% slippage")?;

// Build and sign
let bundle = IntentBuilder::new()
    .with_intent(intent)
    .sign(&wallet)?
    .build();

// Submit
let client = WalletClient::new("https://testnet.vexidus.io");
let result = client.submit_bundle(&bundle).await?;
println!("Intent executed: {}", result.hash);
```

## How It Works

1. **Parse** — Natural language or structured JSON to `Goal` + `Constraints`
2. **Plan** — Automatic route discovery, protocol selection, reverse AMM calculation
3. **Bundle** — All operations wrapped in a single signed atomic bundle
4. **Execute** — All-or-nothing on-chain execution

## Intent Studio

Try intents interactively at [vexidex.com](https://vexidex.com) — type natural language commands and see the decomposed operations before signing.

## Licensing

IntentVM's core engine is available for licensing to wallets, DEXes, and protocols on any blockchain. The adapter pattern means the same patented core works across Solana, EVM, and Vexidus.

For licensing inquiries: **licensing@vexidus.io**
