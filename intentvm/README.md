# Vexidus IntentVM

Express what you want in natural language. IntentVM decomposes your intent into atomic on-chain operations.

**Patent-Pending** — Application filed.

## What is IntentVM?

Instead of constructing individual operations, describe your **goal**:

```
"transfer 100 VXS to Vx0QmR7..."
"swap 50 USDC for VXS"
"stake 1000 VXS with the top validator"
```

IntentVM parses the intent, decomposes it into operations, and executes them atomically — if any step fails, everything reverts.

## Quick Start

```rust
use vexidus_sdk::{parse_intent, IntentBuilder, WalletKeypair};

let wallet = WalletKeypair::generate();

// Parse natural language intent
let intent = parse_intent("transfer 50 USDC to Vx0abc...")?;

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

## Supported Goals

| Goal | Example | Operations Generated |
|------|---------|---------------------|
| Transfer | "send 100 VXS to Vx0..." | `Transfer` |
| Swap | "swap 50 USDC for VXS" | `Swap` |
| Multi-op | "transfer 50 USDC to Vx0... and swap 100 VXS for SOL" | `Transfer` + `Swap` |

## How It Works

1. **Parse**: Natural language → structured `ParsedIntent`
2. **Decompose**: Intent → sequence of native `Operation`s
3. **Bundle**: Operations wrapped in a single signed bundle
4. **Execute**: All-or-nothing atomic execution on-chain

One intent. One signature. N operations.

## Intent Studio

Try intents interactively at [vexidex.com](https://vexidex.com) — the Intent Studio terminal lets you type natural language commands and see the decomposed operations before signing.
